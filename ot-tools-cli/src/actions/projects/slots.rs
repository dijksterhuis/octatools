use crate::{
    actions::banks::utils::{
        create_backup_of_work_file, find_sample_slot_settings_match,
        get_one_indexed_slots_from_zero_indexed, get_zero_indexed_slots_from_one_indexed, BankMeta,
        ProjectMeta, SlotReferenceReassignment,
    },
    actions::{part_update_sample_slot_refs, pattern_update_sample_slot_refs},
    RBoxErr,
};
use itertools::Itertools;
use ot_tools_lib::{
    banks::Bank,
    projects::{slots::ProjectSampleSlot, Project},
    read_type_from_bin_file, write_type_to_bin_file,
};
use std::path::Path;

/// De-duplicate sample slots based on the slot settings, reassigning references
/// to the duplicate slots in the provided bank data.
///
/// WARNINGS:
/// - Does not mutate types, provided new type instances with modifications.
/// - Assumes zero-indexing on the provided array of sample slots, by default
///     these are 1-indexed in the Octatrack data files.
/// - Does not de-duplicate based on sample file content. Sample files with
///     matching file path can be treated as the same file, even if they are
///     different.
fn get_new_deduplicated_sample_slots_and_updated_banks(
    slots: &[ProjectSampleSlot],
    banks: &[Bank],
) -> RBoxErr<(Vec<ProjectSampleSlot>, Vec<Bank>)> {
    let mut deduped = slots
        .iter()
        .cloned()
        .unique_by(|x| {
            (
                // everything except slot id
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
        if !deduped.contains(slot) {
            if let Some(found) = find_sample_slot_settings_match(slot, &deduped) {
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
                pattern_update_sample_slot_refs(
                    p,
                    &reassignment.slot_type,
                    &reassignment.initial_slot_id,
                    &reassignment.new_slot_id,
                )
                .expect("Failed to update sample slot reference in pattern p-locks.");
            });
            bank.parts.unsaved.iter_mut().for_each(|p| {
                part_update_sample_slot_refs(
                    p,
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

// TODO: BankMeta and ProjectMeta need to be changed to load the work and strd file paths

fn load_work_banks_for_project(project_dirpath: &Path) -> RBoxErr<Vec<Bank>> {
    let mut banks: Vec<Bank> = vec![];
    for bank_id in 1..=16 {
        let bank_paths = BankMeta::frompath(project_dirpath, bank_id)?;
        create_backup_of_work_file(&bank_paths.filepath)?;
        banks.push(read_type_from_bin_file::<Bank>(&bank_paths.filepath)?)
    }
    Ok(banks)
}

/// Assumes `banks` ordering is the order in which to write the bank files
fn write_work_banks_for_project(project_dirpath: &Path, banks: &[Bank]) -> RBoxErr<()> {
    for (bank_id, new_bank) in (1..=16).zip(banks) {
        let bank_paths = BankMeta::frompath(project_dirpath, bank_id)?;
        write_type_to_bin_file::<Bank>(new_bank, &bank_paths.filepath)?;
    }
    Ok(())
}

/// Deduplicate sample slots for a project located at `dirpath`, removing
/// duplicates based on slot settings.
///
/// Slot uniqueness is determined by the file path of the registered sample file,
/// gain, tempo, trim length etc.
///
/// The command will also update slot references in all bankXX.work files within
/// the project directory (slot assignments are changed to point at the
/// remaining unique slot).
///
/// WARNING: Does not check whether sample files are unique based on content --
/// requires end-users have been fastidious when naming their sample files.
pub fn cmd_slots_deduplicate(project_dirpath: &Path) -> RBoxErr<()> {
    let project_paths = ProjectMeta::frompath(project_dirpath)?;
    create_backup_of_work_file(&project_paths.filepath)?;
    let project = read_type_from_bin_file::<Project>(&project_paths.filepath)?;
    let banks = load_work_banks_for_project(project_dirpath)?;
    let zero_index_slots = get_zero_indexed_slots_from_one_indexed(&project.slots)?;
    let (deduped_zero_index_slots, new_banks) =
        get_new_deduplicated_sample_slots_and_updated_banks(&zero_index_slots, &banks)?;
    let one_index_slots = get_one_indexed_slots_from_zero_indexed(&deduped_zero_index_slots)?;

    let mut new_project = project.clone();
    new_project.slots = one_index_slots;

    write_type_to_bin_file::<Project>(&new_project, &project_paths.filepath)?;
    write_work_banks_for_project(project_dirpath, &new_banks)?;

    Ok(())
}
