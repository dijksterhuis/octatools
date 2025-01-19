//! Parse Octatrack `project.*` data files.

pub mod metadata;
pub mod options;
pub mod settings;
pub mod slots;
pub mod states;

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, fmt::Debug, str::FromStr};

use crate::{
    projects::{
        metadata::ProjectMetadata, options::ProjectSampleSlotType, settings::ProjectSettings,
        slots::ProjectSampleSlot, states::ProjectStates,
    },
    Decode, Encode, OptionEnumValueConvert, RBoxErr, SerdeOctatrackErrors,
};

/// Trait to use when a new struct can be created from some hashmap with all the necessary fields.
trait FromHashMap {
    /// Type for `HashMap` keys
    type A;

    /// Type for `HashMap` values
    type B;

    /// Type for `Self`
    type T;

    /// Crete a new struct from a `HashMap`.
    fn from_hashmap(hmap: &HashMap<Self::A, Self::B>) -> Result<Self::T, Box<dyn Error>>;
}

/// Trait to use when a new struct can be created by reading a string.
trait ProjectFromString {
    /// Type for `Self`
    type T;

    /// Crete a new struct by parsing a `String`.
    fn from_string(data: &str) -> Result<Self::T, Box<dyn std::error::Error>>;
}

/// Trait to use when a new struct can be created by reading a string.
trait ProjectToString {
    /// Crete a new struct by parsing a `String`.
    fn to_string(&self) -> Result<String, Box<dyn std::error::Error>>;
}

/// Return the string value of a `HashMap<_, String>` parsed into specified type `T`
fn parse_hashmap_string_value<T: FromStr>(
    hmap: &HashMap<String, String>,
    key: &str,
    default_str: Option<&str>,
) -> Result<T, <T as FromStr>::Err>
where
    <T as FromStr>::Err: Debug,
{
    match default_str {
        Some(x) => hmap.get(key).unwrap_or(&x.to_string()).parse::<T>(),
        None => hmap.get(key).unwrap().parse::<T>(),
    }
}

/// Return the string value of a `HashMap<_, String>` parsed into a boolean value
/// (any parsed value != 1 returns `false`)
fn parse_hashmap_string_value_bool(
    hmap: &HashMap<String, String>,
    key: &str,
    default_str: Option<&str>,
) -> Result<bool, Box<dyn std::error::Error>> {
    // NOTE: https://rust-lang.github.io/rust-clippy/master/index.html#match_like_matches_macro
    Ok(matches!(
        parse_hashmap_string_value::<u8>(hmap, key, default_str)?,
        1
    ))
}

/// ASCII data section headings within an Octatrack `project.*` file
#[derive(Debug, PartialEq)]
enum ProjectRawFileSection {
    Meta,
    States,
    Settings,
    Samples,
}

impl OptionEnumValueConvert for ProjectRawFileSection {
    type T = ProjectRawFileSection;
    type V = String;

    fn from_value(v: &Self::V) -> RBoxErr<Self::T> {
        match v.to_ascii_uppercase().as_str() {
            "META" => Ok(Self::Meta),
            "STATES" => Ok(Self::States),
            "SETTINGS" => Ok(Self::Settings),
            "SAMPLES" => Ok(Self::Samples),
            _ => Err(SerdeOctatrackErrors::NoMatchingOptionEnumValue.into()),
        }
    }

    // TODO: doesn't need a Result here as should never error
    fn value(&self) -> RBoxErr<Self::V> {
        match self {
            Self::Meta => Ok("META".to_string()),
            Self::States => Ok("STATES".to_string()),
            Self::Settings => Ok("SETTINGS".to_string()),
            Self::Samples => Ok("SAMPLES".to_string()),
        }
    }
}

impl ProjectRawFileSection {
    fn start_string(&self) -> RBoxErr<String> {
        Ok(format!("[{}]", self.value()?))
    }
    fn end_string(&self) -> RBoxErr<String> {
        Ok(format!("[/{}]", self.value()?))
    }
}

/// Extract ASCII string project data for a specified section as a HashMap of k-v pairs.
fn string_to_hashmap(
    data: &str,
    section: &ProjectRawFileSection,
) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let start_idx: usize = data.find(&section.start_string()?).unwrap();
    let start_idx_shifted: usize = start_idx + section.start_string()?.len();
    let end_idx: usize = data.find(&section.end_string()?).unwrap();

    let section: String = data[start_idx_shifted..end_idx].to_string();

    let mut hmap: HashMap<String, String> = HashMap::new();
    let mut trig_mode_midi_field_idx = 1;

    for split_s in section.split("\r\n") {
        // new line splits returns empty fields :/

        if !split_s.is_empty() {
            let key_pair_string = split_s.to_string();
            let mut key_pair_split: Vec<&str> = key_pair_string.split('=').collect();

            // there are 8x TRIG_MODE_MIDI key value pairs in project settings data
            // but the keys do not have audio track number indicators. i assume they're
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

fn sslots_vec_to_string(v: &[ProjectSampleSlot]) -> String {
    let sslots_mapped: Vec<String> = v.iter().map(|x| x.to_string().unwrap()).collect();

    sslots_mapped.join("\r\n\r\n")
}

/// A parsed representation of an Octatrack Project file (`project.work` or `project.strd`).
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Project {
    /// Metadata key-value pairs from a Project file.
    pub metadata: ProjectMetadata,

    /// Settings key-value pairs from a Project file.
    pub settings: ProjectSettings,

    /// States key-value pairs from a Project file.
    pub states: ProjectStates,

    /// Slots key-value pairs from a Project file.
    pub slots: Vec<ProjectSampleSlot>,
}

impl Project {
    pub fn update_sample_slot_id(
        &mut self,
        old_slot_id: &u8,
        new_slot_id: &u8,
        sample_type: Option<ProjectSampleSlotType>,
    ) -> RBoxErr<()> {
        use itertools::Itertools;
        let type_filt = sample_type.unwrap_or(ProjectSampleSlotType::Static);

        let sample_slot_find: Option<(usize, ProjectSampleSlot)> = self
            .slots
            .clone()
            .into_iter()
            .find_position(|x| x.slot_id == *old_slot_id && x.sample_type == type_filt);

        // there are samples assigned to slots
        if sample_slot_find.is_some() {
            println!("Found matchin slot id");
            let mut sample_slot = sample_slot_find.clone().unwrap().1;
            sample_slot.slot_id = *new_slot_id;
            self.slots[sample_slot_find.unwrap().0] = sample_slot;
        }

        Ok(())
    }
}

impl Default for Project {
    fn default() -> Self {
        let metadata = ProjectMetadata::default();
        let states = ProjectStates::default();
        let settings = ProjectSettings::default();

        let slots: Vec<ProjectSampleSlot> = ProjectSampleSlot::default_vec();

        Project {
            metadata,
            settings,
            states,
            slots,
        }
    }
}

impl ProjectToString for Project {
    /// Turn a Project struct into a String configuration, ready for writing to binary data files
    fn to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let states_header =
            "############################\r\n# Project States\r\n############################"
                .to_string();
        let settings_header =
            "############################\r\n# Project Settings\r\n############################"
                .to_string();
        let slots_header =
            "############################\r\n# Samples\r\n############################".to_string();
        let footer = "############################".to_string();

        let metadata_string: String = self.metadata.to_string()?;
        let states_string: String = self.states.to_string()?;
        let settings_string: String = self.settings.to_string()?;

        let sslots_string = sslots_vec_to_string(&self.slots);

        let v: Vec<String> = vec![
            settings_header,
            metadata_string,
            settings_string,
            states_header,
            states_string,
            slots_header,
            sslots_string,
            footer,
        ];

        let mut project_string = v.join("\r\n\r\n");
        project_string.push_str("\r\n\r\n");
        Ok(project_string)
    }
}

// For project data, need to read bytes as an utf string, then split the structs out from the string
// data.
impl Decode for Project {
    fn decode(bytes: &[u8]) -> RBoxErr<Self> {
        let s = std::str::from_utf8(bytes)?.to_string();

        let metadata = ProjectMetadata::from_string(&s)?;
        let states = ProjectStates::from_string(&s)?;
        let settings = ProjectSettings::from_string(&s)?;
        let slots = ProjectSampleSlot::from_string(&s)?;

        Ok(Self {
            metadata,
            settings,
            states,
            slots,
        })
    }
}

// For project data, need to convert to string values again then into bytes
impl Encode for Project {
    fn encode(&self) -> RBoxErr<Vec<u8>> {
        let data = self.to_string()?;
        let bytes: Vec<u8> = data.bytes().collect::<Vec<u8>>();
        Ok(bytes)
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    mod test_spec {
        use super::*;

        #[test]
        fn section_heading_from_value_no_match_is_err() {
            assert!(ProjectRawFileSection::from_value(&"skfsdkfjskdh".to_string()).is_err())
        }

        #[test]
        fn section_heading_from_value_uppercase_meta() {
            assert_eq!(
                ProjectRawFileSection::from_value(&"META".to_string()).unwrap(),
                ProjectRawFileSection::Meta,
            )
        }

        #[test]
        fn section_heading_from_value_uppercase_states() {
            assert_eq!(
                ProjectRawFileSection::from_value(&"STATES".to_string()).unwrap(),
                ProjectRawFileSection::States,
            )
        }

        #[test]
        fn section_heading_from_value_uppercase_settings() {
            assert_eq!(
                ProjectRawFileSection::from_value(&"SETTINGS".to_string()).unwrap(),
                ProjectRawFileSection::Settings,
            )
        }

        #[test]
        fn section_heading_from_value_uppercase_samples() {
            assert_eq!(
                ProjectRawFileSection::from_value(&"SAMPLES".to_string()).unwrap(),
                ProjectRawFileSection::Samples,
            )
        }

        #[test]
        fn section_heading_from_value_lowercase_meta() {
            assert_eq!(
                ProjectRawFileSection::from_value(&"meta".to_string()).unwrap(),
                ProjectRawFileSection::Meta,
            )
        }

        #[test]
        fn section_heading_from_value_lowercase_states() {
            assert_eq!(
                ProjectRawFileSection::from_value(&"states".to_string()).unwrap(),
                ProjectRawFileSection::States,
            )
        }

        #[test]
        fn section_heading_from_value_lowercase_settings() {
            assert_eq!(
                ProjectRawFileSection::from_value(&"settings".to_string()).unwrap(),
                ProjectRawFileSection::Settings,
            )
        }

        #[test]
        fn section_heading_from_value_lowercase_samples() {
            assert_eq!(
                ProjectRawFileSection::from_value(&"samples".to_string()).unwrap(),
                ProjectRawFileSection::Samples,
            )
        }

        #[test]
        fn section_heading_value_meta() {
            assert_eq!(
                ProjectRawFileSection::Meta.value().unwrap(),
                "META".to_string(),
            )
        }

        #[test]
        fn section_heading_value_states() {
            assert_eq!(
                ProjectRawFileSection::States.value().unwrap(),
                "STATES".to_string(),
            )
        }

        #[test]
        fn section_heading_value_settings() {
            assert_eq!(
                ProjectRawFileSection::Settings.value().unwrap(),
                "SETTINGS".to_string(),
            )
        }

        #[test]
        fn section_heading_value_samples() {
            assert_eq!(
                ProjectRawFileSection::Samples.value().unwrap(),
                "SAMPLES".to_string(),
            )
        }
    }

    mod test_read {
        use super::*;

        #[test]
        fn test_full_to_string() {
            let valid = "############################\r\n# Project Settings\r\n############################\r\n\r\n[META]\r\nTYPE=OCTATRACK DPS-1 PROJECT\r\nVERSION=19\r\nOS_VERSION=R0177     1.40B\r\n[/META]\r\n\r\n[SETTINGS]\r\nWRITEPROTECTED=0\r\nTEMPOx24=2880\r\nPATTERN_TEMPO_ENABLED=0\r\nMIDI_CLOCK_SEND=0\r\nMIDI_CLOCK_RECEIVE=0\r\nMIDI_TRANSPORT_SEND=0\r\nMIDI_TRANSPORT_RECEIVE=0\r\nMIDI_PROGRAM_CHANGE_SEND=0\r\nMIDI_PROGRAM_CHANGE_SEND_CH=-1\r\nMIDI_PROGRAM_CHANGE_RECEIVE=0\r\nMIDI_PROGRAM_CHANGE_RECEIVE_CH=-1\r\nMIDI_TRIG_CH1=0\r\nMIDI_TRIG_CH2=1\r\nMIDI_TRIG_CH3=2\r\nMIDI_TRIG_CH4=3\r\nMIDI_TRIG_CH5=4\r\nMIDI_TRIG_CH6=5\r\nMIDI_TRIG_CH7=6\r\nMIDI_TRIG_CH8=7\r\nMIDI_AUTO_CHANNEL=10\r\nMIDI_SOFT_THRU=0\r\nMIDI_AUDIO_TRK_CC_IN=1\r\nMIDI_AUDIO_TRK_CC_OUT=3\r\nMIDI_AUDIO_TRK_NOTE_IN=1\r\nMIDI_AUDIO_TRK_NOTE_OUT=3\r\nMIDI_MIDI_TRK_CC_IN=1\r\nPATTERN_CHANGE_CHAIN_BEHAVIOR=0\r\nPATTERN_CHANGE_AUTO_SILENCE_TRACKS=0\r\nPATTERN_CHANGE_AUTO_TRIG_LFOS=0\r\nLOAD_24BIT_FLEX=0\r\nDYNAMIC_RECORDERS=0\r\nRECORD_24BIT=0\r\nRESERVED_RECORDER_COUNT=8\r\nRESERVED_RECORDER_LENGTH=16\r\nINPUT_DELAY_COMPENSATION=0\r\nGATE_AB=127\r\nGATE_CD=127\r\nGAIN_AB=64\r\nGAIN_CD=64\r\nDIR_AB=0\r\nDIR_CD=0\r\nPHONES_MIX=64\r\nMAIN_TO_CUE=0\r\nMASTER_TRACK=0\r\nCUE_STUDIO_MODE=0\r\nMAIN_LEVEL=64\r\nCUE_LEVEL=64\r\nMETRONOME_TIME_SIGNATURE=3\r\nMETRONOME_TIME_SIGNATURE_DENOMINATOR=2\r\nMETRONOME_PREROLL=0\r\nMETRONOME_CUE_VOLUME=32\r\nMETRONOME_MAIN_VOLUME=0\r\nMETRONOME_PITCH=12\r\nMETRONOME_TONAL=1\r\nMETRONOME_ENABLED=0\r\nTRIG_MODE_MIDI=0\r\nTRIG_MODE_MIDI=0\r\nTRIG_MODE_MIDI=0\r\nTRIG_MODE_MIDI=0\r\nTRIG_MODE_MIDI=0\r\nTRIG_MODE_MIDI=0\r\nTRIG_MODE_MIDI=0\r\nTRIG_MODE_MIDI=0\r\n[/SETTINGS]\r\n\r\n############################\r\n# Project States\r\n############################\r\n\r\n[STATES]\r\nBANK=0\r\nPATTERN=0\r\nARRANGEMENT=0\r\nARRANGEMENT_MODE=0\r\nPART=0\r\nTRACK=0\r\nTRACK_OTHERMODE=0\r\nSCENE_A_MUTE=0\r\nSCENE_B_MUTE=0\r\nTRACK_CUE_MASK=0\r\nTRACK_MUTE_MASK=0\r\nTRACK_SOLO_MASK=0\r\nMIDI_TRACK_MUTE_MASK=0\r\nMIDI_TRACK_SOLO_MASK=0\r\nMIDI_MODE=0\r\n[/STATES]\r\n\r\n############################\r\n# Samples\r\n############################\r\n\r\n[SAMPLE]\r\nTYPE=FLEX\r\nSLOT=129\r\nPATH=\r\nTRIM_BARSx100=0\r\nTSMODE=2\r\nLOOPMODE=0\r\nGAIN=72\r\nTRIGQUANTIZATION=255\r\n[/SAMPLE]\r\n\r\n[SAMPLE]\r\nTYPE=FLEX\r\nSLOT=130\r\nPATH=\r\nTRIM_BARSx100=0\r\nTSMODE=2\r\nLOOPMODE=0\r\nGAIN=72\r\nTRIGQUANTIZATION=255\r\n[/SAMPLE]\r\n\r\n[SAMPLE]\r\nTYPE=FLEX\r\nSLOT=131\r\nPATH=\r\nTRIM_BARSx100=0\r\nTSMODE=2\r\nLOOPMODE=0\r\nGAIN=72\r\nTRIGQUANTIZATION=255\r\n[/SAMPLE]\r\n\r\n[SAMPLE]\r\nTYPE=FLEX\r\nSLOT=132\r\nPATH=\r\nTRIM_BARSx100=0\r\nTSMODE=2\r\nLOOPMODE=0\r\nGAIN=72\r\nTRIGQUANTIZATION=255\r\n[/SAMPLE]\r\n\r\n[SAMPLE]\r\nTYPE=FLEX\r\nSLOT=133\r\nPATH=\r\nTRIM_BARSx100=0\r\nTSMODE=2\r\nLOOPMODE=0\r\nGAIN=72\r\nTRIGQUANTIZATION=255\r\n[/SAMPLE]\r\n\r\n[SAMPLE]\r\nTYPE=FLEX\r\nSLOT=134\r\nPATH=\r\nTRIM_BARSx100=0\r\nTSMODE=2\r\nLOOPMODE=0\r\nGAIN=72\r\nTRIGQUANTIZATION=255\r\n[/SAMPLE]\r\n\r\n[SAMPLE]\r\nTYPE=FLEX\r\nSLOT=135\r\nPATH=\r\nTRIM_BARSx100=0\r\nTSMODE=2\r\nLOOPMODE=0\r\nGAIN=72\r\nTRIGQUANTIZATION=255\r\n[/SAMPLE]\r\n\r\n[SAMPLE]\r\nTYPE=FLEX\r\nSLOT=136\r\nPATH=\r\nTRIM_BARSx100=0\r\nTSMODE=2\r\nLOOPMODE=0\r\nGAIN=72\r\nTRIGQUANTIZATION=255\r\n[/SAMPLE]\r\n\r\n############################\r\n\r\n";
            assert_eq!(Project::default().to_string().unwrap(), valid);
        }

        #[test]
        fn test_metadata_to_string() {
            let valid = "[META]\r\nTYPE=OCTATRACK DPS-1 PROJECT\r\nVERSION=19\r\nOS_VERSION=R0177     1.40B\r\n[/META]";
            assert_eq!(ProjectMetadata::default().to_string().unwrap(), valid);
        }

        #[test]
        fn test_states_to_string() {
            let valid = "[STATES]\r\nBANK=0\r\nPATTERN=0\r\nARRANGEMENT=0\r\nARRANGEMENT_MODE=0\r\nPART=0\r\nTRACK=0\r\nTRACK_OTHERMODE=0\r\nSCENE_A_MUTE=0\r\nSCENE_B_MUTE=0\r\nTRACK_CUE_MASK=0\r\nTRACK_MUTE_MASK=0\r\nTRACK_SOLO_MASK=0\r\nMIDI_TRACK_MUTE_MASK=0\r\nMIDI_TRACK_SOLO_MASK=0\r\nMIDI_MODE=0\r\n[/STATES]";
            assert_eq!(ProjectStates::default().to_string().unwrap(), valid);
        }

        #[test]
        fn test_settings_to_string() {
            let valid = "[SETTINGS]\r\nWRITEPROTECTED=0\r\nTEMPOx24=2880\r\nPATTERN_TEMPO_ENABLED=0\r\nMIDI_CLOCK_SEND=0\r\nMIDI_CLOCK_RECEIVE=0\r\nMIDI_TRANSPORT_SEND=0\r\nMIDI_TRANSPORT_RECEIVE=0\r\nMIDI_PROGRAM_CHANGE_SEND=0\r\nMIDI_PROGRAM_CHANGE_SEND_CH=-1\r\nMIDI_PROGRAM_CHANGE_RECEIVE=0\r\nMIDI_PROGRAM_CHANGE_RECEIVE_CH=-1\r\nMIDI_TRIG_CH1=0\r\nMIDI_TRIG_CH2=1\r\nMIDI_TRIG_CH3=2\r\nMIDI_TRIG_CH4=3\r\nMIDI_TRIG_CH5=4\r\nMIDI_TRIG_CH6=5\r\nMIDI_TRIG_CH7=6\r\nMIDI_TRIG_CH8=7\r\nMIDI_AUTO_CHANNEL=10\r\nMIDI_SOFT_THRU=0\r\nMIDI_AUDIO_TRK_CC_IN=1\r\nMIDI_AUDIO_TRK_CC_OUT=3\r\nMIDI_AUDIO_TRK_NOTE_IN=1\r\nMIDI_AUDIO_TRK_NOTE_OUT=3\r\nMIDI_MIDI_TRK_CC_IN=1\r\nPATTERN_CHANGE_CHAIN_BEHAVIOR=0\r\nPATTERN_CHANGE_AUTO_SILENCE_TRACKS=0\r\nPATTERN_CHANGE_AUTO_TRIG_LFOS=0\r\nLOAD_24BIT_FLEX=0\r\nDYNAMIC_RECORDERS=0\r\nRECORD_24BIT=0\r\nRESERVED_RECORDER_COUNT=8\r\nRESERVED_RECORDER_LENGTH=16\r\nINPUT_DELAY_COMPENSATION=0\r\nGATE_AB=127\r\nGATE_CD=127\r\nGAIN_AB=64\r\nGAIN_CD=64\r\nDIR_AB=0\r\nDIR_CD=0\r\nPHONES_MIX=64\r\nMAIN_TO_CUE=0\r\nMASTER_TRACK=0\r\nCUE_STUDIO_MODE=0\r\nMAIN_LEVEL=64\r\nCUE_LEVEL=64\r\nMETRONOME_TIME_SIGNATURE=3\r\nMETRONOME_TIME_SIGNATURE_DENOMINATOR=2\r\nMETRONOME_PREROLL=0\r\nMETRONOME_CUE_VOLUME=32\r\nMETRONOME_MAIN_VOLUME=0\r\nMETRONOME_PITCH=12\r\nMETRONOME_TONAL=1\r\nMETRONOME_ENABLED=0\r\nTRIG_MODE_MIDI=0\r\nTRIG_MODE_MIDI=0\r\nTRIG_MODE_MIDI=0\r\nTRIG_MODE_MIDI=0\r\nTRIG_MODE_MIDI=0\r\nTRIG_MODE_MIDI=0\r\nTRIG_MODE_MIDI=0\r\nTRIG_MODE_MIDI=0\r\n[/SETTINGS]";
            assert_eq!(ProjectSettings::default().to_string().unwrap(), valid);
        }

        #[test]
        fn test_sslots_to_string() {
            let valid = "[SAMPLE]\r\nTYPE=FLEX\r\nSLOT=129\r\nPATH=\r\nTRIM_BARSx100=0\r\nTSMODE=2\r\nLOOPMODE=0\r\nGAIN=72\r\nTRIGQUANTIZATION=255\r\n[/SAMPLE]\r\n\r\n[SAMPLE]\r\nTYPE=FLEX\r\nSLOT=130\r\nPATH=\r\nTRIM_BARSx100=0\r\nTSMODE=2\r\nLOOPMODE=0\r\nGAIN=72\r\nTRIGQUANTIZATION=255\r\n[/SAMPLE]\r\n\r\n[SAMPLE]\r\nTYPE=FLEX\r\nSLOT=131\r\nPATH=\r\nTRIM_BARSx100=0\r\nTSMODE=2\r\nLOOPMODE=0\r\nGAIN=72\r\nTRIGQUANTIZATION=255\r\n[/SAMPLE]\r\n\r\n[SAMPLE]\r\nTYPE=FLEX\r\nSLOT=132\r\nPATH=\r\nTRIM_BARSx100=0\r\nTSMODE=2\r\nLOOPMODE=0\r\nGAIN=72\r\nTRIGQUANTIZATION=255\r\n[/SAMPLE]\r\n\r\n[SAMPLE]\r\nTYPE=FLEX\r\nSLOT=133\r\nPATH=\r\nTRIM_BARSx100=0\r\nTSMODE=2\r\nLOOPMODE=0\r\nGAIN=72\r\nTRIGQUANTIZATION=255\r\n[/SAMPLE]\r\n\r\n[SAMPLE]\r\nTYPE=FLEX\r\nSLOT=134\r\nPATH=\r\nTRIM_BARSx100=0\r\nTSMODE=2\r\nLOOPMODE=0\r\nGAIN=72\r\nTRIGQUANTIZATION=255\r\n[/SAMPLE]\r\n\r\n[SAMPLE]\r\nTYPE=FLEX\r\nSLOT=135\r\nPATH=\r\nTRIM_BARSx100=0\r\nTSMODE=2\r\nLOOPMODE=0\r\nGAIN=72\r\nTRIGQUANTIZATION=255\r\n[/SAMPLE]\r\n\r\n[SAMPLE]\r\nTYPE=FLEX\r\nSLOT=136\r\nPATH=\r\nTRIM_BARSx100=0\r\nTSMODE=2\r\nLOOPMODE=0\r\nGAIN=72\r\nTRIGQUANTIZATION=255\r\n[/SAMPLE]";
            assert_eq!(
                sslots_vec_to_string(&ProjectSampleSlot::default_vec()),
                valid
            );
        }
    }
}
