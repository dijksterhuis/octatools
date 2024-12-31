//! Functions for CLI actions related to copying Octatrack data,
//! such as `Bank`s, `Pattern`s, `Part`s or `Project`s.

mod yaml;

use log::{debug, error, info, warn};
use serde_octatrack::projects::Project;

use crate::{
    actions::{banks::yaml::YamlCopyBankConfig, get_bytes_slice, load_from_yaml},
    RBoxErr,
};
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use serde_octatrack::{
    banks::{Bank, BankRawBytes},
    projects::{options::ProjectSampleSlotType, slots::ProjectSampleSlot},
    FromPath, ToPath, ToYamlFile,
};

/// Show deserialised representation of a Sample Attributes file at `path`
pub fn show_bank(path: &PathBuf) -> RBoxErr<()> {
    let b = Bank::from_path(path).expect("Could not load bank file");
    println!("{b:#?}");
    Ok(())
}

/// Show bytes output as u8 values for a Sample Attributes file located at `path`
pub fn show_bank_bytes(
    path: &PathBuf,
    start_idx: &Option<usize>,
    len: &Option<usize>,
) -> RBoxErr<()> {
    let bytes = get_bytes_slice(
        BankRawBytes::from_path(path)
            .expect("Could not load ot file")
            .data
            .to_vec(),
        start_idx,
        len,
    );
    println!("{:#?}", bytes);
    Ok(())
}

/// Load Bank file data from a YAML file
pub fn load_bank(yaml_path: &Path, outfile: &Path) -> RBoxErr<()> {
    load_from_yaml::<Bank>(yaml_path, outfile)
}

/// Dump Bank file data to a YAML file
pub fn dump_bank(bank_path: &Path, yaml_path: &Path) -> RBoxErr<()> {
    let b = Bank::from_path(bank_path).expect("Could not load bank file");
    let _ = b.to_yaml(yaml_path);
    Ok(())
}

/// Find free sample slot locations in a `Project`
fn find_free_sample_slot_ids(
    sample_slots_inuse: &Vec<ProjectSampleSlot>,
    slot_type: ProjectSampleSlotType,
) -> RBoxErr<Vec<u8>> {
    let mut free_slots: Vec<u8> = vec![];
    for i in 1..=128 {
        free_slots.push(i)
    }

    for slot in sample_slots_inuse.iter() {
        if slot_type == slot.sample_type {
            free_slots.retain(|x| *x != slot.slot_id as u8);
        }
    }

    // reverse so we can just use pop instead of needing to import VecDeque::pop_rev()
    free_slots.reverse();

    Ok(free_slots)
}

/// Find sample slots belonging to a `Project` which are used within a `Bank`
fn find_active_sample_slots(
    project_slots: &Vec<ProjectSampleSlot>,
    bank: &Bank,
    slot_type: &ProjectSampleSlotType,
) -> RBoxErr<Vec<ProjectSampleSlot>> {
    // avoid dealing with duplicated sample slots -> sets
    let mut active_slots: HashSet<ProjectSampleSlot> = HashSet::new();

    debug!(
        "Checking for sample slot usage in bank: type={:#?}",
        slot_type
    );
    debug!("Checking bank's pattern p-locks: type={:#?}", slot_type);
    // pattern P-locks
    for (pattern_idx, pattern) in bank.patterns.iter().enumerate() {
        for (track_idx, audio_track_trigs) in pattern.audio_track_trigs.iter().enumerate() {
            for (plock_idx, plock) in audio_track_trigs.plocks.iter().enumerate() {
                let active_slot = match slot_type {
                    ProjectSampleSlotType::Static => {
                        // static slot is assigned (255 is no assignment)
                        if plock.static_slot_id < 128 {
                            project_slots.iter().find(|x| {
                                x.slot_id == plock.static_slot_id && x.sample_type == *slot_type
                            })
                        } else {
                            None
                        }
                    }
                    ProjectSampleSlotType::Flex => {
                        // static slot is assigned (255 is no assignment)
                        if plock.flex_slot_id < 128 {
                            project_slots.iter().find(|x| {
                                x.slot_id == plock.flex_slot_id && x.sample_type == *slot_type
                            })
                        } else {
                            None
                        }
                    }
                    _ => None,
                };

                if active_slot.is_some() {
                    info!(
                        "Found active sample p-lock: type={:#?} pattern={:#?} track={:#?} trig={:#?} slot={:#?}",
                        slot_type,
                        pattern_idx,
                        track_idx,
                        plock_idx,
                        plock.static_slot_id,
                    );
                    active_slots.insert(active_slot.unwrap().clone());
                } else {
                    warn!(
                        "Found an 'inactive' sample p-lock: type={:#?} pattern={:#?} track={:#?} trig={:#?} slot={:#?}",
                        slot_type,
                        pattern_idx,
                        track_idx,
                        plock_idx,
                        plock.static_slot_id,
                    );
                    warn!("Pattern p-lock may eventually point at an existing sample in the destination project.")
                }
            }
        }
    }

    debug!("Checking bank's unsaved part state: type={:#?}", slot_type);
    // parts_unsaved
    for (part_idx, part) in bank.parts_unsaved.iter().enumerate() {
        for (track_idx, audio_track_slots) in part.audio_track_machine_slots.iter().enumerate() {
            // the default sample slot for Static/Flex machines is the track ID.
            // so we check if there is an actual sample assigned to a machine's slot
            // to work out if the machine actually has an 'active' sample slot assignment or not.

            let active_slot = match slot_type {
                ProjectSampleSlotType::Static => project_slots.iter().find(|x| {
                    x.slot_id == audio_track_slots.static_slot_id && x.sample_type == *slot_type
                }),
                ProjectSampleSlotType::Flex => project_slots.iter().find(|x| {
                    x.slot_id == audio_track_slots.flex_slot_id && x.sample_type == *slot_type
                }),
                _ => None,
            };

            if active_slot.is_some() {
                info!(
                    "Found active sample slot machine usage: type={:#?} unsavedPart={:#?} track={:#?} slot={:#?}",
                    slot_type,
                    part_idx,
                    track_idx,
                    audio_track_slots.static_slot_id,
                );
                active_slots.insert(active_slot.unwrap().clone());
            } else {
                warn!("Found an 'inactive' sample slot machine usage: type={:#?} unsavedPart={:#?} track={:#?} slot={:#?}", 
                    slot_type,
                    part_idx,
                    track_idx,
                    audio_track_slots.static_slot_id,
                );
                warn!("Machine sample slot assignment may point at an existing sample in the destination project.")
            }
        }
    }

    debug!("Checking bank's saved part state: type={:#?}", slot_type);
    // parts_saved
    for (part_idx, part) in bank.parts_saved.iter().enumerate() {
        for (track_idx, audio_track_slots) in part.audio_track_machine_slots.iter().enumerate() {
            // the default sample slot for Static/Flex machines is the track ID.
            // so we check if there is an actual sample assigned to a machine's slot
            // to work out if the machine actually has an 'active' sample slot assignment or not.

            let active_slot = match slot_type {
                ProjectSampleSlotType::Static => project_slots.iter().find(|x| {
                    x.slot_id == audio_track_slots.static_slot_id && x.sample_type == *slot_type
                }),
                ProjectSampleSlotType::Flex => project_slots.iter().find(|x| {
                    x.slot_id == audio_track_slots.flex_slot_id && x.sample_type == *slot_type
                }),
                _ => None,
            };

            if active_slot.is_some() {
                info!(
                    "Found active sample slot machine usage: type={:#?} savedPart={:#?} track={:#?} slot={:#?}",
                    slot_type,
                    part_idx,
                    track_idx,
                    audio_track_slots.static_slot_id,
                );
                active_slots.insert(active_slot.unwrap().clone());
            } else {
                warn!("Found an 'inactive' sample slot machine usage: type={:#?} savedPart={:#?} track={:#?} slot={:#?}", 
                    slot_type,
                    part_idx,
                    track_idx,
                    audio_track_slots.static_slot_id,
                );
                warn!("Machine sample slot assignment may point at an existing sample in the destination project.")
            }
        }
    }

    Ok(active_slots.into_iter().collect())
}

/// ### Copy a bank from one project to another project.
///
/// Main function for the `octatools copy bank` command, making it possible to
/// (somewhat safely) move any Octatrack Bank to a new location.
///
/// During a transfers, this
/// 1. searches for 'active' project sample slots used in the source bank
/// 2. copies source slots over to available free sample slots in the destination project
/// 3. mutates all references to the source sample slots in the source bank
/// 4. copys the source sample files to the project's audio pool
/// 5. writes over the destination project and bank with new data.
///
/// A couple of important quirks to highlight:
/// - All 'active' sample files from the source project are consolidated into the
/// destination Set audio pool (the Set which the destination Project belongs to).
/// - Sample slots are not de-duplicated or tested for uniqueness against existing
/// destination sample slots. If you have a lot of duplicate sample slots across
/// banks then you may need to do some clean up.
/// - 'Inactive' sample files will not be moved or copied. Only sample slots that
/// match the following criteria will be copied:
///     - have been assigned to a sample slot within the source Project
///     - sample slot has a p-locked sample locks somewhere in the Patterns of the source Bank.
///     - sample slot has been used by an Audio Track Machine (Static/Flex) in one of the Parts
///     of the source Bank.
///     - sample slot is not a recorder buffer

pub fn copy_bank(
    source_bank_filepath: &PathBuf,
    source_project_filepath: &PathBuf,
    destination_bank_filepath: &PathBuf,
    destination_project_filepath: &PathBuf,
) -> RBoxErr<()> {
    info!("Loading banks ...");

    let mut bank = Bank::from_path(source_bank_filepath).expect("Could not load source bank.");

    info!("Loading projects ...");

    let src_project =
        Project::from_path(source_project_filepath).expect("Could not load source project");

    let mut dest_project =
        Project::from_path(destination_project_filepath).expect("Could not load source project");

    // let mut projects = TransferProject::new(source_project_filepath, destination_project_filepath)
    //     .expect("Could not load projects.");

    // let _ = projects.dest.project.to_path(&projects.dest.path);

    info!("Backing up destination bank to /tmp/ ...");
    let _ = std::fs::copy(destination_bank_filepath, PathBuf::from("/tmp/bank.bak"))
        .expect("Could not back up destination bank file.");

    info!("Backing up destination project to /tmp/ ...");
    let _ = std::fs::copy(
        destination_project_filepath,
        PathBuf::from("/tmp/project.bak"),
    )
    .expect("Could not back up destination bank file.");

    info!("Finding free static sample slots in destination project ...");
    let mut free_static =
        find_free_sample_slot_ids(&dest_project.slots, ProjectSampleSlotType::Static)
            .expect("Error while searching for free static sample slots in destination project.");

    info!("Finding free flex sample slots in destination project ...");
    let mut free_flex = find_free_sample_slot_ids(&dest_project.slots, ProjectSampleSlotType::Flex)
        .expect("Error while searching for free flex sample slots in destination project.");

    info!(
        "Destination project has free sample slots: {:#?} static; {:#?} flex.",
        free_static.len(),
        free_flex.len()
    );

    info!("Finding 'active' sample slots (sample slots actually used within source bank) ...");
    let mut active_static_slots =
        find_active_sample_slots(&src_project.slots, &bank, &ProjectSampleSlotType::Static)
            .expect("Error while finding active static sample slots in source bank.");

    let mut active_flex_slots =
        find_active_sample_slots(&src_project.slots, &bank, &ProjectSampleSlotType::Flex)
            .expect("Error while finding active flex sample slots in source bank.");

    let mut active_slots: Vec<ProjectSampleSlot> = vec![];
    active_slots.append(&mut active_static_slots);
    active_slots.append(&mut active_flex_slots);

    if active_flex_slots.len() > free_flex.len() || active_static_slots.len() > free_static.len() {
        error!("Not enough free samples slots in destination project!");
        error!(
            "Static sample slots: sourceActive={:#?} destAvailable={:#?}",
            active_static_slots.len(),
            free_static
        );
        error!(
            "Flex sample slots: sourceActive={:#?} destAvailable={:#?}",
            active_flex_slots.len(),
            free_flex
        );
        panic!("Not enough samples slots in destination project!");
    }

    info!("Active sample slots in source bank: {:#?}", active_slots);

    // edit the bank data in place, updating:
    // - project's sample slot;
    // - sample plocks reference to project sample slot;
    // - audio track machine assignment reference to project sample slot.
    info!(
        "Updating {:#?} active sample slots in source bank ...",
        active_slots.len()
    );
    let mut updated_sample_slots: Vec<ProjectSampleSlot> = active_slots
        .iter()
        .enumerate()
        .map(
            |(slot_idx, active_slot)|
            {
                // pop a free sample slot ID from the array we created earlier.
                debug!(
                    "Beginning transfer of sample slot: n={:#?} total={:#?} type={:#?}",
                    slot_idx,
                    active_slots.len(),
                    active_slot.sample_type,
                );
                let dest_slot_id = match active_slot.sample_type {
                    ProjectSampleSlotType::Static => free_static.pop().expect("No more destination slots."),
                    ProjectSampleSlotType::Flex => free_flex.pop().expect("No more destination slots."),
                    _ => 255,
                };

                debug!(
                    "Selected sample slot ID in destination project: {:#?}",
                    dest_slot_id,
                );

                debug!(
                    "Updating sample slot reference in pattern p-locks: n={:#?} total={:#?} type={:#?}",
                    slot_idx,
                    active_slots.len(),
                    active_slot.sample_type,
                );

                for pattern in bank.patterns.iter_mut() {
                    pattern
                        .update_plock_sample_slots(
                            &active_slot.sample_type,
                            &active_slot.slot_id,
                            &dest_slot_id,
                        )
                        .expect("Could not update sample slot reference in pattern p-locks.");
                }

                debug!(
                    "Updating sample slot reference in unsaved part audio track machines: n={:#?} total={:#?} type={:#?}",
                    slot_idx,
                    active_slots.len(),
                    active_slot.sample_type,
                );
                for part in bank.parts_unsaved.iter_mut() {
                    part.update_machine_sample_slot(
                        &active_slot.sample_type,
                        &active_slot.slot_id,
                        &dest_slot_id,
                    )
                    .expect("Could not update sample slot reference in unsaved part audio track machine.");
                }

                debug!(
                    "Updating sample slot reference in saved part audio track machines: n={:#?} total={:#?} type={:#?}",
                    slot_idx,
                    active_slots.len(),
                    active_slot.sample_type,
                );
                for part in bank.parts_saved.iter_mut() {
                    part.update_machine_sample_slot(
                        &active_slot.sample_type,
                        &active_slot.slot_id,
                        &dest_slot_id,
                    )
                    .expect("Could not update sample slot reference in saved part audio track machine.");
                }

                debug!(
                    "Creating new project sample slot data: n={:#?} total={:#?} type={:#?}",
                    slot_idx,
                    active_slots.len(),
                    active_slot.sample_type,
                );

                // `blah/blah/my_audio_file.wav`
                // or `blah/AUDIO/my_audio_file.wav`
                let src_path_audio = &source_project_filepath
                    .parent()
                    .unwrap()
                    .to_path_buf()
                    .join(&active_slot.path);

                // `blah/blah/project.work/../../AUDIO/`
                let dest_path_audio = &destination_project_filepath
                    .parent()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .to_path_buf()
                    .join("AUDIO")
                    .join(&active_slot.path.file_name().unwrap());

                let new_sslot = ProjectSampleSlot::new(
                    active_slot.sample_type.clone(),
                    dest_slot_id.clone(),
                    dest_path_audio.clone(),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                )
                .expect("Could not create new sample slot.");

                debug!(
                    "Copying audio file: n={:#?} total={:#?} type={:#?}",
                    slot_idx,
                    active_slots.len(),
                    active_slot.sample_type,
                );

                let _ = std::fs::copy(&src_path_audio, &dest_path_audio)
                .expect(
                    format!("Could not copy audio file: src={:#?} dest={:#?}", src_path_audio, dest_path_audio).as_str(),
                );

                let mut src_path_sample_attr = src_path_audio.clone();
                src_path_sample_attr.set_extension("ot");

                let mut dest_path_sample_attr = dest_path_audio.clone();
                dest_path_sample_attr.set_extension("ot");

                if src_path_sample_attr.exists() {
                    debug!(
                        "Copying sample attributes file: n={:#?} total={:#?} type={:#?}",
                        slot_idx,
                        active_slots.len(),
                        active_slot.sample_type,
                    );
                    let _ = std::fs::copy(&src_path_sample_attr, &dest_path_sample_attr)
                    .expect(
                        format!("Could not copy sample attributes file: src={:#?} dest={:#?}", src_path_sample_attr, dest_path_sample_attr).as_str(),
                    );
                }

                debug!(
                    "Sample slot references updated: n={:#?} total={:#?} type={:#?}",
                    slot_idx,
                    active_slots.len(),
                    active_slot.sample_type,
                );
                new_sslot
            }
        )
        .collect();

    info!("Inserting new sample slots into destination project ...");
    let mut dest_sample_slots: Vec<ProjectSampleSlot> = dest_project.slots;
    dest_sample_slots.append(&mut updated_sample_slots);

    info!("Writing destination project ...");
    dest_project.slots = dest_sample_slots;
    dest_project
        .to_path(destination_project_filepath)
        .expect("Could not write project to file");

    info!("Writing new bank file ...");
    bank.to_path(destination_bank_filepath)
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
    let conf = YamlCopyBankConfig::from_path(yaml_config_path)?;

    for x in conf.bank_copies {
        let _ = copy_bank(&x.src.bank, &x.src.project, &x.dest.bank, &x.dest.project);
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

        let audio_pool = PathBuf::from("../data/tests/copy/bank/AUDIO-TEST");

        let inbank = PathBuf::from("../data/tests/copy/bank/BANK-COPY-SRC/bank01.work");
        let outbank = PathBuf::from("../data/tests/copy/bank/BANK-COPY-DUMMY/bank01.work");

        let inproject = PathBuf::from("../data/tests/copy/bank/BANK-COPY-SRC/project.work");
        let outproject = PathBuf::from("../data/tests/copy/bank/BANK-COPY-DUMMY/project.work");

        let _ = std::fs::create_dir(&audio_pool);

        // copy test destination project to a new directory, so we have a fresh test each time
        let _ = copy_dir::copy_dir(
            PathBuf::from("../data/tests/copy/bank/BANK-COPY-DEST/"),
            outbank.parent().unwrap(),
        );

        let _source_bank = Bank::from_path(&inbank).unwrap();
        let _ = copy_bank(&inbank, &inproject, &outbank, &outproject);
        let _copied_bank = Bank::from_path(&outbank).unwrap();

        // remove the test destination project directory
        let _ = std::fs::remove_dir_all(outbank.parent().unwrap());
        let _ = std::fs::remove_dir_all(audio_pool);

        assert!(true);
    }
}
