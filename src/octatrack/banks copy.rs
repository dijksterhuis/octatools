//! Reading `bank??.*` files

use crate::octatrack::common::FromFileAtPathBuf;
use bincode;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;
use std::collections::HashMap;
use std::fmt;
use std::ops::Range;
use std::{default, error::Error, fs::File, io::Read, path::PathBuf};

use crate::common::{RBoxErr, RVoidError};

/// A step on the sequencer -- note that Off is a specific option here.
#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
enum TrigType {
    /// No trig enabled for this step
    #[default]
    Off,

    /// Trigger trig type
    Trigger,

    /// 'Hold' trig type
    Hold,

    /// 'Envelope' (?) trig type
    Envelope,

    /// 'P-Lock' trig type
    Parameter,
}

/// The type of machine currently active on a track.
/// **TODO**: All fields for each machine type?

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum TrackMachineType {
    /// Static machine
    StaticMachine { sample_slot: u8 },

    /// Flex machine
    FlexMachine { sample_slot: u8 },

    /// Thru (external record in) machine
    Thru,

    /// Neighbor machine.
    /// **NOTE**: cannot be active on track 1.
    Neighbor,

    /// Pickup machine.
    /// **NOTE**: First pickup machine is always master for tempo sync.
    Pickup,
}

use crate::octatrack::common::OptionEnumValueConvert;

impl OptionEnumValueConvert for TrackMachineType {
    type T = TrackMachineType;
    type V = u8;

    fn from_value(_v: Self::V) -> RVoidError<Self::T> {
        // requires raw data to get the sample slot for sample machines
        unimplemented!();
    }

    fn value(&self) -> RVoidError<Self::V> {
        match self {
            TrackMachineType::StaticMachine { sample_slot: _ } => Ok(0),
            TrackMachineType::FlexMachine { sample_slot: _ } => Ok(1),
            TrackMachineType::Thru => Ok(2),
            TrackMachineType::Neighbor => Ok(3),
            TrackMachineType::Pickup => Ok(4),
        }
    }
}

/// Bare bones Audio track data.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct AudioTrack {
    pub machine: TrackMachineType,
    //todo -- other settings
}

impl AudioTrack {
    fn from_byte_data(bytes: &Vec<u8>) -> ! {
        todo!()
    }
}

/// Bare bones MIDI track data.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
struct MidiTrack {
    channel: u8,
    //todo -- other settings
}

/// A specific step's trig.
/// Currently limited to sample slot assignmnets as that's all I care about for now.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Trig {
    idx: u8,
    trig_type: TrigType,
    // TODO: this need to be able to accept "TRACK DEFAULT" as an option?!
    //     --> TRACK DEFAULT is 255!
    // todo: recording buffers on flex machines
    pub sample_slot: u8,
    //todo -- other settings
}

/// A specific step's trig.
/// Currently limited to sample slot assignmnets as that's all I care about for now.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct TrackTrigs {
    #[serde(with = "BigArray")]
    pub trigs: [Trig; 64],
}

/// A pattern of trigs stored in the bank.

// TODO: This needs to be aware of tracks ... !!!!!!!!
// a pattern is a collection of trigs for each of the tracks

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Pattern {
    /// Trigs per each step.
    /// In the bank file these are masked values which need to be decoded into a 64 length array.
    #[serde(with = "BigArray")]
    pub track_trigs: [TrackTrigs; 8],
    // trigs: [Trig; 64],
    /// Number of trigs
    length: u8,
    //todo -- other settings
}

impl Pattern {
    fn from_byte_data(bytes: &Vec<u8>) -> Result<Self, ()> {
        // TODO!!! PER TRACK?!

        let default_trig = Trig {
            idx: 255,
            trig_type: TrigType::Trigger,
            sample_slot: 0,
        };

        // trig sample slots assignments start with Trig 1 at byte position 159
        // then they are every 32 bytes for each subsequent trig.
        // FOR ONE TRACK!

        let track_start_offset: usize = 9;
        // let track_data_len: usize = 2368;
        let track_data_len: usize = 2338;

        println!("BYTES LEN {:#?}", bytes.len());

        let _trac_start_bytes = [84, 82, 65, 67];

        // let trig_slot_offset: usize = 127;
        let trig_slot_offset: usize = 97;

        let default_trigs: TrackTrigs = TrackTrigs {
            trigs: [default_trig; 64],
        };
        let mut pattern_trigs: [TrackTrigs; 8] = [default_trigs; 8];

        for audio_track_idx in 0..=7 {
            let track_data_start = track_start_offset + (audio_track_idx * track_data_len);
            let track_data_end = track_data_start + track_data_len;
            let track_data = &bytes[track_data_start..track_data_end].to_vec();

            let mut trigs = TrackTrigs {
                trigs: [default_trig; 64],
            };
            // TODO !!!: Only use specific track pattern byte vector here.

            for trig_idx in 1..=64_usize {
                let trig_slot_curr_byte_index: usize = trig_slot_offset + (trig_idx * 32);

                let trig = Trig {
                    idx: trig_idx as u8,
                    trig_type: TrigType::Trigger,
                    sample_slot: track_data[trig_slot_curr_byte_index],
                };

                trigs.trigs[trig_idx - 1] = trig;
            }

            pattern_trigs[audio_track_idx] = trigs;
        }

        let pattern = Pattern {
            track_trigs: pattern_trigs,
            // dummy value for now.
            length: 0,
        };

        Ok(pattern)
    }
}

// TODO: For some reaosn there are EIGHT part sections in the data file...
// I do not know why ... previous states?

/// Parts in the bank, containing track data.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Part {
    /// Parts contain the parameter data for audio and midi tracks.
    #[serde(with = "BigArray")]
    pub audio_tracks: [AudioTrack; 8],

    /// Parts contain the parameter data for audio and midi tracks.
    #[serde(with = "BigArray")]
    midi_tracks: [MidiTrack; 8],
}

fn get_audio_track_data_from_bytes(bytes: &Vec<u8>, track_idx: usize) -> AudioTrack {
    let machine_type_offset = 42;
    let static_slot_id_offset = 723;

    // the machine type bytes are stored sequentially per track
    let machine_type_byte = bytes[machine_type_offset + track_idx];

    // slot ids from flex and static are stored separately.
    // static first, then flex the byte immediately after.

    // WARN: default tracks are always Static Slot 0.
    //       So Static Slot 0 will always be returned here.
    //       I can't see an effective way around this yet.
    let static_slot_id = bytes[static_slot_id_offset];
    let flex_slot_id = bytes[static_slot_id_offset + 1];

    println!(
        "MACHINE TYPE BYTE: {:#?}",
        bytes[(machine_type_offset + track_idx)..((machine_type_offset + track_idx) + 16)].to_vec()
    );

    let machine = match machine_type_byte {
        0 => TrackMachineType::StaticMachine {
            sample_slot: static_slot_id + 1,
        },
        1 => TrackMachineType::FlexMachine {
            sample_slot: flex_slot_id + 1,
        },
        2 => TrackMachineType::Thru {},
        3 => TrackMachineType::Neighbor {},
        4 => TrackMachineType::Pickup {},
        _ => {
            panic!()
        }
    };

    let audio_track = AudioTrack { machine };

    audio_track
}

impl Part {
    fn from_byte_data(bytes: &Vec<u8>) -> Result<Self, ()> {
        let audio_tracks: [AudioTrack; 8] =
            core::array::from_fn(|i| i + 1).map(|v| get_audio_track_data_from_bytes(&bytes, v));

        let midi_tracks: [MidiTrack; 8] =
            core::array::from_fn(|i| i + 1).map(|_| MidiTrack { channel: 0 });

        let part = Self {
            audio_tracks,
            midi_tracks,
        };

        Ok(part)
    }
}

/// A Bank.
/// **NOTE**: this only gets data releveant to sample slots at the moment.
/// Anything else is currently out of scope, but the main building blocks
/// are here to expand in future.

#[derive(Serialize, Deserialize)]
pub struct Bank {
    /// All patterns in a bank
    #[serde(with = "BigArray")]
    pub patterns: [Pattern; 16],

    /// The different parts, always four of them.
    #[serde(with = "BigArray")]
    pub parts: [Part; 4],

    /// The different parts, always four of them.
    pub part_names: Vec<String>,

    /// Whether parts have been saved or not?!
    /// Need to check what the last value in the bank file is.
    /// It looks like a mask for which parts are edited or not and not yet saved.
    pub parts_saved: [bool; 4],

    bytes: Vec<u8>,
}

impl Bank {
    // This method and all dependent remapping method should mutate data
    pub fn change_slot_data(self, patterns: [Pattern; 16], parts: [Part; 4]) -> Result<Self, ()> {
        let part_names = self.part_names;
        let parts_saved = self.parts_saved;
        let bytes = self.bytes;

        let start_pattern_byte_idx: usize = 21;
        let pattern_data_len: usize = 36588;

        let x: Vec<u8> = (0..=15).map_into().collect();

        let y: Vec<Pattern> = x.into_iter().map(|v| patterns[v as usize]).collect();

        for pattern_idx in 0..=15 {
            // get the specific pattern data

            let c = start_pattern_byte_idx + (pattern_idx * pattern_data_len);
            let pattern_data = bytes[c..(c + pattern_data_len)].to_vec();
        }

        Ok(Self {
            patterns,
            parts,
            part_names,
            parts_saved,
            bytes,
        })
    }
}

impl fmt::Debug for Bank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        #[derive(Debug)]
        struct Bank<'a> {
            pub patterns: &'a [Pattern; 16],
            pub parts: &'a [Part; 4],
            pub part_names: &'a Vec<String>,
            pub parts_saved: &'a [bool; 4],
        }

        let Self {
            patterns,
            parts,
            part_names,
            parts_saved,
            bytes: _,
        } = self;

        // per Chayim Friedman’s suggestion
        fmt::Debug::fmt(
            &Bank {
                patterns,
                parts,
                part_names,
                parts_saved,
            },
            f,
        )
    }
}

impl FromFileAtPathBuf for Bank {
    type T = Bank;

    /// Crete a new struct by reading a file located at `path`.
    fn from_pathbuf(path: PathBuf) -> Result<Self::T, Box<dyn Error>> {
        let mut infile = File::open(path)?;
        let mut bytes: Vec<u8> = vec![];
        let _: usize = infile.read_to_end(&mut bytes)?;

        // ///////////////////////////////////////////////////////////////////
        // TRIG SAMPLE LOCKS

        let default_trig = Trig {
            idx: 255,
            trig_type: TrigType::Off,
            sample_slot: 0,
        };

        let default_track_trigs = TrackTrigs {
            trigs: [default_trig; 64],
        };

        let default_pattern = Pattern {
            track_trigs: [default_track_trigs; 8],
            length: 64,
        };

        let start_pattern_byte_idx: usize = 21;
        let pattern_data_len: usize = 36588;

        let mut patterns = [default_pattern; 16];
        for pattern_idx in 0..=15 {
            // get the specific pattern data

            let c = start_pattern_byte_idx + (pattern_idx * pattern_data_len);
            let pattern_data = bytes[c..(c + pattern_data_len)].to_vec();

            patterns[pattern_idx] = Pattern::from_byte_data(&pattern_data).unwrap();
        }

        // ///////////////////////////////////////////////////////////////////
        // AUDIO TRACK MACHINE SAMPLE SLOTS

        let default_audio_track = AudioTrack {
            machine: TrackMachineType::StaticMachine {
                sample_slot: u8::MAX,
            },
        };
        let default_midi_track = MidiTrack { channel: 0 };
        let default_part = Part {
            audio_tracks: [default_audio_track; 8],
            midi_tracks: [default_midi_track; 8],
        };

        // TODO: this might need to shift depending on what that first `@` character
        // is used for... looks like it's a boolean for whether the part is saved or not.
        // tracks / patterns can't be 'svaed' on the octatrack... that's an A4 thing.

        let start_part_byte_idx: usize = 585430;
        let offset_part_byte_idx: usize = 6331;

        let mut parts = [default_part; 4];
        for part_idx in 0..=3 {
            // get the specific part data
            let c = start_part_byte_idx + (part_idx * offset_part_byte_idx);
            let part_data = bytes[c..(c + offset_part_byte_idx)].to_vec();

            parts[part_idx] = Part::from_byte_data(&part_data).unwrap();
        }

        let mut part_names: Vec<String> = Vec::with_capacity(4);

        for i in 0..=3 {
            let name_idx = 636083 + (i * 7);

            // TODO: optimise this
            let name = bytes[name_idx..name_idx + 6]
                .to_vec()
                .escape_ascii()
                .to_string()
                .split("\\")
                .take(1)
                .collect();
            part_names.push(name);
        }

        let bank = Bank {
            patterns,
            parts,
            part_names,
            parts_saved: [true; 4],
            bytes,
        };

        Ok(bank)
    }
}





/// Transfer a bank from one project to another project

pub fn transfer_bank(
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

    let dst_dirpath = &dest_bank_file_path.parent().unwrap().to_path_buf();
    let dest_project = Project::from_pathbuf(dst_proj_path).unwrap();

    // 4. find space in new project sample slots

    let mut base_vec: Vec<u8> = vec![];
    for i in 1..=128 {
        base_vec.push(i)
    }
    let mut dest_free_static_sample_slots_ids = base_vec.clone();
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

    // for i in 0..127_u8 {
    //     if !&dest_sample_slot_ids.contains(&i) {
    //         dest_free_sample_slots_ids.push(i);
    //     }
    // }

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

    // 5. read src bank data
    //  * machine assigned sample slots
    //  * trig sample lock assigned sample slots
    let src_bank_data = Bank::from_pathbuf(source_bank_file_path).unwrap();

    let mut active_static_slots: HashSet<u8> = HashSet::new();
    let mut active_flex_slots: HashSet<u8> = HashSet::new();

    for pattern in src_bank_data.patterns {
        for (_idx, track_trigs) in pattern.track_trigs.into_iter().enumerate() {
            for trig in track_trigs.trigs {
                if trig.sample_slot < 128 {
                    // When tracks have a Trig Sample Lock the sample lock does not
                    // care about flex / static. The sample locked trig will trigger
                    // whatever sample is in the sample slot indicated by the trig lock.
                    //
                    // so we have to assume that BOTH flex & static sample slots can be
                    // used by trig sample locks

                    active_static_slots.insert(trig.sample_slot);
                    active_flex_slots.insert(trig.sample_slot);
                }
            }
        }
    }

    if active_static_slots.len() > 0 {
        warn!(
            "Detected Trig sample locks. Assuming both Flex and Static slots can be used (Part switching while Pattern playing)."
        );
    }

    for part in src_bank_data.parts {
        for audio_track in part.audio_tracks {
            match audio_track.machine {
                TrackMachineType::StaticMachine { sample_slot } => {
                    active_static_slots.insert(sample_slot);
                }
                TrackMachineType::FlexMachine { sample_slot } => {
                    active_flex_slots.insert(sample_slot);
                }
                _ => {}
            }
        }
    }

    println!("SOURCE BANK DATA: {:#?}", src_bank_data.parts);

    println!(
        "SOURCE STATIC SAMPLE SLOTS IN USE: {:#?}",
        active_static_slots
    );
    println!("SOURCE FLEX SAMPLE SLOTS IN USE: {:#?}", active_flex_slots);
    println!(
        "DEST STATIC SAMPLE SLOTS FREE: {:#?}",
        dest_free_static_sample_slots_ids
    );
    println!(
        "DEST FLEX SAMPLE SLOTS FREE: {:#?}",
        dest_free_flex_sample_slots_ids
    );

    // exit(1);

    // 6. edit read bank data sample slot usage
    // this is just a creating a mapping from old to new.

    let mut source_to_dest_static_slot_map: HashMap<u8, u8> = HashMap::new();
    let mut source_to_dest_flex_slot_map: HashMap<u8, u8> = HashMap::new();

    // reverse so we can just use pop instead of needing to import VecDeque::pop_rev()
    dest_free_static_sample_slots_ids.reverse();
    dest_free_flex_sample_slots_ids.reverse();

    for active_static_slot in active_static_slots {
        let dest_slot_id = dest_free_static_sample_slots_ids.pop().unwrap();
        source_to_dest_static_slot_map.insert(active_static_slot, dest_slot_id);
    }

    for active_flex_slot in active_flex_slots {
        let dest_slot_id = dest_free_flex_sample_slots_ids.pop().unwrap();
        source_to_dest_flex_slot_map.insert(active_flex_slot, dest_slot_id);
    }

    // first, change the struct data so we've got everything correct.

    // TODO: Does this actually **mutate** the bank data
    // or does it just mutate the iterator output in for scope?

    for (k, v) in source_to_dest_static_slot_map.iter() {
        for pattern in src_bank_data.patterns {
            for track_trigs in pattern.track_trigs {
                for mut trig in track_trigs.trigs {
                    if trig.sample_slot == *k {
                        trig.sample_slot = v.clone();
                    }
                }
            }
        }
        for part in src_bank_data.parts {
            for mut audio_track in part.audio_tracks {
                match audio_track.machine {
                    TrackMachineType::StaticMachine { sample_slot } => {
                        audio_track.machine = TrackMachineType::StaticMachine {
                            sample_slot: source_to_dest_static_slot_map
                                .get(&sample_slot)
                                .unwrap()
                                .clone(),
                        };
                    }
                    _ => {}
                }
            }
        }
    }

    for (k, v) in source_to_dest_flex_slot_map.iter() {
        for pattern in src_bank_data.patterns {
            for track_trigs in pattern.track_trigs {
                for mut trig in track_trigs.trigs {
                    if trig.sample_slot == *k {
                        trig.sample_slot = v.clone();
                    }
                }
            }
        }
        for part in src_bank_data.parts {
            for mut audio_track in part.audio_tracks {
                match audio_track.machine {
                    TrackMachineType::FlexMachine { sample_slot } => {
                        audio_track.machine = TrackMachineType::FlexMachine {
                            sample_slot: source_to_dest_flex_slot_map
                                .get(&sample_slot)
                                .unwrap()
                                .clone(),
                        };
                    }
                    _ => {}
                }
            }
        }
    }

    println!("CHANGED SOURCE BANK DATA: {:#?}", src_bank_data.parts);

    // now change the actual bank bytes data

    // NOTE: this would be a lot easier with bank files Serde mapped out fully,
    //       but that's a massive undertaking I'm not super keen on today.
    //       So, we're going with messy, but works, in the first instance!

    // 7. edit read bank data sample slots
    //  *  machine assignment
    //  *  trig smaple lock assignment

    // let sample_slots = todo!();
    // let trig_sample_locks = todo!();
    // let sample_slots_active_machines = todo!();

    // todo!();

    // // Copy bank file
    // copy(src_dirpath, dst_dirpath);

    // // TODO: move them

    Ok(())
}