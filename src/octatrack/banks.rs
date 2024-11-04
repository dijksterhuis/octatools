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
