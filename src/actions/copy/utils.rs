use log::{debug, info, trace};

use std::ffi::OsStr;
use std::{collections::HashSet, path::PathBuf};

use crate::common::RBoxErr;

use serde_octatrack::{
    banks::Bank,
    projects::{options::ProjectSampleSlotType, slots::ProjectSampleSlot, Project},
    FromPathBuf,
};

/// Helper struct for tracking sample slots being used within a `Bank`.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ActiveSampleSlot {
    pub slot_id: u8,
    pub sample_type: ProjectSampleSlotType,
}

/// Helper struct to hold `Project` metadata (and the `Project` itself).
#[derive(Debug, Clone)]
pub struct TransferProjectMeta {
    pub path: PathBuf,
    pub dirpath: PathBuf,
    pub project: Project,
}

impl TransferProjectMeta {
    /// Work out the Source/Dest project file path from a bank file.
    fn get_project_dirpath_from_bank_fpath(path: &PathBuf) -> RBoxErr<PathBuf> {
        let project_dirpath = path.parent().unwrap().to_path_buf();
        Ok(project_dirpath)
    }

    /// Create a `ProjectMetaStore` for easier references to project data when copying banks.
    fn from_pathbuf(fpath: &PathBuf) -> RBoxErr<Self> {
        let dirpath = Self::get_project_dirpath_from_bank_fpath(&fpath)?;
        let path = dirpath.join("project.work");
        let project = Project::from_pathbuf(&path).unwrap();

        Ok(TransferProjectMeta {
            path,
            dirpath,
            project,
        })
    }
}

/// Helper struct to refer to both source and destination `Project`s.
#[derive(Clone)]
pub struct TransferProject {
    pub src: TransferProjectMeta,
    pub dest: TransferProjectMeta,
}

impl TransferProject {
    pub fn new(src: &PathBuf, dest: &PathBuf) -> RBoxErr<Self> {
        Ok(TransferProject {
            src: TransferProjectMeta::from_pathbuf(src)?,
            dest: TransferProjectMeta::from_pathbuf(dest)?,
        })
    }
}
/// Helper struct to refer to both source and destination `Bank`s.
pub struct TransferBank {
    pub src: Bank,
    pub dest: Bank,
}

impl TransferBank {
    pub fn new(src: &PathBuf, dest: &PathBuf) -> RBoxErr<Self> {
        Ok(TransferBank {
            src: Bank::from_pathbuf(src)?,
            dest: Bank::from_pathbuf(dest)?,
        })
    }
}

/// Find free sample slot locations in a `Project`
pub fn find_free_sslots(projects: &TransferProject) -> RBoxErr<(Vec<u8>, Vec<u8>)> {
    let mut base_vec: Vec<u8> = vec![];
    for i in 1..=128 {
        base_vec.push(i)
    }
    let mut free_static_sample_slots_ids: Vec<u8> = base_vec.clone();
    let mut free_flex_sample_slots_ids: Vec<u8> = base_vec;

    for slot in projects.dest.project.slots.iter() {
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
    trace!("Checking if sample slot is populated within project ...");

    let static_exists_in_project_slots = project_slots
        .iter()
        .find(|x| x.slot_id == *slot_id && x.sample_type == *sample_type);

    Ok(!static_exists_in_project_slots.is_none())
}

/// Find sample slot locations from a `Project` which are being used in a `Bank`
pub fn get_active_sslot_ids(
    project_slots: &Vec<ProjectSampleSlot>,
    bank: &Bank,
) -> RBoxErr<HashSet<ActiveSampleSlot>> {
    let mut active_slots: HashSet<ActiveSampleSlot> = HashSet::new();

    debug!("Checking if sample slot is populated within project ...");

    for (pattern_idx, pattern) in bank.patterns.iter().enumerate() {
        for (track_idx, audio_track_trigs) in pattern.audio_track_trigs.iter().enumerate() {
            for (plock_idx, plock) in audio_track_trigs.plocks.iter().enumerate() {
                if plock.sample_lock_static < 128 {
                    if project_sample_slot_is_populated(
                        project_slots,
                        &(plock.sample_lock_static as u16),
                        &ProjectSampleSlotType::Static,
                    )? {
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
                    )? {
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
            )? {
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
            )? {
                let flex_machine = ActiveSampleSlot {
                    slot_id: audio_track_slots.flex_slot_id,
                    sample_type: ProjectSampleSlotType::Flex,
                };
                active_slots.insert(flex_machine);
                info!("Found active Flex machine usage: Part: {part_idx:#?} Track: {track_idx:#?} StaticSlot:{:#?}", audio_track_slots.flex_slot_id);
            }
        }
    }

    Ok(active_slots)
}

/// Update Static sample slots references within a Bank.
pub fn update_sslot_references_static(
    project: &mut Project,
    banks: &mut TransferBank,
    active_slot_id: u8,
    dest_slot_id: u8,
) -> RBoxErr<()> {
    debug!("Updating static sample slots for static machines in parts ...");
    for part in banks.src.parts.iter_mut() {
        part.update_static_machine_slot(&active_slot_id, &dest_slot_id)?;
    }
    debug!("Updating static sample slots for static plocks in patterns ...");
    for pattern in banks.src.patterns.iter_mut() {
        pattern.update_static_sample_plocks(&active_slot_id, &dest_slot_id)?;
    }
    debug!("Updating source project static sample slot location ...");
    project.update_sample_slot_id(
        &active_slot_id,
        &dest_slot_id,
        Some(ProjectSampleSlotType::Static),
    )?;

    Ok(())
}

/// Update Flex sample slots references within a Bank.
pub fn update_sslot_references_flex(
    project: &mut Project,
    banks: &mut TransferBank,
    active_slot_id: u8,
    dest_slot_id: u8,
) -> RBoxErr<()> {
    debug!("Updating flex sample slots for flex machines in parts ...");
    for part in banks.src.parts.iter_mut() {
        part.update_flex_machine_slot(&active_slot_id, &dest_slot_id)?;
    }
    debug!("Updating flex sample slots for flex plocks in patterns ...");
    for pattern in banks.src.patterns.iter_mut() {
        pattern.update_flex_sample_plocks(&active_slot_id, &dest_slot_id)?;
    }
    debug!("Updating source project flex sample slot location ...");
    project.update_sample_slot_id(
        &active_slot_id,
        &dest_slot_id,
        Some(ProjectSampleSlotType::Flex),
    )?;

    Ok(())
}

/// Get the file name of the audio file for a slot.
fn get_sslot_audio_file_fname(slot: &ProjectSampleSlot) -> RBoxErr<&OsStr> {
    Ok(slot.path.file_name().unwrap())
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
/// From a project saple slot the audio pool path is always: "../AUDIO/fname.ext"
pub fn get_relative_audio_pool_path_audio_file(slot: &ProjectSampleSlot) -> RBoxErr<PathBuf> {
    let fname = get_sslot_audio_file_fname(&slot).unwrap();
    let relative_path = PathBuf::from("../AUDIO").join(fname);

    Ok(relative_path)
}

fn copy_file(src: &PathBuf, dest: &PathBuf) -> RBoxErr<u64> {
    trace!("Copying file: from={src:#?} to={dest:#?}");
    let write_res = std::fs::copy(src, dest);
    debug!("Copied file: from={src:#?} to={dest:#?}");
    Ok(write_res?)
}

fn maybe_copy_ot_attr_file(src_path: &PathBuf, dest_path: &PathBuf) -> RBoxErr<()> {
    let mut ot_attr_src_fpath = src_path.clone();
    ot_attr_src_fpath.set_extension("ot");

    if ot_attr_src_fpath.exists() {
        let mut ot_attr_dest_fpath = dest_path.clone();
        ot_attr_dest_fpath.set_extension("ot");

        let _ = copy_file(&ot_attr_src_fpath, &ot_attr_dest_fpath)?;
    }
    Ok(())
}

/// If necessary, copy audio files to a new audio pool location and change the path for the sample slot.
pub fn copy_sslot_sample_files(
    projects: &TransferProject,
    slot: &ProjectSampleSlot,
) -> RBoxErr<()> {
    debug!("Copying audio file for sample slot ...");

    let (src_path, dest_path) =
        get_abs_paths_for_sslot_audio_file(&projects.src.dirpath, &projects.dest.dirpath, &slot)?;

    let _ = copy_file(&src_path, &dest_path)?;
    let _ = maybe_copy_ot_attr_file(&src_path, &dest_path)?;

    Ok(())
}
