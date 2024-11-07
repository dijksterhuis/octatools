//! Project settings from a parsed Octatrack Project file.
//! e.g. whether Track 8 is a master track or not.

mod control_menu;
mod mixer;
mod tempo;
mod trig_mode_midi_tracks;

use serde::{Deserialize, Serialize};

use crate::common::{FromHashMap, FromString, ParseHashMapValueAs};

use crate::projects::{
    common::string_to_hashmap,
    common::ProjectRawFileSection,
    settings::{
        control_menu::ControlMenu, mixer::MixerMenu, tempo::TempoMenu,
        trig_mode_midi_tracks::MidiTrackTrigModes,
    },
};

/// Project settings read from a parsed Octatrack Project file

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ProjectSettings {
    /// Whether the project can be written to (is currently being read/written when `true`)
    pub write_protected: bool,

    /// Current settings in the `Project`'s control menu UI.
    pub control: ControlMenu,

    /// Unknown: Whether MIDI 'Thru' is enabled/disabled?
    pub midi_soft_thru: bool,

    /// Current state of the settings in the Mixer Menu overview
    pub mixer: MixerMenu,

    /// Current state of the settings in the Tempo menu
    pub tempo: TempoMenu,

    /// Current selections for MIDI Track Trig Mode
    pub midi_tracks_trig_mode: MidiTrackTrigModes,
}

impl ParseHashMapValueAs for ProjectSettings {}

impl FromString for ProjectSettings {
    type T = Self;

    /// Load project 'state' data from the raw project ASCII file.
    fn from_string(s: &String) -> Result<Self, Box<dyn std::error::Error>> {
        let hmap = string_to_hashmap(&s, &ProjectRawFileSection::Settings)?;

        Ok(Self {
            write_protected: Self::parse_hashmap_value_bool(&hmap, "writeprotected")?,
            // Unknown: Whether MIDI 'Thru' is enabled/disabled?
            midi_soft_thru: Self::parse_hashmap_value_bool(&hmap, "midi_soft_thru")?,
            //
            control: ControlMenu::from_hashmap(&hmap).unwrap(),
            mixer: MixerMenu::from_hashmap(&hmap)?,
            tempo: TempoMenu::from_hashmap(&hmap)?,
            midi_tracks_trig_mode: MidiTrackTrigModes::from_hashmap(&hmap)?,
        })
    }
}
