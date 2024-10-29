//! Current Trig mode setting for MIDI tracks.
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error};

use crate::octatrack::common::{FromHashMap, ParseHashMapValueAs};

/// Current Trig mode setting for MIDI tracks.

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct MidiTrackTrigModes {
    // helpfully these are all just called TRIG_MODE_MIDI
    // but there's 8 of them so they must refer to the channels somehow
    // all default to 0.
    // refers to whether MIDI track is on TRACK or CHRMOATIC trig mode.
    // should be u8 as *possible* for more trig modes to be added (although unlikely)
    /// Trig mode for MIDI Track 1
    trig_mode_midi_track_1: u8,

    /// Trig mode for MIDI Track 2
    trig_mode_midi_track_2: u8,

    /// Trig mode for MIDI Track 3
    trig_mode_midi_track_3: u8,

    /// Trig mode for MIDI Track 4
    trig_mode_midi_track_4: u8,

    /// Trig mode for MIDI Track 5
    trig_mode_midi_track_5: u8,

    /// Trig mode for MIDI Track 6
    trig_mode_midi_track_6: u8,

    /// Trig mode for MIDI Track 7
    trig_mode_midi_track_7: u8,

    /// Trig mode for MIDI Track 8
    trig_mode_midi_track_8: u8,
}

impl ParseHashMapValueAs for MidiTrackTrigModes {}

impl FromHashMap for MidiTrackTrigModes {
    type A = String;
    type B = String;
    type T = MidiTrackTrigModes;

    fn from_hashmap(hmap: &HashMap<Self::A, Self::B>) -> Result<Self::T, Box<dyn Error>> {
        Ok(Self {
            trig_mode_midi_track_1: Self::parse_hashmap_value::<u8>(
                &hmap,
                "trig_mode_midi_track_1",
            )?,
            trig_mode_midi_track_2: Self::parse_hashmap_value::<u8>(
                &hmap,
                "trig_mode_midi_track_2",
            )?,
            trig_mode_midi_track_3: Self::parse_hashmap_value::<u8>(
                &hmap,
                "trig_mode_midi_track_3",
            )?,
            trig_mode_midi_track_4: Self::parse_hashmap_value::<u8>(
                &hmap,
                "trig_mode_midi_track_4",
            )?,
            trig_mode_midi_track_5: Self::parse_hashmap_value::<u8>(
                &hmap,
                "trig_mode_midi_track_5",
            )?,
            trig_mode_midi_track_6: Self::parse_hashmap_value::<u8>(
                &hmap,
                "trig_mode_midi_track_6",
            )?,
            trig_mode_midi_track_7: Self::parse_hashmap_value::<u8>(
                &hmap,
                "trig_mode_midi_track_7",
            )?,
            trig_mode_midi_track_8: Self::parse_hashmap_value::<u8>(
                &hmap,
                "trig_mode_midi_track_8",
            )?,
        })
    }
}
