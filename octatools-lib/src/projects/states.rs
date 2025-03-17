//! A project's related state, usually what lights will be shown on the box.
//! e.g. the currently seleced pattern/bank/wehther either scene is muted, current muted tracks etc.

use crate::RBoxErr;
use serde::{Deserialize, Serialize};

use crate::projects::{
    parse_hashmap_string_value, parse_hashmap_string_value_bool, string_to_hashmap,
    ProjectFromString, ProjectRawFileSection, ProjectToString,
};

/*
[STATES]\r\nBANK=0\r\nPATTERN=0\r\nARRANGEMENT=0\r\nARRANGEMENT_MODE=0\r\nPART=0\r\nTRACK=0\r\nTRACK_OTHERMODE=0\r\nSCENE_A_MUTE=0\r\nSCENE_B_MUTE=0\r\nTRACK_CUE_MASK=0\r\nTRACK_MUTE_MASK=0\r\nTRACK_SOLO_MASK=0\r\nMIDI_TRACK_MUTE_MASK=0\r\nMIDI_TRACK_SOLO_MASK=0\r\nMIDI_MODE=0\r\n[/STATES]

----

[STATES]
BANK=0
PATTERN=0
ARRANGEMENT=0
ARRANGEMENT_MODE=0
PART=0
TRACK=0
TRACK_OTHERMODE=0
SCENE_A_MUTE=0
SCENE_B_MUTE=0
TRACK_CUE_MASK=0
TRACK_MUTE_MASK=0
TRACK_SOLO_MASK=0
MIDI_TRACK_MUTE_MASK=0
MIDI_TRACK_SOLO_MASK=0
MIDI_MODE=0
[/STATES]
*/

/// Project state from a parsed Octatrack `project.*` file.
/// This is the current 'UX focus' state, i.e. what parts, patterns, tracks are currently selected etc.

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub struct ProjectStates {
    /// Current active bank
    pub bank: u8,

    /// Current active pattern within a bank
    pub pattern: u8,

    /// Current active arrangement.
    pub arrangement: u8,

    /// Whether in arrangement mode or not.
    pub arrangement_mode: u8, // dunno if this is a toggle or an enum

    /// Current active part for the active pattern within a bank
    pub part: u8,

    /// Current active track
    pub track: u8,

    /// Unknown
    pub track_othermode: u8, // WTFF is this?

    /// Whether Scene A is disabled/enabled
    pub scene_a_mute: bool,

    /// Whether Scene B is disabled/enabled
    pub scene_b_mute: bool,

    /// Mask detailingg which CUE outputs are muted on Audio Tracks.  
    pub track_cue_mask: u8,

    /// Mask detailingg which MAIN outputs are muted on Audio Tracks.  
    pub track_mute_mask: u8,

    /// Mask detailingg which MAIN outputs are soloed on Audio Tracks.  
    pub track_solo_mask: u8,

    /// Mask detailingg which MIDI outputs are muted on MIDI  Tracks.  
    pub midi_track_mute_mask: u8,

    /// Mask detailingg which MIDI outputs are soloed on MIDI  Tracks.  
    pub midi_track_solo_mask: u8,

    // Unknown
    pub midi_mode: u8,
}

impl ProjectFromString for ProjectStates {
    type T = Self;

    /// Load project 'state' data from the raw project ASCII file.
    fn from_string(s: &str) -> RBoxErr<Self> {
        let hmap = string_to_hashmap(s, &ProjectRawFileSection::States)?;

        Ok(Self {
            bank: parse_hashmap_string_value::<u8>(&hmap, "bank", None)?,
            pattern: parse_hashmap_string_value::<u8>(&hmap, "pattern", None)?,
            arrangement: parse_hashmap_string_value::<u8>(&hmap, "arrangement", None)?,
            arrangement_mode: parse_hashmap_string_value::<u8>(&hmap, "arrangement_mode", None)?,
            part: parse_hashmap_string_value::<u8>(&hmap, "part", None)?,
            track: parse_hashmap_string_value::<u8>(&hmap, "track", None)?,
            track_othermode: parse_hashmap_string_value::<u8>(&hmap, "track_othermode", None)?,
            scene_a_mute: parse_hashmap_string_value_bool(&hmap, "scene_a_mute", None)?,
            scene_b_mute: parse_hashmap_string_value_bool(&hmap, "scene_b_mute", None)?,
            track_cue_mask: parse_hashmap_string_value::<u8>(&hmap, "track_cue_mask", None)?,
            track_mute_mask: parse_hashmap_string_value::<u8>(&hmap, "track_mute_mask", None)?,
            track_solo_mask: parse_hashmap_string_value::<u8>(&hmap, "track_solo_mask", None)?,
            midi_track_mute_mask: parse_hashmap_string_value::<u8>(
                &hmap,
                "midi_track_mute_mask",
                None,
            )?,
            midi_track_solo_mask: parse_hashmap_string_value::<u8>(
                &hmap,
                "midi_track_solo_mask",
                None,
            )?,
            midi_mode: parse_hashmap_string_value::<u8>(&hmap, "midi_mode", None)?,
        })
    }
}

impl ProjectToString for ProjectStates {
    /// Extract `OctatrackProjectMetadata` fields from the project file's ASCII data
    fn to_string(&self) -> RBoxErr<String> {
        let mut s = "".to_string();
        s.push_str("[STATES]\r\n");
        s.push_str(format!("BANK={}", self.bank).as_str());
        s.push_str("\r\n");
        s.push_str(format!("PATTERN={}", self.pattern).as_str());
        s.push_str("\r\n");
        s.push_str(format!("ARRANGEMENT={}", self.arrangement).as_str());
        s.push_str("\r\n");
        s.push_str(format!("ARRANGEMENT_MODE={}", self.arrangement_mode).as_str());
        s.push_str("\r\n");
        s.push_str(format!("PART={}", self.part).as_str());
        s.push_str("\r\n");
        s.push_str(format!("TRACK={}", self.track).as_str());
        s.push_str("\r\n");
        s.push_str(format!("TRACK_OTHERMODE={}", self.track_othermode).as_str());
        s.push_str("\r\n");
        s.push_str(format!("SCENE_A_MUTE={}", self.scene_a_mute as u8).as_str());
        s.push_str("\r\n");
        s.push_str(format!("SCENE_B_MUTE={}", self.scene_b_mute as u8).as_str());
        s.push_str("\r\n");
        s.push_str(format!("TRACK_CUE_MASK={}", self.track_cue_mask).as_str());
        s.push_str("\r\n");
        s.push_str(format!("TRACK_MUTE_MASK={}", self.track_mute_mask).as_str());
        s.push_str("\r\n");
        s.push_str(format!("TRACK_SOLO_MASK={}", self.track_solo_mask).as_str());
        s.push_str("\r\n");
        s.push_str(format!("MIDI_TRACK_MUTE_MASK={}", self.midi_track_mute_mask).as_str());
        s.push_str("\r\n");
        s.push_str(format!("MIDI_TRACK_SOLO_MASK={}", self.midi_track_solo_mask).as_str());
        s.push_str("\r\n");
        s.push_str(format!("MIDI_MODE={}", self.midi_mode).as_str());
        s.push_str("\r\n[/STATES]");

        Ok(s)
    }
}
