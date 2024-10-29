//! Utilities for reading `project.*` files.

use crate::octatrack::common::OptionEnumValueConvert;
use std::collections::HashMap;

/// ASCII data section headings within an Octatrack `project.*` file
pub enum ProjectRawFileSection {
    Meta,
    States,
    Settings,
    Samples,
}

impl OptionEnumValueConvert for ProjectRawFileSection {
    type T = ProjectRawFileSection;
    type V = String;

    fn from_value(v: Self::V) -> Result<Self::T, ()> {
        match v.to_ascii_uppercase().as_str() {
            "META" => Ok(Self::Meta),
            "STATES" => Ok(Self::States),
            "SETTINGS" => Ok(Self::Settings),
            "SAMPLES" => Ok(Self::Samples),
            _ => Err(()),
        }
    }

    // TODO: This should never error, so doesn't need a Result here!
    fn value(&self) -> Result<Self::V, ()> {
        match self {
            Self::Meta => Ok("META".to_string()),
            Self::States => Ok("STATES".to_string()),
            Self::Settings => Ok("SETTINGS".to_string()),
            Self::Samples => Ok("SAMPLES".to_string()),
        }
    }
}

impl ProjectRawFileSection {
    pub fn start_string(self: &Self) -> String {
        format!("[{}]", self.value().unwrap())
    }
    pub fn end_string(self: &Self) -> String {
        format!("[/{}]", self.value().unwrap())
    }
}

/// Extract ASCII string project data for a specified section as a HashMap of k-v pairs.

pub fn string_to_hashmap(
    data: &String,
    section: &ProjectRawFileSection,
) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let start_idx: usize = data.find(&section.start_string()).unwrap();
    let start_idx_shifted: usize = start_idx + &section.start_string().len();
    let end_idx: usize = data.find(&section.end_string()).unwrap();

    let section: String = data[start_idx_shifted..end_idx].to_string();

    let mut hmap: HashMap<String, String> = HashMap::new();
    let mut trig_mode_midi_field_idx = 1;

    for split_s in section.split("\r\n") {
        // new line splits returns empty fields :/

        if split_s != "" {
            let key_pair_string = split_s.to_string();
            let mut key_pair_split: Vec<&str> = key_pair_string.split("=").into_iter().collect();

            // there are 8x TRIG_MODE_MIDI key value pairs in project settings data
            // but the keys do not have track number indicators. i assume they're
            // stored in order of the midi track number, and each subsequent one we
            // read is the next track.
            let key_renamed: String = format!("trig_mode_midi_track_{}", &trig_mode_midi_field_idx);
            if key_pair_split[0] == "TRIG_MODE_MIDI" {
                key_pair_split[0] = key_renamed.as_str();
                trig_mode_midi_field_idx += 1;
            }

            hmap.insert(
                key_pair_split[0].to_string().to_ascii_lowercase(),
                key_pair_split[1].to_string(),
            );
        }
    }

    Ok(hmap)
}
