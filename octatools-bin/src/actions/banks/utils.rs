
use chrono;

use crate::{OctatoolErrors, RBoxErr};
use itertools::Itertools;
use octatools_lib::samples::options::{
    SampleAttributeLoopMode, SampleAttributeTimestrechMode, SampleAttributeTrigQuantizationMode,
};
use octatools_lib::{
    banks::{parts::Part, patterns::Pattern, Bank},
    projects::{options::ProjectSampleSlotType, slots::ProjectSampleSlot, Project},
};
use std::{
    array::from_fn, cmp::PartialEq, collections::HashSet, ffi::OsStr, path::Path, path::PathBuf,
};

type SlotType = ProjectSampleSlotType;
type Slot = ProjectSampleSlot;

/// 1-indexed reservation of an existing "empty" slot in a project
struct ReservedEmptySlots {
    static_slot: u8,
    flex_slot: u8,
}

/// Project metadata, i.e. paths to file / directory
pub(crate) struct ProjectMeta {
    pub(crate) dirpath: PathBuf,
    pub(crate) filepath: PathBuf,
}


impl ProjectMeta {

    /// Load `ProjectMeta` data given a project directory path `PathBuf`
    pub(crate) fn frompath(dirpath: &Path) -> Self {
        let project_filepath = resolve_project_work_file_from_project_dirpath(&dirpath)
            .expect("Project file not found.");

        Self {
            dirpath: dirpath.to_path_buf(),
            filepath: project_filepath, // used later
        }
    }
}

/// Bank metadata, basically just a container for the path to the bank file
pub(crate) struct BankMeta {
    pub(crate) filepath: PathBuf,
}

impl BankMeta {
    /// Create a bank file from the project directory path and the bank's ID number (1-16 inclusive)
    pub(crate) fn frompath(dirpath: &Path, bank_id: usize) -> Self {
        let filepath = resolve_bank_work_file_from_project_dirpath(&dirpath, bank_id)
            .expect("Bank file not found.");

        Self { filepath }
    }
}

/// Metadata on all octatrack data paths used during bank copy operations.
pub(crate) struct BankCopyPathsMeta {
    pub(crate) project: ProjectMeta,
    pub(crate) bank: BankMeta,
}

/// Helper struct to make copying audio files cleaner (we create new sample slot
/// data first, then use the slot data to copy files).
pub(crate) struct SlotFileCopy {
    from: Slot,
    to: Slot,
}

/// Type of sample slot reference in bank data.
#[derive(Clone, Eq, Hash, PartialEq)]
pub(crate) enum BankSlotReferenceType {
    /// bank data points to a sample slot with a sample loaded in the slot
    Inactive,
    /// bank data points to a sample slot without no sample loaded
    Active,
}

/// Container data structure for data related to a sample slot reference.
#[derive(Clone, Eq, Hash, PartialEq)]
pub(crate) struct BankSlotReference {
    pub(crate) sample_type: SlotType,
    pub(crate) slot_id: u8,
    pub(crate) reference_type: BankSlotReferenceType,
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub(crate) struct SlotsSlotReference {
    pub(crate) sample_type: SlotType,
    pub(crate) slot_id: u8,
    pub(crate) op_type: SampleSlotOperationType,
}

struct SlotReferenceReassignment {
    initial_slot_id: u8,
    new_slot_id: u8,
    slot_type: SlotType,
}

/// Type of operation we'll be performing with repsect to sample slots
#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub(crate) enum SampleSlotOperationType {
    /// Create a brand new sample slot in the destination, will result in sample files being copied
    /// over with a hash suffix n the file names
    NewSlot,
    /// we can reuse some slot that will end up in the destination project -- the same sample files
    /// and slot settings will be present in the destination after all changes are made.
    ReuseSlot,
}

/// Container type holding necessary data for both building sets of changes and making the changes
#[derive(Debug, PartialEq, Clone, Eq, Hash)]
struct SampleSlotOperation {
    src_slot: Slot,
    dest_slot: Slot,
    op_type: SampleSlotOperationType,
}

/// Does a file or directory actually exist on the filesystem at this path?
fn check_file_exists(path: &Path) -> RBoxErr<()> {
    if !path.exists() {
        return Err(Box::new(OctatoolErrors::PathDoesNotExist));
    }

    Ok(())
}

/// Copy a given `*.work` file with the file extension suffixed to `*.work_octatools_UtcTimestamp`
pub(crate) fn create_backup_of_work_file(path: &Path) -> RBoxErr<()> {
    match check_file_exists(path) {
        Ok(_) => {
            let datetime = chrono::Utc::now().timestamp();
            let mut backup_filepath = path.to_path_buf();
            backup_filepath.set_extension(format!["work_octatools_{:?}", datetime]);
            println!("Creating working file backup: {backup_filepath:?}");
            let _ = std::fs::copy(path, backup_filepath)?;
            Ok(())
        }
        // passes through the validation error
        Err(e) => {
            eprintln!("Could not create backup of file: path={path:?}");
            Err(e)
        }
    }
}

/// Given a project directory path, check the `project.work` file exists and return the file path for it
fn resolve_project_work_file_from_project_dirpath(dirpath: &Path) -> RBoxErr<PathBuf> {
    let project_fpath = dirpath.join("project.work");
    match check_file_exists(&project_fpath) {
        Ok(_) => Ok(project_fpath),
        // passes through the validation error
        Err(e) => {
            eprintln!(
                "No `project.work` file found in project directory: expectedFilePath={:?}",
                project_fpath,
            );
            Err(e)
        }
    }
}

pub(crate) fn get_bank_fname_from_id(bank_id: usize) -> String {
    format!["bank{bank_id:0>2}.work"].to_string()
}

/// Given a project directory path and a bank ID number, check the `bank??.work` file exists and
/// return the file path for it
fn resolve_bank_work_file_from_project_dirpath(dirpath: &Path, bank_id: usize) -> RBoxErr<PathBuf> {
    let bank_fpath = dirpath.join(get_bank_fname_from_id(bank_id));
    match check_file_exists(&bank_fpath) {
        Ok(_) => Ok(bank_fpath),
        Err(e) => {
            eprintln!(
                "No matching bank file found in project directory: expectedFilePath={:?}",
                bank_fpath,
            );
            Err(e)
        }
    }
}

/// Find free sample slot locations in a `Project`.
fn find_free_sample_slot_ids(slots_inuse: &[Slot], slot_type: SlotType) -> RBoxErr<Vec<u8>> {
    let arr: [u8; 127] = from_fn(|i| i as u8);
    let mut free_slots: Vec<u8> = arr.to_vec();

    for slot in slots_inuse.iter() {
        if slot_type == slot.sample_type {
            free_slots.retain(|x| *x != slot.slot_id);
        }
    }

    // reverse so we can just use pop instead of needing to import VecDeque::pop_rev()
    free_slots.reverse();

    Ok(free_slots)
}

/// Get sample slot IDs that match a given sample type (Flex/Static).
fn get_sample_slot_ids(sample_slots: &[Slot], sample_type: &SlotType) -> Vec<u8> {
    let sample_slot_ids = sample_slots
        .iter()
        .filter(|x| x.sample_type == *sample_type)
        .map(|s| s.slot_id)
        .collect_vec();

    sample_slot_ids
}

/// Does this slot ID exist in the project sample slots for the given slot type?
/// Yes: Active. No: Inactive.
fn get_active_or_inactive_bank_slot_reference(slots: &[Slot], sample_type: SlotType, slot_id: u8) -> RBoxErr<BankSlotReference> {
    Ok(
        if get_sample_slot_ids(slots, &sample_type).contains(&slot_id) {
            BankSlotReference { sample_type, slot_id, reference_type: BankSlotReferenceType::Active }
        } else {
            BankSlotReference { sample_type, slot_id, reference_type: BankSlotReferenceType::Inactive }
        }
    )
}


//noinspection DuplicatedCode
/// Iterate over and into patterns to discover any references to sample slots.
/// - 'active' assignments point to a populated sample slot that has a sample file loaded
/// - 'inactive' assignments point to an empty sample slot (no sample file loaded)
// TODO: Iterator traits on parts / pattern P-lock collection types.
// TODO: Generic Collection types? Would these work with serde?
pub(crate) fn find_sample_slot_refs_in_patterns(
    slots: &[Slot],
    patterns: &[Pattern],
) -> RBoxErr<HashSet<BankSlotReference>> {
    let mut slot_usage: HashSet<BankSlotReference> = HashSet::new();

    for pattern in patterns.iter() {
        for audio_track_trigs in pattern.audio_track_trigs.iter() {
            // note: have to iter here because Box<Array<_>> is not an iterable
            for plock in audio_track_trigs.plocks.iter() {
                for (slot_id, sample_type) in [
                    (plock.static_slot_id, SlotType::Static),
                    (plock.flex_slot_id, SlotType::Flex),
                ] {
                    // plock slot reference is enabled
                    if slot_id != 255 {
                        slot_usage.insert(
                            get_active_or_inactive_bank_slot_reference(&slots, sample_type, slot_id)?
                        );
                    }
                }
            }
        }
    }

    Ok(slot_usage)
}

/// Iterate over and into parts to discover any references to sample slots.
// TODO: Iterator traits on parts / pattern P-lock collection types.
// TODO: Generic Collection types? Would these work with serde?
pub(crate) fn find_sample_slot_refs_in_parts(
    slots: &[Slot],
    parts: &[Part],
) -> RBoxErr<HashSet<BankSlotReference>> {
    let mut slot_usage: HashSet<BankSlotReference> = HashSet::new();

    for part in parts.iter() {
        for audio_track_slots in part.audio_track_machine_slots.iter() {
            for (slot_id, sample_type) in [
                (audio_track_slots.static_slot_id, SlotType::Static),
                (audio_track_slots.flex_slot_id, SlotType::Flex),
            ] {
                slot_usage.insert(
                    get_active_or_inactive_bank_slot_reference(&slots, sample_type, slot_id)?
                );
            }
        }
    }

    Ok(slot_usage)
}

/// Iterate through bank data to discover any references to sample slots.
///
/// This function is basically a wrapper around `find_sample_slot_refs_in_patterns`
/// and `find_sample_slot_refs_in_parts`.
pub(crate) fn find_sample_slot_refs_in_bank(
    slots: &[Slot],
    bank: &Bank,
) -> RBoxErr<HashSet<BankSlotReference>> {
    let pattern_slot_usage = find_sample_slot_refs_in_patterns(
        slots,
        &bank.patterns.as_slice(), // boxed serde_big_array needs to be sliced
    )?;

    let unsaved_part_slot_usage = find_sample_slot_refs_in_parts(
        slots,
        &bank.parts_unsaved.as_slice(), // boxed serde_big_array needs to be sliced
    )?;

    Ok(pattern_slot_usage
        .union(&unsaved_part_slot_usage)
        .cloned()
        .collect::<HashSet<BankSlotReference>>())
}

/// Get the file path of a sample's `.ot` file as a new/cloned `PathBuf`, given the audio file path,
/// (change fle extension to `.ot`). Note: These files are not guaranteed to exist.
fn resolve_otfile_fpath_from_audio_fpath(path: &Path) -> PathBuf {
    let mut ot_filepath = path.to_path_buf();
    ot_filepath.set_extension("ot");
    ot_filepath
}

/// Extract the filename and file extension from a `PathBuf` as a single String,
/// i.e. `PathBuf("some/path/to/file.ext")` -> `"file.ext"`.
pub(crate) fn resolve_fname_and_fext_from_path(path: &Path) -> RBoxErr<String> {
    if !path.extension().is_some() {
        return Err(Box::new(OctatoolErrors::InvalidFilenameOrExtension));
    }

    Ok(format![
        "{name}.{ext}",
        name = &path.file_stem().and_then(OsStr::to_str).unwrap(),
        ext = &path.extension().and_then(OsStr::to_str).unwrap(),
    ])
}

/// Find the greatest valid slot ID which isn't populated in the given sample slots.
/// Assumes zero-indexing.
fn find_last_empty_slot(slots: &[Slot], sample_type: &SlotType) -> RBoxErr<u8> {
    let mut used_slot_ids = slots
        .iter()
        .filter(|x| x.sample_type == *sample_type)
        .map(|x| x.slot_id);

    let mut possible_ids: [u8; 128] = from_fn(|x| x as u8);
    possible_ids.reverse();

    possible_ids
        .iter()
        .filter(|x| !used_slot_ids.contains(x))
        .cloned()
        .next()
        .ok_or(Box::new(OctatoolErrors::CliNoFreeSampleSlots))
}

/// Find a match for a given sample slot based only on the SETTINGS of the sample slot, i.e. do not
/// match on `slot_id`
fn find_sample_slot_settings_match(candidate: &Slot, slots: &[Slot]) -> Option<Slot> {
    slots
        .iter()
        .filter(|x| {
            x.sample_type == candidate.sample_type
                // TODO: ignore path equality?
                && x.path == candidate.path
                && x.trim_bars_x100 == candidate.trim_bars_x100
                && x.trig_quantization_mode == candidate.trig_quantization_mode
                && x.timestrech_mode == candidate.timestrech_mode
                && x.loop_mode == candidate.loop_mode
                && x.gain == candidate.gain
                && x.bpm == candidate.bpm
        })
        // first one in ascending order
        .sorted_by(|x, y| Ord::cmp(&x.slot_id, &y.slot_id))
        .cloned()
        .next()
}

/// Create a new sample slot based on another sample slot.
/// Always copies the slot's settings and, depending on options provided, maybe
/// copies: the path and/or slot_id.
fn create_sample_slot_from_existing(
    existing: &Slot,
    new_path: Option<&Path>,
    new_id: &u8,
) -> RBoxErr<Slot> {
    let slot_id = *new_id;
    let path = new_path.unwrap_or(&existing.path).to_path_buf();

    Slot::new(
        existing.sample_type.clone(),
        slot_id,
        path,
        Some(existing.trim_bars_x100),
        Some(existing.timestrech_mode),
        Some(existing.loop_mode),
        Some(existing.trig_quantization_mode),
        Some(existing.gain),
        Some(existing.bpm),
    )
}


/// Get a new vector of project slots, zero-indexed
pub(crate) fn get_zero_indexed_slots_from_one_indexed(slots: &[Slot]) -> RBoxErr<Vec<Slot>> {
    Ok(slots
        .iter()
        .map(|s| {
            let new_slot_id = s.slot_id - 1;
            let mut new_slot = s.clone();
            new_slot.slot_id = new_slot_id;
            new_slot
        })
        .collect())
}

/// Get a new vector of project slots, one-indexed
fn get_one_indexed_slots_from_zero_indexed(slots: &[Slot]) -> RBoxErr<Vec<Slot>> {
    Ok(slots
        .iter()
        .map(|s| {
            let new_slot_id = s.slot_id + 1;
            let mut new_slot = s.clone();
            new_slot.slot_id = new_slot_id;
            new_slot
        })
        .collect())
}



// TODO: The basics of an `octatools-bin projects slots deduplicate` command, except the actual
//       command will need to mutate sample files to have content hash strings in the file names.
//       Although, this is only to catch the edge case where someone has files named the same in
//       different projects... how likely is it to happen where they aren't actually the same file?
/// Assumes zero-indexing on both inputs
fn get_deduplicated_sample_slots_and_updated_banks(
    slots: &[Slot],
    banks: &[Bank],
) -> RBoxErr<(Vec<Slot>, Vec<Bank>)> {
    // everything except slot id
    // don't worry about file name hash suffixes here.
    let mut deduped = slots
        .iter()
        .cloned()
        .unique_by(|x| {
            (
                x.sample_type.clone(),
                x.path.clone(),
                x.gain,
                x.loop_mode,
                x.timestrech_mode,
                x.trig_quantization_mode,
                x.bpm,
                x.trim_bars_x100,
            )
        })
        .collect_vec();

    // original -> deduplicated
    let mut reassignments: Vec<SlotReferenceReassignment> = vec![];

    for slot in slots {
        if !deduped.contains(&slot) {
            if let Some(found) = find_sample_slot_settings_match(&slot, &deduped) {
                reassignments.push(SlotReferenceReassignment {
                    initial_slot_id: slot.slot_id,
                    new_slot_id: found.slot_id,
                    slot_type: found.sample_type,
                });
            } else {
                deduped.push(slot.clone());
            }
        }
    }

    let mut new_banks = banks.to_owned();

    for reassignment in reassignments {
        for bank in &mut new_banks {
            bank.patterns.iter_mut().for_each(|p| {
                p.update_plock_sample_slots(
                    &reassignment.slot_type,
                    &reassignment.initial_slot_id,
                    &reassignment.new_slot_id,
                )
                    .expect("Failed to update sample slot reference in pattern p-locks.");
            });
            bank.parts_unsaved.iter_mut().for_each(|p| {
                p.update_machine_sample_slot(
                    &reassignment.slot_type,
                    &reassignment.initial_slot_id,
                    &reassignment.new_slot_id,
                )
                    .expect(
                        "Failed to update sample slot reference in unsaved part audio track machine.",
                    );
            });
        }
    }

    Ok((deduped, new_banks))
}


/// Apply required sample slot changes to the provided bank, in place.
fn apply_slot_changes_to_bank_inplace(operations: &HashSet<SampleSlotOperation>, bank: &mut Bank) {
    operations
        .iter()
        // descending order of dest_slot_id, otherwise we can end up changing slot references twice
        .sorted_by(|x, y| Ord::cmp(&y.src_slot.slot_id, &x.src_slot.slot_id))
        .for_each(|x| {
            println!(
                "changeType={:?} slotType={:?} srcSlotId={} newSlotId={}",
                x.op_type, x.src_slot.sample_type, x.src_slot.slot_id, x.dest_slot.slot_id,
            );

            // TODO: I don't like the fact this is being applied by a method provided by lib
            //       ... lib is only for io/serde/parsing/defaults etc
            //       ... it shouldn't have setters etc.

            bank.patterns.iter_mut().for_each(|p| {
                p.update_plock_sample_slots(&x.src_slot.sample_type, &x.src_slot.slot_id, &x.dest_slot.slot_id)
                    .expect("Failed to update sample slot reference in pattern p-locks.");
            });
            bank.parts_unsaved.iter_mut().for_each(|p| {
                p.update_machine_sample_slot(&x.src_slot.sample_type, &x.src_slot.slot_id, &x.dest_slot.slot_id).expect(
                    "Failed to update sample slot reference in unsaved part audio track machine.",
                );
            });
        });
}

/// Calculate data changes required to copy a bank from one project location to another. This
/// function can also be used to calculate the data changes required to copy banks within the same
/// project (swap the banks).
pub fn calculate_copy_bank_changes(
    src_project_dirpath: &Path,
    src_project: &Project,
    src_bank: &Bank,
    dest_project: &Project,
) -> RBoxErr<(Project, Bank, Vec<SlotFileCopy>)> {

    println!("Calculating destination project slot changes ...");

    let (deduped_src_zero_indexed_slots, deduped_banks) =
        get_deduplicated_sample_slots_and_updated_banks(
            &get_zero_indexed_slots_from_one_indexed(&src_project.slots)?,
            &[src_bank.to_owned()],
        )?;

    // we will be changing this in place so needs to be mutable
    let mut deduped_bank = deduped_banks[0].to_owned();

    let dest_zero_indexed_slots = get_zero_indexed_slots_from_one_indexed(&dest_project.slots)?;

    // this is so we can remap 'inactive' sample slot references somewhere safer
    let static_empty_slot =
        find_last_empty_slot(&dest_zero_indexed_slots, &ProjectSampleSlotType::Flex)
            .expect("No empty static sample slot in destination project.");
    let flex_empty_slot =
        find_last_empty_slot(&dest_zero_indexed_slots, &ProjectSampleSlotType::Flex)
            .expect("No empty flex sample slot in destination project.");

    println!(
        "Will use static/flex slots {:?}/{:?} for remapping inactive slot references.",
        static_empty_slot,
        flex_empty_slot,
    );

    // can't do anything without a free static/flex slot
    let reserved_slot_meta = ReservedEmptySlots {
        static_slot: static_empty_slot,
        flex_slot: flex_empty_slot,
    };

    let mut free_static =
        find_free_sample_slot_ids(&dest_zero_indexed_slots, ProjectSampleSlotType::Static)
            .expect("Failed to find free static sample slots in destination project.");

    let mut free_flex =
        find_free_sample_slot_ids(&dest_zero_indexed_slots, ProjectSampleSlotType::Flex)
            .expect("Failed to find free flex sample slots in destination project.");

    free_static.retain(|x| *x != reserved_slot_meta.static_slot);
    free_flex.retain(|x| *x != reserved_slot_meta.flex_slot);

    // TODO: reverse ordering -- insert new sample slots starting from 1-indexed sample slot 127
    //       then we should get fewer conflicts between active and inactive slot refs
    //       ... will need to update all the tests :/
    // free_static.sort_by(|x, y| Ord::cmp(x, y));
    // free_flex.sort_by(|x, y| Ord::cmp(x, y));

    println!(
        "There are {:?}/{:?} static/flex slots available for additions.",
        free_static.len(),
        free_flex.len(),
    );

    let bank_slot_refs =
        find_sample_slot_refs_in_bank(&deduped_src_zero_indexed_slots, &deduped_bank)
            .expect("Failed to resolve sample slots usage within source bank data.");

    // the set of source slots where
    // - the slot can be mapped onto existing destination slots
    // - the slot is referenced in the bank we are going to copy
    let src_slots_reuses = deduped_src_zero_indexed_slots
        .iter()
        .filter(|src| find_sample_slot_settings_match(&src, &dest_zero_indexed_slots).is_some())
        .cloned()
        .map(|src| SlotsSlotReference {
            sample_type: src.sample_type,
            slot_id: src.slot_id,
            op_type: SampleSlotOperationType::ReuseSlot,
        })
        .filter(|slot| {
            bank_slot_refs
                .iter()
                .find(|x| x.slot_id == slot.slot_id)
                .is_some()
        })
        .collect::<HashSet<_>>();

    // the set of source slots where
    // - the slot may need to be inserted into destination slots
    // - the slot is referenced in the bank we are going to copy
    let src_slots_inserts = deduped_src_zero_indexed_slots
        .iter()
        .filter(|src| find_sample_slot_settings_match(&src, &dest_zero_indexed_slots).is_none())
        .cloned()
        .map(|src| SlotsSlotReference {
            sample_type: src.sample_type,
            slot_id: src.slot_id,
            op_type: SampleSlotOperationType::NewSlot,
        })
        .filter(|slot| {
            bank_slot_refs
                .iter()
                .find(|x| x.slot_id == slot.slot_id)
                .is_some()
        })
        .collect::<HashSet<_>>();

    let static_slot_inserts_count = src_slots_inserts
        .iter()
        .filter(|x| x.sample_type == ProjectSampleSlotType::Static)
        .count();

    let flex_slot_inserts_count = src_slots_inserts
        .iter()
        .filter(|x| x.sample_type == ProjectSampleSlotType::Flex)
        .count();

    println!(
        "Will reuse {:?}/{:?} destination static/flex slots.",
        src_slots_reuses.iter().filter(|x|x.sample_type == ProjectSampleSlotType::Static).count(),
        src_slots_reuses.iter().filter(|x|x.sample_type == ProjectSampleSlotType::Flex).count(),
    );

    println!(
        "Will add {:?}/{:?} destination static/flex slots.",
        static_slot_inserts_count, flex_slot_inserts_count
    );

    if static_slot_inserts_count > free_static.len()
        || flex_slot_inserts_count > free_flex.len()
    {
        eprintln!("Not enough free samples slots in destination project!");
        return Err(Box::new(OctatoolErrors::CliNoFreeSampleSlots));
    }

    let inactive_slot_ops = bank_slot_refs
        .iter()
        .filter(|b| b.reference_type == BankSlotReferenceType::Inactive)
        .sorted_by_key(|x| x.slot_id)
        .map(|x| {
            let new_slot_id = match x.sample_type {
                ProjectSampleSlotType::Static => static_empty_slot,
                ProjectSampleSlotType::Flex => flex_empty_slot,
                _ => 254, // zero-indexed, will switch to 1-indexed before write so will +1
            };

            let src_slot = Slot {
                sample_type: x.sample_type.clone(),
                slot_id: x.slot_id,
                path: PathBuf::from(""),
                // rest of the settings don't matter, we won't be using them
                trim_bars_x100: 0,
                timestrech_mode: SampleAttributeTimestrechMode::Normal,
                loop_mode: SampleAttributeLoopMode::Off,
                trig_quantization_mode: SampleAttributeTrigQuantizationMode::PatternLength,
                gain: 24,
                bpm: 120,
            };

            let dest_slot =
                create_sample_slot_from_existing(&src_slot, None, &new_slot_id).unwrap();

            let op = SampleSlotOperation {
                src_slot,
                dest_slot,
                op_type: SampleSlotOperationType::ReuseSlot,
            };

            op
        })
        .collect::<HashSet<_>>();

    let active_reuse_ops = src_slots_reuses
        .into_iter()
        .sorted_by_key(|x| x.slot_id)
        .map(|x| {
            let src_slot = deduped_src_zero_indexed_slots
                .iter()
                .find(|y| x.slot_id == y.slot_id && x.sample_type == y.sample_type)
                .unwrap(); // TODO: eww

            let new_slot =
                find_sample_slot_settings_match(src_slot, &dest_zero_indexed_slots).unwrap();

            SampleSlotOperation {
                src_slot: src_slot.clone(),
                dest_slot: new_slot.clone(),
                op_type: SampleSlotOperationType::ReuseSlot,
            }
        })
        .collect::<HashSet<_>>();

    let active_insert_ops = src_slots_inserts
        .into_iter()
        .sorted_by_key(|x| x.slot_id)
        .map(|x| {
            let src_slot = deduped_src_zero_indexed_slots
                .iter()
                .find(|y| y.slot_id == x.slot_id && x.sample_type == y.sample_type)
                .unwrap(); // TODO: eww

            let src_path_audio_abs = &src_project_dirpath.join(&src_slot.path);
            let audio_fpath_rel_dest =
                PathBuf::from(&resolve_fname_and_fext_from_path(&src_path_audio_abs).unwrap());
            let dest_slot_id = match src_slot.sample_type {
                SlotType::Static => free_static
                    .pop()
                    .ok_or(Box::new(OctatoolErrors::CliNoFreeSampleSlots)),
                SlotType::Flex => free_flex
                    .pop()
                    .ok_or(Box::new(OctatoolErrors::CliNoFreeSampleSlots)),
                _ => Some(255).ok_or(Box::new(OctatoolErrors::CliNoFreeSampleSlots)),
            }
                .unwrap();

            let dest_slot = create_sample_slot_from_existing(
                src_slot,
                Some(&audio_fpath_rel_dest),
                &dest_slot_id,
            )
                .expect("Unable to create remapped sample slot.");

            SampleSlotOperation {
                src_slot: src_slot.clone(),
                dest_slot,
                op_type: SampleSlotOperationType::NewSlot,
            }
        })
        .collect::<HashSet<_>>();

    // TODO: This seems like a good cut point. "Get required operations" ... but need the zero
    //       indexed & deduplicated project data/bank data to continue... so maybe it isn't...
    //       need to dwell on it a little.

    // the order of changes are important.
    //
    // inactive must change first, as often we are remapping slots 1 -> 8 due to default track setup
    // also, active slot reuse operations can overwrite insert changes
    // the only operation guaranteed to not conflict is RenameCreate, so it has to come last
    println!("Calculating sample slot reassignments for source bank ...");
    apply_slot_changes_to_bank_inplace(&inactive_slot_ops, &mut deduped_bank);
    apply_slot_changes_to_bank_inplace(&active_reuse_ops, &mut deduped_bank);
    apply_slot_changes_to_bank_inplace(&active_insert_ops, &mut deduped_bank);

    let dest_new_zero_indexed_slots = if active_insert_ops.len() > 0 {
        println!("Adding new sample slots to destination project data ...");
        let static_sample_slot_insertions = &active_insert_ops
            .iter()
            .filter(|x| x.op_type == SampleSlotOperationType::NewSlot && x.src_slot.sample_type == SlotType::Static)
            .cloned()
            .map(|x| x.dest_slot)
            .sorted_by(|x, y| Ord::cmp(&x.slot_id, &y.slot_id))
            .collect_vec();

        let flex_sample_slot_insertions = &active_insert_ops
            .iter()
            .filter(|x| x.op_type == SampleSlotOperationType::NewSlot && x.src_slot.sample_type == SlotType::Static)
            .cloned()
            .map(|x| x.dest_slot)
            .sorted_by(|x, y| Ord::cmp(&x.slot_id, &y.slot_id))
            .collect_vec();


        let mut dest_new_zero_indexed_slots = dest_zero_indexed_slots.clone();
        dest_new_zero_indexed_slots.append(&mut static_sample_slot_insertions.clone());
        dest_new_zero_indexed_slots.append(&mut flex_sample_slot_insertions.clone());

        println!(
            "Added {}/{} static/flex sample slots.",
            static_sample_slot_insertions.len(),
            flex_sample_slot_insertions.len(),
        );
        dest_new_zero_indexed_slots
    }
    else {
        println!("No sample slots will be added to destination project.");
        dest_zero_indexed_slots
    };

    let sample_transfers = active_insert_ops
        .iter()
        .filter(|x| x.op_type == SampleSlotOperationType::NewSlot)
        .cloned()
        .map(|x| (x.src_slot, x.dest_slot))
        .map(|x| SlotFileCopy { from: x.0, to: x.1 })
        .collect_vec();

    let mut new_project = dest_project.clone();
    new_project.slots = get_one_indexed_slots_from_zero_indexed(&dest_new_zero_indexed_slots)?;

    Ok((new_project, deduped_bank, sample_transfers))

}


/// Transfer sample files (audio and `.ot` files) related to `NewSlot` operations.
///
/// This function uses the destination sample slot in a `SampleSlotOperation` to get the
/// destination file path.
pub(crate) fn transfer_sample_files(
    transfers: &[SlotFileCopy],
    src_dirpath: &Path,
    dest_dirpath: &Path,
) -> RBoxErr<()> {
    for transfer in transfers {
        let src_path_audio_abs = &src_dirpath.to_path_buf().join(&transfer.from.path);
        let src_ot_filepath_abs = resolve_otfile_fpath_from_audio_fpath(&src_path_audio_abs);

        let dest_filename_and_ext = resolve_fname_and_fext_from_path(&transfer.to.path).expect(
            "Failed to resolve file name and/or extension of audio file destination location.",
        );
        let dest_path_audio_abs = dest_dirpath.join(dest_filename_and_ext);
        let dest_ot_filepath_abs = resolve_otfile_fpath_from_audio_fpath(&dest_path_audio_abs);

        println!(
            "Checking for audio file copy: {:?} -> {:?}",
            src_path_audio_abs, dest_path_audio_abs,
        );
        if dest_path_audio_abs.exists() {
            println!(
                "File exists in destination, skipping copy: {:?} -> {:?}",
                src_path_audio_abs, dest_path_audio_abs,
            );
        } else {
            println!(
                "Transferring audio file: {:?} -> {:?}",
                src_path_audio_abs, dest_path_audio_abs
            );
            std::fs::copy(src_path_audio_abs, dest_path_audio_abs)
                .expect("Failed to copy audio file from source to destination.");
        }

        println!(
            "Checking for OT file copy: {:?} -> {:?}",
            src_ot_filepath_abs, dest_ot_filepath_abs,
        );
        if dest_ot_filepath_abs.exists() {
            println!("File exists in destination, skipping")
        } else if src_ot_filepath_abs.exists() {
            println!(
                "Transferring OT file: {:?} -> {:?}",
                src_ot_filepath_abs, dest_ot_filepath_abs
            );
            std::fs::copy(src_ot_filepath_abs, dest_ot_filepath_abs)
                .expect("Failed to copy OT file from source to destination.");
        } else {
            println!(
                "OT file doesn't exist in source, skipping copy: {:?}",
                src_ot_filepath_abs
            )
        }
    }

    Ok(())
}

