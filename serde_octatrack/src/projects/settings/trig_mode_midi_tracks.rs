//! Current Trig mode setting for MIDI tracks.
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error};

use crate::projects::{parse_hashmap_string_value, FromHashMap};

/// Current Trig mode setting for MIDI tracks.

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct MidiTrackTrigModes {
    // helpfully these are all just called TRIG_MODE_MIDI
    // but there's 8 of them so they must refer to the channels somehow
    // all default to 0.
    // refers to whether MIDI track is on TRACK or CHRMOATIC trig mode.
    // should be u8 as *possible* for more trig modes to be added (although unlikely)
    /// Trig mode for MIDI Track 1
    pub trig_mode_midi_track_1: u8,

    /// Trig mode for MIDI Track 2
    pub trig_mode_midi_track_2: u8,

    /// Trig mode for MIDI Track 3
    pub trig_mode_midi_track_3: u8,

    /// Trig mode for MIDI Track 4
    pub trig_mode_midi_track_4: u8,

    /// Trig mode for MIDI Track 5
    pub trig_mode_midi_track_5: u8,

    /// Trig mode for MIDI Track 6
    pub trig_mode_midi_track_6: u8,

    /// Trig mode for MIDI Track 7
    pub trig_mode_midi_track_7: u8,

    /// Trig mode for MIDI Track 8
    pub trig_mode_midi_track_8: u8,
}

impl FromHashMap for MidiTrackTrigModes {
    type A = String;
    type B = String;
    type T = MidiTrackTrigModes;

    fn from_hashmap(hmap: &HashMap<Self::A, Self::B>) -> Result<Self::T, Box<dyn Error>> {
        Ok(Self {
            trig_mode_midi_track_1: parse_hashmap_string_value::<u8>(
                hmap,
                "trig_mode_midi_track_1",
                None,
            )?,
            trig_mode_midi_track_2: parse_hashmap_string_value::<u8>(
                hmap,
                "trig_mode_midi_track_2",
                None,
            )?,
            trig_mode_midi_track_3: parse_hashmap_string_value::<u8>(
                hmap,
                "trig_mode_midi_track_3",
                None,
            )?,
            trig_mode_midi_track_4: parse_hashmap_string_value::<u8>(
                hmap,
                "trig_mode_midi_track_4",
                None,
            )?,
            trig_mode_midi_track_5: parse_hashmap_string_value::<u8>(
                hmap,
                "trig_mode_midi_track_5",
                None,
            )?,
            trig_mode_midi_track_6: parse_hashmap_string_value::<u8>(
                hmap,
                "trig_mode_midi_track_6",
                None,
            )?,
            trig_mode_midi_track_7: parse_hashmap_string_value::<u8>(
                hmap,
                "trig_mode_midi_track_7",
                None,
            )?,
            trig_mode_midi_track_8: parse_hashmap_string_value::<u8>(
                hmap,
                "trig_mode_midi_track_8",
                None,
            )?,
        })
    }
}
