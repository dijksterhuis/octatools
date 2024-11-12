//! Functions for CLI actions related to copying Octatrack data,
//! such as `Bank`s, `Pattern`s, `Part`s or `Project`s.

use log::{debug, error, info, trace, warn};
use serde_octatrack::constants::OCTATRACK_AUDIO_FILE_BIT_DEPTH;
use std::ffi::OsStr;
use std::{collections::HashSet, path::PathBuf};

use crate::common::RBoxErr;

use serde_octatrack::{
    banks::Bank,
    common::{FromFileAtPathBuf, ProjectToString, ToFileAtPathBuf},
    projects::{options::ProjectSampleSlotType, slots::ProjectSampleSlot, Project},
};

/// Helper struct for tracking sample slots being used within a `Bank`.
#[derive(Debug, PartialEq, Eq, Hash)]
struct ActiveSampleSlot {
    slot_id: u8,
    sample_type: ProjectSampleSlotType,
}

/// Helper struct to hold `Project` metadata (and the `Project` itself).
#[derive(Debug, Clone)]
struct ProjectMetaStore {
    path: PathBuf,
    dirpath: PathBuf,
    project: Project,
    sample_slots: Vec<ProjectSampleSlot>,
}

/// Helper struct to refer to both source and destination `Project`s.
#[derive(Clone)]
struct TransferMetaProject {
    src: ProjectMetaStore,
    dest: ProjectMetaStore,
}

/// Helper struct to hold `Bank` metadata (and the `Bank` itself).
struct BankMetaStore {
    path: PathBuf,
    bank: Bank,
}

/// Helper struct to refer to both source and destination `Bank`s.
struct TransferMetaBank {
    src: BankMetaStore,
    dest: BankMetaStore,
}

/// Create a `ProjectMetaStore` for easier references to project data when copying banks.
fn get_project_metastorage(bank_file_path: PathBuf) -> RBoxErr<ProjectMetaStore> {
    let path = get_project_path_from_bank_file_path(bank_file_path)?;
    let dirpath = path.parent().unwrap().to_path_buf();
    let project = Project::from_pathbuf(path.clone()).unwrap();
    let sample_slots = get_project_sslots(project.clone())?;

    let p = ProjectMetaStore {
        path,
        dirpath,
        project,
        sample_slots,
    };

    Ok(p)
}

/// Work out the Source/Dest project file path from a bank file.
fn get_project_path_from_bank_file_path(path: PathBuf) -> RBoxErr<PathBuf> {
    // todo: unwrap on an option. need to handle none case
    println!("EBUGPATH: {:#?}", path);
    let strd = path.parent().unwrap().to_path_buf().join("project.strd");
    println!("EBUGPATH: {:#?}", strd);
    let project_path = match strd.exists() {
        true => strd,
        false => path.parent().unwrap().to_path_buf().join("project.work"),
    };
    println!("EBUGPATH: {:#?}", project_path);

    Ok(project_path)
}

/// Get the current sample slots for a `Project`
fn get_project_sslots(project: Project) -> RBoxErr<Vec<ProjectSampleSlot>> {
    let sample_slots: Vec<ProjectSampleSlot> = project
        .slots
        .into_iter()
        .filter(|x| x.sample_type != ProjectSampleSlotType::RecorderBuffer) // no recording buffers
        .collect();

    Ok(sample_slots)
}

/// Find free sample slot locations in a `Project`
fn find_free_sslots(projects: TransferMetaProject) -> RBoxErr<(Vec<u8>, Vec<u8>)> {
    let mut base_vec: Vec<u8> = vec![];
    for i in 1..=128 {
        base_vec.push(i)
    }
    let mut free_static_sample_slots_ids: Vec<u8> = base_vec.clone();
    let mut free_flex_sample_slots_ids: Vec<u8> = base_vec.clone();

    for slot in projects.dest.project.slots {
        match slot.sample_type {
            ProjectSampleSlotType::Static => {
                free_static_sample_slots_ids.retain(|x| *x != slot.slot_id as u8);
            }
            ProjectSampleSlotType::Flex => {
                free_flex_sample_slots_ids.retain(|x| *x != slot.slot_id as u8);
            }
            _ => {}
        }
    }

    // reverse so we can just use pop instead of needing to import VecDeque::pop_rev()
    free_static_sample_slots_ids.reverse();
    free_flex_sample_slots_ids.reverse();

    Ok((free_static_sample_slots_ids, free_flex_sample_slots_ids))
}

fn project_sample_slot_is_populated(
    project_slots: &Vec<ProjectSampleSlot>,
    slot_id: &u16,
    sample_type: &ProjectSampleSlotType,
) -> RBoxErr<bool> {
    let static_exists_in_project_slots = project_slots
        .iter()
        .find(|x| x.slot_id == *slot_id && x.sample_type == *sample_type);

    Ok(!static_exists_in_project_slots.is_none())
}

/// Find sample slot locations from a `Project` which are being used in a `Bank`
fn get_active_sslot_ids(
    project_slots: &Vec<ProjectSampleSlot>,
    bank: &Bank,
) -> HashSet<ActiveSampleSlot> {
    let mut active_slots: HashSet<ActiveSampleSlot> = HashSet::new();

    for (pattern_idx, pattern) in bank.patterns.iter().enumerate() {
        for (track_idx, audio_track_trigs) in pattern.audio_track_trigs.iter().enumerate() {
            for (plock_idx, plock) in audio_track_trigs.plocks.iter().enumerate() {
                if plock.sample_lock_static < 128 {
                    if project_sample_slot_is_populated(
                        project_slots,
                        &(plock.sample_lock_static as u16),
                        &ProjectSampleSlotType::Static,
                    )
                    .unwrap()
                    {
                        let x = ActiveSampleSlot {
                            slot_id: plock.sample_lock_static,
                            sample_type: ProjectSampleSlotType::Static,
                        };
                        info!("Found active Static sample plock: Pattern: {pattern_idx:#?} Track: {track_idx:#?} Trig: {plock_idx:#?} FlexSlot:{:#?}", plock.sample_lock_static);
                        active_slots.insert(x);
                    }
                }
                if plock.sample_lock_flex < 128 {
                    if project_sample_slot_is_populated(
                        project_slots,
                        &(plock.sample_lock_flex as u16),
                        &ProjectSampleSlotType::Flex,
                    )
                    .unwrap()
                    {
                        let x = ActiveSampleSlot {
                            slot_id: plock.sample_lock_flex,
                            sample_type: ProjectSampleSlotType::Flex,
                        };
                        info!("Found active Flex sample plock: Pattern: {pattern_idx:#?} Track: {track_idx:#?} Trig: {plock_idx:#?} FlexSlot:{:#?}", plock.sample_lock_flex);
                        active_slots.insert(x);
                    }
                }
            }
        }
    }

    for (part_idx, part) in bank.parts.iter().enumerate() {
        for (track_idx, audio_track_slots) in part.audio_track_machine_slots.iter().enumerate() {
            // the default sample slot for Static/Flex machines is the track ID.
            // so we check if there is an actual sample assigned to a machine's slot
            // to work out if the machine actually has an 'active' sample slot assignment or not.

            if project_sample_slot_is_populated(
                project_slots,
                &(audio_track_slots.static_slot_id as u16),
                &ProjectSampleSlotType::Static,
            )
            .unwrap()
            {
                let static_machine = ActiveSampleSlot {
                    slot_id: audio_track_slots.static_slot_id,
                    sample_type: ProjectSampleSlotType::Static,
                };
                active_slots.insert(static_machine);
                info!("Found active Static machine usage: Part: {part_idx:#?} Track: {track_idx:#?} StaticSlot:{:#?}", audio_track_slots.static_slot_id);
            }

            if project_sample_slot_is_populated(
                project_slots,
                &(audio_track_slots.flex_slot_id as u16),
                &ProjectSampleSlotType::Flex,
            )
            .unwrap()
            {
                let flex_machine = ActiveSampleSlot {
                    slot_id: audio_track_slots.flex_slot_id,
                    sample_type: ProjectSampleSlotType::Flex,
                };
                active_slots.insert(flex_machine);
                info!("Found active Flex machine usage: Part: {part_idx:#?} Track: {track_idx:#?} StaticSlot:{:#?}", audio_track_slots.flex_slot_id);
            }
        }
    }

    active_slots
}

/// Update Static sample slots references within a Bank.
fn update_sslot_references_static(
    project: &mut Project,
    banks: &mut TransferMetaBank,
    active_slot_id: u8,
    dest_slot_id: u8,
) -> () {
    for part in banks.src.bank.parts.iter_mut() {
        part.update_static_machine_slot(&active_slot_id, &dest_slot_id);
    }
    for pattern in banks.src.bank.patterns.iter_mut() {
        pattern.update_static_sample_plocks(&active_slot_id, &dest_slot_id);
    }

    println!(
        "SLOTS TO CHANGE dest -- {:#?} : {:#?}",
        dest_slot_id, project.slots
    );
    project.update_sample_slot_id(
        &active_slot_id,
        &dest_slot_id,
        Some(ProjectSampleSlotType::Static),
    );

    println!("CHANGED SLOTS: {:#?}", project.slots);
}

/// Update Flex sample slots references within a Bank.
fn update_sslot_references_flex(
    project: &mut Project,
    banks: &mut TransferMetaBank,
    active_slot_id: u8,
    dest_slot_id: u8,
) -> () {
    for part in banks.src.bank.parts.iter_mut() {
        part.update_flex_machine_slot(&active_slot_id, &dest_slot_id);
    }
    for pattern in banks.src.bank.patterns.iter_mut() {
        pattern.update_flex_sample_plocks(&active_slot_id, &dest_slot_id);
    }

    println!("SLOTS TO CHANGE: {:#?}", project.slots);

    project.update_sample_slot_id(
        &active_slot_id,
        &dest_slot_id,
        Some(ProjectSampleSlotType::Flex),
    );

    println!("CHANGED SLOTS: {:#?}", project.slots);
}

/// Get the file name of the audio file for a slot.
fn get_sslot_audio_file_fname(slot: &ProjectSampleSlot) -> RBoxErr<&OsStr> {
    Ok(slot.path.file_name().unwrap())
}

/// Get the file name of the audio file for a slot.
fn get_sslot_attributes_file_path(
    src_proj_path: &PathBuf,
    slot: &ProjectSampleSlot,
) -> RBoxErr<Option<PathBuf>> {
    let mut sample_attr_path_pool = src_proj_path
        .join("../AUDIO")
        .join(get_sslot_audio_file_fname(&slot)?);
    sample_attr_path_pool.set_extension("ot");

    let mut sample_attr_path_project = src_proj_path.join(get_sslot_audio_file_fname(&slot)?);
    sample_attr_path_project.set_extension("ot");

    let mut res: Option<PathBuf> = None;

    // exists in project
    if sample_attr_path_project.exists() {
        res = Some(sample_attr_path_project);
    } else if !sample_attr_path_pool.exists() {
        res = Some(sample_attr_path_pool);
    };

    Ok(res)
}

/// Resolve absolute paths for an audio file in a sample slot
/// TODO: Need to look for Sample Attributes files too!
fn get_abs_paths_for_sslot_audio_file(
    src_proj_dirpath: &PathBuf,
    dest_proj_dirpath: &PathBuf,
    slot: &ProjectSampleSlot,
) -> RBoxErr<(PathBuf, PathBuf)> {
    trace!("Getting absolute file paths for sample slot audio file.");
    let true_src_path = src_proj_dirpath.join(&slot.path);

    let fname = get_sslot_audio_file_fname(&slot)?;
    let true_dest_path = dest_proj_dirpath
        .parent()
        .unwrap()
        .join("AUDIO")
        .join(fname);

    Ok((true_src_path, true_dest_path))
}

/// Create a new relative path for an audio file in an audio pool.
/// From a project saple slot the audio pool path is always: "../AUDIO/<fname>.<ext>"
fn get_relative_audio_pool_path_audio_file(slot: &ProjectSampleSlot) -> RBoxErr<PathBuf> {
    let fname = get_sslot_audio_file_fname(&slot).unwrap();
    let relative_path = PathBuf::from("../AUDIO").join(fname);

    Ok(relative_path)
}

fn copy_file(src: &PathBuf, dest: &PathBuf) -> () {
    trace!("Copying file: from={src:#?} to={dest:#?}");
    let _ = std::fs::copy(src, dest);
    debug!("Copied file: from={src:#?} to={dest:#?}");
}

fn maybe_copy_ot_attr_file(src_path: &PathBuf, dest_path: &PathBuf) -> () {
    let mut ot_attr_src_fpath = src_path.clone();
    ot_attr_src_fpath.set_extension("ot");

    if ot_attr_src_fpath.exists() {
        let mut ot_attr_dest_fpath = dest_path.clone();
        ot_attr_dest_fpath.set_extension("ot");

        let _ = copy_file(&ot_attr_src_fpath, &ot_attr_dest_fpath);
    }
}

/// If necessary, copy audio files to a new audio pool location and change the path for the sample slot.
fn copy_sslot_sample_files(
    projects: &TransferMetaProject,
    slot: &mut ProjectSampleSlot,
) -> RBoxErr<()> {
    debug!("Copying audio file for sample slot");

    let (src_path, dest_path) = get_abs_paths_for_sslot_audio_file(
        &projects.src.dirpath.join(&slot.path),
        &projects.dest.dirpath,
        &slot,
    )?;

    let _ = copy_file(&src_path, &dest_path);
    let _ = maybe_copy_ot_attr_file(&src_path, &dest_path);

    Ok(())
}

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

pub fn copy_bank(source_bank_file_path: PathBuf, dest_bank_file_path: PathBuf) -> Result<(), ()> {
    // read projects
    let mut projects = TransferMetaProject {
        src: get_project_metastorage(source_bank_file_path.clone()).unwrap(),
        dest: get_project_metastorage(dest_bank_file_path.clone()).unwrap(),
    };

    println!(
        "PROJECT SLOTS | SRC | START: {:#?}",
        projects.src.project.slots
    );
    println!(
        "PROJECT SLOTS | DEST | START: {:#?}",
        projects.dest.project.slots
    );
    println!("PROJECT REWRITE",);

    let _ = projects.dest.project.to_pathbuf(projects.dest.path.clone());

    // read banks
    let mut banks = TransferMetaBank {
        src: BankMetaStore {
            bank: Bank::from_pathbuf(source_bank_file_path.clone()).unwrap(),
            path: source_bank_file_path,
        },
        dest: BankMetaStore {
            bank: Bank::from_pathbuf(dest_bank_file_path.clone()).unwrap(),
            path: dest_bank_file_path.clone(),
        },
    };

    println!(
        "BANK | SRC | PART | START: {:#?}",
        banks.src.bank.parts[0].audio_track_machine_slots
    );
    println!(
        "BANK | DEST | PART | START: {:#?}",
        banks.dest.bank.parts[0].audio_track_machine_slots
    );

    // create backups of the destination data files
    let _ = std::fs::copy(
        projects.dest.path.clone(),
        PathBuf::from("/tmp/project.bak"),
    );
    let _ = std::fs::copy(banks.dest.path.clone(), PathBuf::from("/tmp/bank.bak"));

    // find possible free space in destination project's sample slots
    let (mut free_static, mut free_flex) = find_free_sslots(projects.clone()).unwrap();

    info!(
        "Destination project has free sample slots: {:#?} static; {:#?} flex.",
        free_static.len(),
        free_flex.len()
    );

    let src_static_sslot_count = projects
        .src
        .sample_slots
        .iter()
        .filter(|x| x.sample_type == ProjectSampleSlotType::Static)
        .count();

    let src_flex_sslot_count = projects
        .src
        .sample_slots
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

    // read the source bank, looking for sample slots in active use
    let active_slots = get_active_sslot_ids(&projects.src.project.slots, &banks.src.bank);

    info!(
        "\"Active\" sample slots in source bank: {:#?}",
        active_slots,
    );

    let mut src_proj = projects.src.project.clone();
    let mut updated_sample_slots: Vec<ProjectSampleSlot> = vec![];

    // edit the bank data in place, updating:
    // - project's sample slot;
    // - sample plocks reference to project sample slot;
    // - audio track machine assignment reference to project sample slot.

    for active_slot in active_slots {
        let new_slot_id = match active_slot.sample_type {
            ProjectSampleSlotType::Static => {
                let dest_slot_id = free_static.pop().unwrap();
                update_sslot_references_static(
                    &mut src_proj,
                    &mut banks,
                    active_slot.slot_id,
                    dest_slot_id,
                );

                dest_slot_id
            }
            ProjectSampleSlotType::Flex => {
                let dest_slot_id = free_flex.pop().unwrap();
                update_sslot_references_flex(
                    &mut src_proj,
                    &mut banks,
                    active_slot.slot_id,
                    dest_slot_id,
                );

                dest_slot_id
            }
            ProjectSampleSlotType::RecorderBuffer => {
                warn!("Usupported behaviour: Attempted to update a Recording Buffer sample slot reference.");
                255
            }
        };

        let src_project_slot = src_proj
            .slots
            .clone()
            .into_iter()
            .find(|x| x.slot_id == new_slot_id as u16);

        if !src_project_slot.is_none() {
            let mut s = src_project_slot.unwrap();
            if s.sample_type != ProjectSampleSlotType::RecorderBuffer {
                let _ = copy_sslot_sample_files(&projects, &mut s);
                s.path = get_relative_audio_pool_path_audio_file(&s).unwrap();
                updated_sample_slots.push(s);
            }
        }
    }

    let mut dest_sample_slots: Vec<ProjectSampleSlot> = projects.dest.project.slots.clone();
    dest_sample_slots.append(&mut updated_sample_slots);

    // update sample slots for the destination project
    projects.dest.project.slots = dest_sample_slots;

    println!(
        "bank changes: {:#?}",
        banks.src.bank.parts[0].audio_track_machine_slots
    );

    let _ = projects.src.project.to_pathbuf(projects.src.path);
    let _ = projects.dest.project.to_pathbuf(projects.dest.path);
    // write new bank data over old bank file
    // let _ = banks.dest.bank.to_pathbuf(dest_bank_file_path);

    // write over project file -- todo! requires Project to be serializable!
    // let _ = dest_proj.to_pathbuf(projects.dest.path);

    Ok(())
}
