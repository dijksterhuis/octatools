//! Functions for CLI actions related to copying Octatrack data,
//! such as `Bank`s, `Pattern`s, `Part`s or `Project`s.

mod utils;
mod yaml;

use log::{debug, error, info, trace, warn};

use crate::actions::copy::utils::*;
use crate::actions::copy::yaml::YamlCopyBankConfig;
use crate::common::RBoxErr;
use std::path::PathBuf;

use serde_octatrack::{
    projects::{options::ProjectSampleSlotType, slots::ProjectSampleSlot},
    FromFileAtPathBuf, ToFileAtPathBuf,
};

/// ### Copy a bank from one project / bank to another project / bank.
///
/// Main function for the `octatools copy bank` command, making it possible to
/// (somewhat safely) move any Octatrack Bank to a new location.
///
/// During a transfers, this
/// 1. searches for 'active' sample slots in the source Project
/// 2. copies source slots over to available free sample slots in the destination Project
/// 3. mutates all references to the source sample slots in the source Bank
/// 4. copys the source sample files to the Project's Set Audio Pool
/// 5. writes over the destination Project and Bank with new data.
///
/// A couple of important quirks to highlight:
/// - All 'active' sample files from the source project are consolidated into the
/// destination Set audio pool (the Set which the destination Project belongs to).
/// - Sample slots are not de-duplicated or tested for uniqueness. If you have a
/// lot of duplicate sample slots across Banks you are transferring then you may need to
/// perform some clean up later.
/// - 'Inactive' sample files will not be moved or copied. Only sample slots that
/// match the following criteria will be copied:
///     - have been assigned to a sample slot within the source Project
///     - sample slot has a p-locked sample locks somewhere in the Patterns of the source Bank.
///     - sample slot has been used by an Audio Track Machine (Static/Flex) in one of the Parts
///     of the source Bank.
///     - sample slot is not a recorder buffer
///
///

pub fn copy_bank(source_bank_file_path: &PathBuf, dest_bank_file_path: &PathBuf) -> RBoxErr<()> {
    info!("Loading banks ...");

    let mut banks = TransferMetaBank::new(source_bank_file_path, dest_bank_file_path)
        .expect("Could not load banks.");

    info!("Loading bank projects ...");
    let mut projects = TransferMetaProject::new(source_bank_file_path, dest_bank_file_path)
        .expect("Could not load projects.");

    let _ = projects.dest.project.to_pathbuf(&projects.dest.path);

    info!("Backing up destination bank to /tmp/ ...");
    let _ = std::fs::copy(&dest_bank_file_path, PathBuf::from("/tmp/bank.bak"))
        .expect("Could not back up destination bank file.");

    info!("Backing up destination project to /tmp/ ...");
    let _ = std::fs::copy(&projects.dest.path, PathBuf::from("/tmp/project.bak"))
        .expect("Could not back up destination bank file.");

    info!("Finding free sample slots in destination project ...");
    let (mut free_static, mut free_flex) = find_free_sslots(&projects)
        .expect("Error while searching for free sample slots in destination project.");

    info!(
        "Destination project has free sample slots: {:#?} static; {:#?} flex.",
        free_static.len(),
        free_flex.len()
    );

    info!("Finding sample slots usage in source project ...");
    let src_static_sslot_count = projects
        .src
        .project
        .slots
        .iter()
        .filter(|x| x.sample_type == ProjectSampleSlotType::Static)
        .count();

    let src_flex_sslot_count = projects
        .src
        .project
        .slots
        .iter()
        .filter(|x| x.sample_type == ProjectSampleSlotType::Flex)
        .count();

    info!(
        "Source project has sample slots allocations: {:#?} static; {:#?} flex.",
        src_static_sslot_count, src_flex_sslot_count,
    );

    // not enough sample slots -- clean up slot allocations please!

    if src_static_sslot_count > free_static.len() || src_flex_sslot_count > free_flex.len() {
        panic!("Not enough static slots in destination project!");
    }

    info!("Finding 'active' sample slots (actually used in source bank) ...");
    // read the source bank, looking for sample slots in active use
    let active_slots = get_active_sslot_ids(&projects.src.project.slots, &banks.src)
        .expect("Error while finding active sample slots in source bank.");

    info!(
        "\"Active\" sample slots in source bank: {:#?}",
        active_slots,
    );

    let mut updated_sample_slots: Vec<ProjectSampleSlot> = vec![];

    // edit the bank data in place, updating:
    // - project's sample slot;
    // - sample plocks reference to project sample slot;
    // - audio track machine assignment reference to project sample slot.

    // todo: what about the case where we have an 'inactive' sample plock,
    // i.e. to an empty sample slot, but that sample slot is in-use within
    // the destinaton project
    info!("Updating 'active' sample slots in source bank ...");
    for active_slot in active_slots {
        let new_slot_id = match active_slot.sample_type {
            ProjectSampleSlotType::Static => {
                let dest_slot_id = free_static.pop().expect("No more destination slots.");

                update_sslot_references_static(
                    &mut projects.src.project,
                    &mut banks,
                    active_slot.slot_id,
                    dest_slot_id,
                )
                .expect("Could not update static sample slot references from source bank.");

                dest_slot_id
            }
            ProjectSampleSlotType::Flex => {
                let dest_slot_id = free_flex.pop().expect("No more destination slots.");

                update_sslot_references_flex(
                    &mut projects.src.project,
                    &mut banks,
                    active_slot.slot_id,
                    dest_slot_id,
                )
                .expect("Could not update flex slot references from source bank.");

                dest_slot_id
            }
            ProjectSampleSlotType::RecorderBuffer => {
                warn!("Usupported behaviour: Attempted to update a Recording Buffer sample slot reference.");
                255
            }
        };

        let src_project_slot = projects
            .src
            .project
            .slots
            .iter()
            .find(|x| x.slot_id == new_slot_id as u16);

        if !src_project_slot.is_none() {
            let mut s: ProjectSampleSlot = src_project_slot
                .expect("Empty sample slots in source project.")
                .clone();

            if s.sample_type != ProjectSampleSlotType::RecorderBuffer {
                let _ = copy_sslot_sample_files(&projects, &s);
                s.path = get_relative_audio_pool_path_audio_file(&s).expect(
                    "Could not get new file path for sample to transfer to destination project.",
                );
                debug!("Updating sample slot ...");
                updated_sample_slots.push(s);
            }
        }
    }

    info!("Inserting 'active' sample slots from source project to destination project ...");
    let mut dest_sample_slots: Vec<ProjectSampleSlot> = projects.dest.project.slots;
    dest_sample_slots.append(&mut updated_sample_slots);

    info!("Writing sample slots to destination project ...");
    projects.dest.project.slots = dest_sample_slots;
    let _ = projects
        .dest
        .project
        .to_pathbuf(&projects.dest.path)
        .expect("Could not write project to file");

    info!("Writing new bank within project ...");
    let _ = banks
        .dest
        .to_pathbuf(dest_bank_file_path)
        .expect("Could not write bank to file");

    info!("Bank copy complete.");
    Ok(())
}

/// ### Batched bank copying using a YAML config
///
/// Expanded functionality on top of the `octatools copy bank` command.
/// Perform multiple copies one after the other by defining how to copy banks in a YAML config file.
///
/// All the caveats and deails for the `copy_bank` function still apply
/// (this function calls it multiple times).

pub fn batch_copy_banks(yaml_config_path: &PathBuf) -> RBoxErr<()> {
    let conf = YamlCopyBankConfig::from_pathbuf(yaml_config_path)?;

    for x in conf.bank_copies {
        let _ = copy_bank(&x.src, &x.dest);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_octatrack::banks::Bank;

    #[test]
    fn test_copy_bank() {
        use copy_dir;

        let infile = PathBuf::from("./data/tests/copy/bank/BANK-COPY-SRC/bank01.work");
        let outfile = PathBuf::from("./data/tests/copy/bank/BANK-COPY-DUMMY/bank01.work");

        // copy test destination project to a new directory, so we have a fresh test each time
        let _ = copy_dir::copy_dir(
            PathBuf::from("./data/tests/copy/bank/BANK-COPY-DEST/"),
            outfile.parent().unwrap(),
        );

        let _source_bank = Bank::from_pathbuf(&infile).unwrap();
        let _ = copy_bank(&infile, &outfile);
        let _copied_bank = Bank::from_pathbuf(&outfile).unwrap();

        // remove the test destination project directory
        let _ = std::fs::remove_dir_all(outfile.parent().unwrap());

        assert!(true);
    }
}
