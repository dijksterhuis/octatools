//! Functions for CLI actions related to copying Octatrack data,
//! such as `Bank`s, `Pattern`s, `Part`s or `Project`s.

#[cfg(test)]
mod tests;
pub(crate) mod utils;
mod yaml;

use crate::{actions::banks::yaml::YamlCopyBankConfig, OctatoolErrors, RBoxErr};
use itertools::Itertools;
use octatools_lib::projects::options::ProjectSampleSlotType;
use octatools_lib::{
    banks::{Bank, BankRawBytes},
    get_bytes_slice,
    projects::Project,
    read_type_from_bin_file, write_type_to_bin_file, yaml_file_to_type,
};
use std::{path::Path, path::PathBuf};
use utils::{
    calculate_copy_bank_changes, create_backup_of_work_file, find_sample_slot_refs_in_bank,
    get_bank_fname_from_id, get_zero_indexed_slots_from_one_indexed, transfer_sample_files,
    BankCopyPathsMeta, BankMeta, BankSlotReferenceType, ProjectMeta,
};

/// Show bytes output as u8 values for a Sample Attributes file located at `path`
pub fn show_bank_bytes(path: &Path, start_idx: &Option<usize>, len: &Option<usize>) -> RBoxErr<()> {
    let raw_bank = read_type_from_bin_file::<BankRawBytes>(path).expect("Could not read bank file");

    let bytes = get_bytes_slice(raw_bank.data.to_vec(), start_idx, len);
    println!("{:#?}", bytes);
    Ok(())
}

/// ### Copy Banks
///
/// Copy a bank from one project location to another, also transferring sample files and updating
/// project sample slots. Can be used to copy banks within the same project (swap the banks).
///
/// A couple of important notes to highlight:
///
/// - Only 'Active' sample slots are copied from the source project to the destination project. If a
///     sample slot is not used within the target bank then it is not copied to the destination
///     project.
///
/// - Copied sample files from the source project will be copied to the destination project
///     directory, not the `AUDIO` pool.
///
/// - Destination sample slots are reused if the slot settings and sample file paths match. If you
///     have different samples in two projects that use the same filename, you will get breakage.
///
/// - Bank data is modified, remapping sample slots that are 'active' or 'inactive'.
///     - Active: An Audio Track's machine / P-Lock trig references a sample slot that has a sample
///       loaded
///     - Inactive: An Audio Track's machine / P-Lock trig references a sample slot that **does
///       not** have a sample loaded
pub fn copy_bank_by_paths(
    source_project_dirpath: &Path,
    destination_project_dirpath: &Path,
    source_bank_number: usize,
    destination_bank_number: usize,
) -> RBoxErr<()> {
    if !(1..=16).contains(&source_bank_number) || !(1..=16).contains(&destination_bank_number) {
        return Err(Box::new(OctatoolErrors::CliInvalidBankIndex));
    }

    let source_meta = BankCopyPathsMeta {
        project: ProjectMeta::frompath(source_project_dirpath),
        bank: BankMeta::frompath(source_project_dirpath, source_bank_number),
    };

    println!("===================================================================================");
    println!("Loading data files ...");

    let src_project = read_type_from_bin_file::<Project>(&source_meta.project.filepath)
        .expect("Failed to read source project file.");

    let destination_meta = BankCopyPathsMeta {
        project: ProjectMeta::frompath(destination_project_dirpath),
        bank: BankMeta::frompath(destination_project_dirpath, destination_bank_number),
    };

    // up-front check to make sure thee are no missing audio files, could be breakage if there are
    // missing files.
    let mising_source_file_slot_ids = src_project
        .slots
        .iter()
        .filter(|x| {
            let src_path_audio_abs = &source_meta.project.dirpath.join(&x.path);
            !src_path_audio_abs.exists()
        })
        .cloned()
        .map(|x| (x.sample_type, x.slot_id))
        .into_group_map();

    if !mising_source_file_slot_ids.is_empty() {
        eprintln!("Missing sample files detected in source project! Not continuing.");
        eprintln!(
            "Slot IDs with no audio file: {:?}",
            mising_source_file_slot_ids
        );
        return Err(Box::new(OctatoolErrors::PathDoesNotExist));
    }

    create_backup_of_work_file(&destination_meta.project.filepath)
        .expect("Failed to create destination project file backup.");
    create_backup_of_work_file(&destination_meta.bank.filepath)
        .expect("Failed to create destination bank file backup.");

    let dest_project = read_type_from_bin_file::<Project>(&destination_meta.project.filepath)
        .expect("Failed to read destination project file.");

    let bank = read_type_from_bin_file::<Bank>(&source_meta.bank.filepath)?;

    println!("===================================================================================");
    println!("Calculating changes ...");

    let (new_project, new_bank, sample_transfers) =
        calculate_copy_bank_changes(source_project_dirpath, &src_project, &bank, &dest_project)?;

    println!("===================================================================================");

    /*
    ================================================================================================

    This is where we begin making changes on the file system. If you want to
    include some warnings to the user about potentially destructive actions
    occurring -- __now is the time to do it__!!!

    ================================================================================================
    */

    if !sample_transfers.is_empty() {
        println!("Copying necessary sample files ...")
    } else {
        println!("No sample files need copying.")
    }

    transfer_sample_files(
        &sample_transfers,
        source_project_dirpath,
        destination_project_dirpath,
    )?;

    println!("Writing sample slot modifications to destination ...");
    write_type_to_bin_file::<Project>(&new_project, &destination_meta.project.filepath)
        .expect("Could not write modified project data to destination location.");

    println!("Writing bank modifications to destination ...");
    write_type_to_bin_file::<Bank>(&new_bank, &destination_meta.bank.filepath)
        .expect("Could not write modified bank to destination location.");

    println!("===================================================================================");
    println!("Bank copy complete.");
    Ok(())
}

/// ### Batched bank copying using a YAML config
///
/// Wrapper over the `copy_bank_by_paths` function / `octatools copy bank` command.
/// Allows users to perform multiple copies in one command run by defining how to copy banks in a
/// YAML config file.
///
/// All the caveats and details for the `copy_bank_by_paths` function still apply.
pub fn batch_copy_banks(yaml_config_path: &Path) -> RBoxErr<()> {
    let conf = yaml_file_to_type::<YamlCopyBankConfig>(yaml_config_path)
        .expect("Could not load YAML configuration for batch bank transfers");

    for x in conf.bank_copies {
        copy_bank_by_paths(
            &x.src.project,
            &x.dest.project,
            x.src.bank_id,
            x.dest.bank_id,
        )
        .expect("Could not copy bank");
    }

    Ok(())
}

#[derive(Debug)]
#[allow(dead_code)] // clippy doesn't detect the usage below for some reason
struct SlotUseListItem {
    sample_loaded: bool,
    sample_type: ProjectSampleSlotType,
    slot_id: u8,
    path: Option<PathBuf>,
}

/// List samples slots that are used in a bank
pub fn list_bank_sample_slot_references(
    project_dirpath: &Path,
    bank_id: usize,
    ignore_empty_slots: bool,
) -> RBoxErr<()> {
    let project_fpath = project_dirpath.to_path_buf().join("project.work");

    let bank_fpath = project_dirpath
        .to_path_buf()
        .join(get_bank_fname_from_id(bank_id));

    let src_project =
        read_type_from_bin_file::<Project>(&project_fpath).expect("Failed to read project file.");

    let bank = read_type_from_bin_file::<Bank>(&bank_fpath).expect("Failed to read bank file.");

    find_sample_slot_refs_in_bank(
        &get_zero_indexed_slots_from_one_indexed(&src_project.slots)?,
        &bank,
    )?
    .iter()
    .filter(|x| ignore_empty_slots != (x.reference_type == BankSlotReferenceType::Active))
    .sorted_by(|x, y| Ord::cmp(&x.slot_id, &y.slot_id))
    .map(|x| {
        let path = if x.reference_type == BankSlotReferenceType::Active {
            Some(
                &src_project
                    .slots
                    .iter()
                    .find(|s| s.slot_id == x.slot_id)
                    .unwrap()
                    .path,
            )
        } else {
            None
        };

        SlotUseListItem {
            sample_loaded: (x.reference_type == BankSlotReferenceType::Active),
            sample_type: x.sample_type.clone(),
            slot_id: x.slot_id + 1,
            path: path.cloned(),
        }
    })
    .for_each(|x| println!("{:?}", x));

    Ok(())
}
