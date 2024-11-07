//! CLI actions related to copying Octatrack entities

use log::{debug, error, info, warn};
use std::error::Error;
use std::path::PathBuf;
use std::collections::{HashMap, HashSet};

use serde_octatrack::{
    banks::Bank,
    banks::parts::Part,
    banks::patterns::Pattern,
    common::{FromFileAtPathBuf, RBoxErr, ToFileAtPathBuf},
    projects::{slots::ProjectSampleSlots, options::ProjectSampleSlotType, Project},
};

/// Transfer a bank from one project to another project

pub fn copy_bank(
    source_bank_file_path: PathBuf,
    dest_bank_file_path: PathBuf,
    merge_duplicate_sample_slots: bool,
) -> Result<(), ()> {
    // === take sample slots and copy them to new slots in new project ===
    // ===================================================================
    //
    // 1. read old project
    // 2. get sample slots
    // 3. read new project
    // 4. find space in new project sample slots
    // 5. read src bank data
    //  * machine assigned sample slots
    //  * trig sample lock assigned sample slots
    // 6. edit read bank data sample slot usage
    // 7. edit read bank data sample slots
    //  *  machine assignment
    //  *  trig smaple lock assignment
    // 8. create backup files
    //  * new project
    //  * new bank file
    // 9. copy samples to new project folder
    //  * todo: add a .txt log file detailing copied files?
    // 10. add samples to project sample slots
    // 11. write over project file
    // 11. write new bank data over old bank

    // 1. read old project

    let src_proj_path = source_bank_file_path
        .parent()
        .unwrap()
        .to_path_buf()
        .join("project.work");

    let src_dirpath = &src_proj_path.parent().unwrap().to_path_buf();
    let src_project = Project::from_pathbuf(src_proj_path).unwrap();

    // 2. get sample slots
    let src_sample_slots: Vec<ProjectSampleSlots> = src_project
        .slots
        .into_iter()
        .filter(|x| x.slot_id < 128) // no recording buffers
        .collect();

    // 3. read new project
    let dst_proj_path = dest_bank_file_path
        .parent()
        .unwrap()
        .to_path_buf()
        .join("project.work");

    println!("PROJECT PATH: {dst_proj_path:#?}");

    let dst_dirpath = &dest_bank_file_path.parent().unwrap().to_path_buf();
    let dest_project = Project::from_pathbuf(dst_proj_path).unwrap();

    // 4. find space in new project sample slots

    let mut base_vec: Vec<u8> = vec![];
    for i in 1..=128 {
        base_vec.push(i)
    }
    let mut dest_free_static_sample_slots_ids: Vec<u8> = base_vec.clone();
    let mut dest_free_flex_sample_slots_ids: Vec<u8> = base_vec.clone();

    println!("DEST SLOT USAGE: {:#?}", dest_project.slots);

    for slot in dest_project.slots {
        match slot.sample_type {
            ProjectSampleSlotType::Static => {
                dest_free_static_sample_slots_ids.retain(|x| *x != slot.slot_id as u8);
            }
            ProjectSampleSlotType::Flex => {
                dest_free_flex_sample_slots_ids.retain(|x| *x != slot.slot_id as u8);
            }
            _ => {}
        }
    }


    // not enough sample slots -- clean up slot allocations please.
    if src_sample_slots.len()
        > (dest_free_static_sample_slots_ids.len() + dest_free_flex_sample_slots_ids.len())
    {
        panic!(
            "Not enough spare sample slots in destination project! srcSlotCount={:#?} destSlotCount={:#?}",
            src_sample_slots.len(),
            dest_free_static_sample_slots_ids.len() + dest_free_flex_sample_slots_ids.len(),
        );
    }

    println!("DEBUG");

    // 5. read src bank data
    //  * machine assigned sample slots
    //  * trig sample lock assigned sample slots
    println!("DEBUG");
    let mut src_bank_data = Bank::from_pathbuf(source_bank_file_path).unwrap();

    let mut active_static_slots: HashSet<u8> = HashSet::new();
    let mut active_flex_slots: HashSet<u8> = HashSet::new();

    // for pattern in src_bank_data.patterns.iter() {
    //     for audio_track_trigs in pattern.audio_track_trigs.iter() {
    //         for plock in audio_track_trigs.plocks.iter() {

    //             // TODO
    //             let sample_slots = plock.unknown;

    //             if sample_slots[0] < 128 {
    //                 active_static_slots.insert(sample_slots[0]);
    //                 active_flex_slots.insert(sample_slots[0]);
    //             }
    //         }
    //     }
    // }

    // if active_static_slots.len() > 0 {
    //     warn!("Detected Trig sample locks.");
    //     warn!("Assuming both Flex and Static slots can be used (Part switching while Pattern playing).")
    // }

    // for (_idx, part) in src_bank_data.parts.iter().enumerate() {
    //     for audio_track_slots in part.audio_track_machine_slots.iter() {
    //         active_static_slots.insert(audio_track_slots.static_slot_id);
    //         active_flex_slots.insert(audio_track_slots.flex_slot_id);
    //     }
    // }

    // let mut source_to_dest_static_slot_map: HashMap<u8, u8> = HashMap::new();
    // let mut source_to_dest_flex_slot_map: HashMap<u8, u8> = HashMap::new();

    // // reverse so we can just use pop instead of needing to import VecDeque::pop_rev()
    // dest_free_static_sample_slots_ids.reverse();
    // dest_free_flex_sample_slots_ids.reverse();

    // for active_static_slot in active_static_slots {
    //     let dest_slot_id = dest_free_static_sample_slots_ids.pop().unwrap();
    //     source_to_dest_static_slot_map.insert(active_static_slot, dest_slot_id);
    // }

    // for active_flex_slot in active_flex_slots {
    //     let dest_slot_id = dest_free_flex_sample_slots_ids.pop().unwrap();
    //     source_to_dest_flex_slot_map.insert(active_flex_slot, dest_slot_id);
    // }

    // modify static and flex sample slots for Part data
    // for (_idx, mut part) in src_bank_data.parts.into_iter().into_iter().enumerate() {
    //     for (slot_idx, audio_track_slots) in part.audio_track_machine_slots.into_iter().enumerate() {
    //         for (k, v) in source_to_dest_static_slot_map.iter() {
    //             if audio_track_slots.static_slot_id == *k {
    //                 part.audio_track_machine_slots[slot_idx].static_slot_id = *v;
    //             }
    //         }
    //         for (k, v) in source_to_dest_flex_slot_map.iter() {
    //             if audio_track_slots.flex_slot_id == *k {
    //                 part.audio_track_machine_slots[slot_idx].flex_slot_id = *v;
    //             }
    //         }
    //     }    
    // }

    // modify static and flex sample slots for Pattern data
    // for mut pattern in src_bank_data.patterns.iter_mut() {
    //     for (track_idx, mut audio_track_trigs) in pattern.audio_track_trigs.iter_mut().enumerate() {
    //         for (plock_idx, plock) in audio_track_trigs.plocks.iter_mut().enumerate() {
    //             for (k, v) in source_to_dest_static_slot_map.iter() {
    //                 if plock.unknown[0] == *k {
    //                     pattern.audio_track_trigs[track_idx].plocks[plock_idx].unknown[0] = *v;
    //                 }
    //             }
    //             for (k, v) in source_to_dest_flex_slot_map.iter() {
    //                 if plock.unknown[0] == *k {
    //                     pattern.audio_track_trigs[track_idx].plocks[plock_idx].unknown[0] = *v;
    //                 }
    //             }
    //         }
    //     }
    // }

    // copy audio files over
    // std::fs::copy();

    // assign new sample slots in project -- requires Project to be serializable!
    // dest_project.slots.

    // copy the bank data
    // let _ = src_bank_data.to_pathbuf(dest_bank_file_path);

    Ok(())
}


