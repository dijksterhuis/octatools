//! Reading `bank??.*` files

use crate::octatrack::common::FromFileAtPathBuf;
use bincode;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;
use std::{error::Error, fs::File, io::Read, path::PathBuf};
use symphonia::core::formats::Track;

/// A step on the sequencer -- note that Off is a specific option here.
#[derive(Default, Serialize, Deserialize, Clone, Copy)]
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

#[derive(Serialize, Deserialize, Clone, Copy)]
enum TrackMachineType {
    /// Static machine
    StaticMachine { sample_slot: u16 },

    /// Flex machine
    FlexMachine { sample_slot: u16 },

    /// Thru (external record in) machine
    Thru,

    /// Neighbor machine.
    /// **NOTE**: cannot be active on track 1.
    Neighbor,

    /// Pickup machine.
    /// **NOTE**: First pickup machine is always master for tempo sync.
    Pickup,
}

/// Bare bones Audio track data.
#[derive(Serialize, Deserialize, Clone, Copy)]
struct AudioTrack {
    machine: TrackMachineType,
}

/// Bare bones MIDI track data.
#[derive(Serialize, Deserialize, Clone, Copy)]
struct MidiTrack {
    channel: u8,
}

/// A specific step's trig.
/// Currently limited to sample slot assignmnets as that's all I care about for now.
#[derive(Serialize, Deserialize, Clone, Copy)]
struct Trig {
    trig_type: TrigType,
    sample_slot: u16,
}

/// A pattern of trigs stored in the bank.
#[derive(Serialize, Deserialize, Clone, Copy)]
struct Pattern {
    /// Trigs per each step.
    /// In the bank file these are masked values which need to be decoded into a 64 length array.
    #[serde(with = "BigArray")]
    trigs: [Trig; 64],

    /// Number of trigs
    length: u8,
}

// TODO: For some reaosn there are EIGHT part sections in the data file...
// I do not know why ... previous states?

/// Parts in the bank, containing track data.
#[derive(Serialize, Deserialize, Clone, Copy)]
struct Part {
    /// Parts contain the parameter data for audio and midi tracks.
    #[serde(with = "BigArray")]
    audio_tracks: [AudioTrack; 8],

    /// Parts contain the parameter data for audio and midi tracks.
    #[serde(with = "BigArray")]
    midi_tracks: [MidiTrack; 8],
}

/// A Bank.
/// **NOTE**: this only gets data releveant to sample slots at the moment.
/// Anything else is currently out of scope, but the main building blocks
/// are here to expand in future.

#[derive(Serialize, Deserialize)]
pub struct Bank {
    /// All patterns in a bank
    #[serde(with = "BigArray")]
    patterns: [Pattern; 16],

    /// The different parts, always four of them.
    #[serde(with = "BigArray")]
    parts: [Part; 4],

    /// Whether parts have been saved or not?!
    /// Need to check what the last value in the bank file is.
    /// It looks like a mask for which parts are edited or not and not yet saved.
    parts_saved: [bool; 4],
}

impl FromFileAtPathBuf for Bank {
    type T = Bank;

    /// Crete a new struct by reading a file located at `path`.
    fn from_pathbuf(path: PathBuf) -> Result<Self::T, Box<dyn Error>> {
        let mut infile = File::open(path)?;
        let mut bytes: Vec<u8> = vec![];
        let _: usize = infile.read_to_end(&mut bytes)?;

        // for each pattern (16)

        let default_trig = Trig {
            trig_type: TrigType::Off,
            sample_slot: 0,
        };

        let default_pattern = Pattern {
            trigs: [default_trig; 64],
            length: 64,
        };

        // done iter

        // for each part (4)
        // for each AUDIO track (8)

        let default_machine = TrackMachineType::StaticMachine { sample_slot: 0 };

        let default_audio_track = AudioTrack {
            machine: default_machine,
        };

        // done audio iter

        let default_midi_track = MidiTrack { channel: 0 };

        // done midi iter (8)
        // done part iter

        let default_part = Part {
            audio_tracks: [default_audio_track; 8],
            midi_tracks: [default_midi_track; 8],
        };

        //

        let bank = Bank {
            patterns: [default_pattern; 16],
            parts: [default_part; 4],
            parts_saved: [true; 4],
        };

        Ok(bank)
    }
}
