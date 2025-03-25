//! Module containing code related to running commands

use crate::RBoxErr;
use ot_tools_io::banks::parts::Part;
use ot_tools_io::banks::patterns::Pattern;
use ot_tools_io::projects::options::ProjectSampleSlotType;

pub mod arrangements;
pub mod banks;
pub mod drive;
pub mod parts;
pub mod patterns;
pub mod projects;
pub mod samples;

// TODO: T needs a Where and for a trait or something with a `data()` method to ensure the generic
//       function definition knows we can get the `data` field from it
// /// Show raw bytes representation of a binary data file of type `T` at `path`
// fn show_type_bytes<T>(
//     path: &PathBuf,
//     start_idx: &Option<usize>,
//     len: &Option<usize>,
// ) -> RBoxErr<()> {
//
//     let bytes = read_bin_file(path)?;
//
//     let raw_data = deserialize_bin_to_type::<T>(&bytes)?;
//
//
//     let bytes = get_bytes_slice(
//         raw_data.data.to_vec(),
//         start_idx,
//         len,
//     );
//     println!("{:#?}", bytes);
//     Ok(())
// }
//

pub fn pattern_update_sample_slot_refs(
    pattern: &mut Pattern,
    sample_type: &ProjectSampleSlotType,
    old: &u8,
    new: &u8,
) -> RBoxErr<()> {
    for audio_track_trigs in pattern.audio_track_trigs.iter_mut() {
        for plock in audio_track_trigs.plocks.iter_mut() {
            match sample_type {
                ProjectSampleSlotType::Static => {
                    if plock.static_slot_id == *old {
                        plock.static_slot_id = *new;
                    }
                }
                ProjectSampleSlotType::Flex => {
                    if plock.flex_slot_id == *old {
                        plock.flex_slot_id = *new;
                    }
                }
                ProjectSampleSlotType::RecorderBuffer => {}
            }
        }
    }
    Ok(())
}

pub fn part_update_sample_slot_refs(
    part: &mut Part,
    sample_type: &ProjectSampleSlotType,
    old: &u8,
    new: &u8,
) -> RBoxErr<()> {
    for audio_track_slots in part.audio_track_machine_slots.iter_mut() {
        match sample_type {
            ProjectSampleSlotType::Static => {
                if audio_track_slots.static_slot_id == *old {
                    audio_track_slots.static_slot_id = *new;
                }
            }
            ProjectSampleSlotType::Flex => {
                if audio_track_slots.flex_slot_id == *old {
                    audio_track_slots.flex_slot_id = *new;
                }
            }
            ProjectSampleSlotType::RecorderBuffer => {}
        }
    }

    Ok(())
}
