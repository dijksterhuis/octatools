//! Project sample slots read from a parsed Octatrack Project file.
//! This only loads data from the project file.
//! Samples not added to a project sample lsit for sstatic/flex machines will not be loaded.
//! **NOTE**: any fields matching those in an Octatrack sample attributes file
//! may not have been writtten to an attributes file yet.
//! (these are project files loaded into memory when switching to the project)/

/*
Example data:
[SAMPLE]\r\nTYPE=FLEX\r\nSLOT=001\r\nPATH=../AUDIO/flex.wav\r\nTRIM_BARSx100=173\r\nTSMODE=2\r\nLOOPMODE=1\r\nGAIN=48\r\nTRIGQUANTIZATION=-1\r\n[/SAMPLE]
-----

[SAMPLE]
TYPE=FLEX
SLOT=001
PATH=../AUDIO/flex.wav
TRIM_BARSx100=173
TSMODE=2
LOOPMODE=1
GAIN=48
TRIGQUANTIZATION=-1
[/SAMPLE]
*/

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, convert::TryFrom, path::PathBuf, str::FromStr};

use crate::{
    projects::{
        options::ProjectSampleSlotType, parse_hashmap_string_value, FromHashMap, ProjectFromString,
        ProjectToString,
    },
    samples::options::{
        SampleAttributeLoopMode, SampleAttributeTimestrechMode, SampleAttributeTrigQuantizationMode,
    },
    OptionEnumValueConvert, RBoxErr, SerdeOctatrackErrors,
};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
pub struct ProjectSampleSlot {
    /// Type of sample: STATIC or FLEX
    pub sample_type: ProjectSampleSlotType,

    /// String ID Number of the slot the sample is assigned to e.g. 001, 002, 003...
    /// Maximum of 128 entries for STATIC sample slots, but can be up to 136 for flex
    /// slots as there are 8 recorders + 128 flex slots.
    pub slot_id: u8,

    /// Relative path to the file on the card from the project directory.
    pub path: PathBuf,

    /// Current bar trim (float). This is multiplied by 100 on the machine.
    /// This is not used for recording buffer 'flex' tracks.
    pub trim_bars_x100: u16,

    /// Current `SampleTimestrechModes` setting for the specific slot. Example: `TSMODE=2`
    pub timestrech_mode: SampleAttributeTimestrechMode,

    /// Current `SampleLoopModes` setting for the specific slot.
    pub loop_mode: SampleAttributeLoopMode,

    /// Current `SampleTrigQuantizationModes` setting for this specific slot.
    /// This is not used for recording buffer 'flex' tracks.
    pub trig_quantization_mode: SampleAttributeTrigQuantizationMode,

    /// Sample gain. 48 is default as per sample attributes file. maximum 96, minimum 0.
    pub gain: i8,

    /// BPM of the sample in this slot.
    pub bpm: u16,
}

#[allow(clippy::too_many_arguments)] // not my fault there's a bunch of inputs for this...
impl ProjectSampleSlot {
    pub fn new(
        sample_type: ProjectSampleSlotType,
        slot_id: u8,
        path: PathBuf,
        trim_bars_x100: Option<u16>,
        timestretch_mode: Option<SampleAttributeTimestrechMode>,
        loop_mode: Option<SampleAttributeLoopMode>,
        trig_quantization_mode: Option<SampleAttributeTrigQuantizationMode>,
        gain: Option<i8>,
        bpm: Option<u16>,
    ) -> RBoxErr<Self> {
        Ok(ProjectSampleSlot {
            sample_type,
            slot_id,
            path,
            trim_bars_x100: trim_bars_x100.unwrap_or(0),
            timestrech_mode: timestretch_mode.unwrap_or_default(),
            loop_mode: loop_mode.unwrap_or_default(),
            trig_quantization_mode: trig_quantization_mode.unwrap_or_default(),
            gain: gain.unwrap_or(24),
            bpm: bpm.unwrap_or(120),
        })
    }

    /// Create a default vector of Project Sample Slots; 8x Recorder Buffers.
    pub fn defaults() -> Vec<Self> {
        let mut slots = [
            ProjectSampleSlot {
                sample_type: ProjectSampleSlotType::RecorderBuffer,
                slot_id: 129,
                path: PathBuf::from(""),
                trim_bars_x100: 0,
                timestrech_mode: SampleAttributeTimestrechMode::default(),
                loop_mode: SampleAttributeLoopMode::default(),
                trig_quantization_mode: SampleAttributeTrigQuantizationMode::default(),
                gain: 24,
                bpm: 120,
            },
            ProjectSampleSlot {
                sample_type: ProjectSampleSlotType::RecorderBuffer,
                slot_id: 130,
                path: PathBuf::from(""),
                trim_bars_x100: 0,
                timestrech_mode: SampleAttributeTimestrechMode::default(),
                loop_mode: SampleAttributeLoopMode::default(),
                trig_quantization_mode: SampleAttributeTrigQuantizationMode::default(),
                gain: 24,
                bpm: 120,
            },
            ProjectSampleSlot {
                sample_type: ProjectSampleSlotType::RecorderBuffer,
                slot_id: 131,
                path: PathBuf::from(""),
                trim_bars_x100: 0,
                timestrech_mode: SampleAttributeTimestrechMode::default(),
                loop_mode: SampleAttributeLoopMode::default(),
                trig_quantization_mode: SampleAttributeTrigQuantizationMode::default(),
                gain: 24,
                bpm: 120,
            },
            ProjectSampleSlot {
                sample_type: ProjectSampleSlotType::RecorderBuffer,
                slot_id: 132,
                path: PathBuf::from(""),
                trim_bars_x100: 0,
                timestrech_mode: SampleAttributeTimestrechMode::default(),
                loop_mode: SampleAttributeLoopMode::default(),
                trig_quantization_mode: SampleAttributeTrigQuantizationMode::default(),
                gain: 24,
                bpm: 120,
            },
            ProjectSampleSlot {
                sample_type: ProjectSampleSlotType::RecorderBuffer,
                slot_id: 133,
                path: PathBuf::from(""),
                trim_bars_x100: 0,
                timestrech_mode: SampleAttributeTimestrechMode::default(),
                loop_mode: SampleAttributeLoopMode::default(),
                trig_quantization_mode: SampleAttributeTrigQuantizationMode::default(),
                gain: 24,
                bpm: 120,
            },
            ProjectSampleSlot {
                sample_type: ProjectSampleSlotType::RecorderBuffer,
                slot_id: 134,
                path: PathBuf::from(""),
                trim_bars_x100: 0,
                timestrech_mode: SampleAttributeTimestrechMode::default(),
                loop_mode: SampleAttributeLoopMode::default(),
                trig_quantization_mode: SampleAttributeTrigQuantizationMode::default(),
                gain: 24,
                bpm: 120,
            },
            ProjectSampleSlot {
                sample_type: ProjectSampleSlotType::RecorderBuffer,
                slot_id: 135,
                path: PathBuf::from(""),
                trim_bars_x100: 0,
                timestrech_mode: SampleAttributeTimestrechMode::default(),
                loop_mode: SampleAttributeLoopMode::default(),
                trig_quantization_mode: SampleAttributeTrigQuantizationMode::default(),
                gain: 24,
                bpm: 120,
            },
            ProjectSampleSlot {
                sample_type: ProjectSampleSlotType::RecorderBuffer,
                slot_id: 136,
                path: PathBuf::from(""),
                trim_bars_x100: 0,
                timestrech_mode: SampleAttributeTimestrechMode::default(),
                loop_mode: SampleAttributeLoopMode::default(),
                trig_quantization_mode: SampleAttributeTrigQuantizationMode::default(),
                gain: 24,
                bpm: 120,
            },
        ]
        .to_vec();
        slots.sort_by_key(|x| x.slot_id);
        slots
    }
}

fn parse_id(hmap: &HashMap<String, String>) -> RBoxErr<u8> {
    let x = parse_hashmap_string_value::<u8>(hmap, "slot", None);

    // ParseIntError doesn't allow ? usage
    if x.is_err() {
        return Err(Box::new(
            SerdeOctatrackErrors::ProjectSampleSlotParsingError,
        ));
    }

    Ok(x?)
}

fn parse_trim_bars(hmap: &HashMap<String, String>) -> RBoxErr<u16> {
    let x = parse_hashmap_string_value::<u16>(hmap, "trim_barsx100", Some("0")).unwrap_or(0);
    Ok(x)
}

fn parse_loop_mode(hmap: &HashMap<String, String>) -> RBoxErr<SampleAttributeLoopMode> {
    let x = parse_hashmap_string_value::<u32>(hmap, "loopmode", Some("0")).unwrap_or(0_u32);
    SampleAttributeLoopMode::from_value(&x)
}

fn parse_tstrech_mode(hmap: &HashMap<String, String>) -> RBoxErr<SampleAttributeTimestrechMode> {
    let x = parse_hashmap_string_value::<u32>(hmap, "tsmode", Some("0")).unwrap_or(0_u32);
    SampleAttributeTimestrechMode::from_value(&x)
}

fn parse_trig_quantize_mode(
    hmap: &HashMap<String, String>,
) -> RBoxErr<SampleAttributeTrigQuantizationMode> {
    let x_i16 =
        parse_hashmap_string_value::<i16>(hmap, "trigquantization", Some("255")).unwrap_or(255_i16);
    let x_u32 = u32::try_from(x_i16).unwrap_or(255_u32);
    SampleAttributeTrigQuantizationMode::from_value(&x_u32)
}

fn parse_gain(hmap: &HashMap<String, String>) -> RBoxErr<i8> {
    let x = parse_hashmap_string_value::<i8>(hmap, "gain", Some("48")).unwrap_or(48_i8);
    Ok(x - 48_i8)
}

fn parse_tempo(hmap: &HashMap<String, String>) -> RBoxErr<u16> {
    let x = parse_hashmap_string_value::<u16>(hmap, "bpm", Some("2880")).unwrap_or(2880_u16);
    Ok(x / 24_u16)
}

// cannot use FromProjectStringData because it expects a lone Self result, rather than a Vec.
impl FromHashMap for ProjectSampleSlot {
    type A = String;
    type B = String;
    type T = ProjectSampleSlot;

    fn from_hashmap(hmap: &HashMap<Self::A, Self::B>) -> RBoxErr<Self::T> {
        let slot_id = parse_id(hmap)?;

        // recorder buffers are the only slots with IDs > 128
        let sample_slot_type = if slot_id >= 129 {
            "RECORDER".to_string()
        } else {
            // TODO: option plain unwrap
            hmap.get("type").unwrap().to_string()
        };

        let sample_type = ProjectSampleSlotType::from_value(&sample_slot_type)?;
        // TODO: option plain unwrap
        let path = PathBuf::from_str(hmap.get("path").unwrap())?;
        let trim_bars = parse_trim_bars(hmap)?;
        let loop_mode = parse_loop_mode(hmap)?;
        let timestrech_mode = parse_tstrech_mode(hmap)?;
        let trig_quantization_mode = parse_trig_quantize_mode(hmap)?;
        // todo: check gain transformation values
        let gain = parse_gain(hmap)?;
        let bpm = parse_tempo(hmap)?;

        let sample_struct = Self {
            sample_type,
            slot_id,
            path,
            trim_bars_x100: trim_bars,
            timestrech_mode,
            loop_mode,
            trig_quantization_mode,
            gain,
            bpm,
        };

        Ok(sample_struct)
    }
}

impl ProjectFromString for ProjectSampleSlot {
    type T = Vec<Self>;

    /// Load project 'samples' data from the raw project ASCII file.
    fn from_string(data: &str) -> RBoxErr<Vec<Self>> {
        // TODO: option plain unwrap
        let footer_stripped = data
            .strip_suffix("\r\n\r\n############################\r\n\r\n")
            .unwrap();

        let data_window: Vec<&str> = footer_stripped
            .split("############################\r\n# Samples\r\n############################")
            .collect();

        let mut samples_string: Vec<&str> = data_window[1].split("[/SAMPLE]").collect();

        // last one is always a blank string.
        samples_string.pop();

        let samples: Vec<Vec<Vec<&str>>> = samples_string
            .into_iter()
            .map(|sample: &str| {
                // TODO: option plain unwraps
                sample
                    .strip_prefix("\r\n\r\n[SAMPLE]\r\n")
                    .unwrap()
                    .strip_suffix("\r\n")
                    .unwrap()
                    .split("\r\n")
                    .map(|x: &str| x.split('=').collect_vec())
                    .filter(|x: &Vec<&str>| x.len() == 2)
                    .collect_vec()
            })
            .collect();

        let mut sample_structs: Vec<ProjectSampleSlot> = Vec::new();
        for sample in samples {
            let mut hmap: HashMap<String, String> = HashMap::new();
            for key_value_pair in sample {
                hmap.insert(
                    key_value_pair[0].to_string().to_lowercase(),
                    key_value_pair[1].to_string(),
                );
            }

            let sample_struct = Self::from_hashmap(&hmap)?;

            sample_structs.push(sample_struct);
        }

        Ok(sample_structs)
    }
}

impl ProjectToString for ProjectSampleSlot {
    /// Extract `OctatrackProjectMetadata` fields from the project file's ASCII data
    fn to_string(&self) -> RBoxErr<String> {
        // Recording buffers are actually stored as FLEX slots with
        // a slot ID > 128.
        let sample_type = match self.sample_type {
            ProjectSampleSlotType::Static | ProjectSampleSlotType::Flex => {
                self.sample_type.value()?
            }
            ProjectSampleSlotType::RecorderBuffer => "FLEX".to_string(),
        };

        let mut s = "[SAMPLE]\r\n".to_string();
        s.push_str(format!("TYPE={}", sample_type).as_str());
        s.push_str("\r\n");
        s.push_str(format!("SLOT={}", self.slot_id).as_str());
        s.push_str("\r\n");
        s.push_str(format!("PATH={:#?}", self.path).replace('"', "").as_str());
        s.push_str("\r\n");
        s.push_str(format!("TRIM_BARSx100={}", self.trim_bars_x100).as_str());
        s.push_str("\r\n");
        s.push_str(format!("TSMODE={}", self.timestrech_mode.value()?).as_str());
        s.push_str("\r\n");
        s.push_str(format!("LOOPMODE={}", self.loop_mode.value()?).as_str());
        s.push_str("\r\n");
        s.push_str(format!("GAIN={}", self.gain + 48).as_str());
        s.push_str("\r\n");
        s.push_str(format!("TRIGQUANTIZATION={}", self.trig_quantization_mode.value()?).as_str());
        s.push_str("\r\n[/SAMPLE]");

        Ok(s)
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod test {

    #[test]
    fn test_parse_id_correct() {
        let mut hmap = std::collections::HashMap::new();
        hmap.insert("slot".to_string(), "1".to_string());

        let slot_id = crate::projects::slots::parse_id(&hmap);

        assert_eq!(1, slot_id.unwrap());
    }

    #[test]
    fn test_parse_id_err_bad_value_type_err() {
        let mut hmap = std::collections::HashMap::new();
        hmap.insert("slot".to_string(), "AAAA".to_string());
        let slot_id = crate::projects::slots::parse_id(&hmap);
        assert!(slot_id.is_err());
    }

    #[test]
    fn test_parse_tempo_correct_default() {
        let mut hmap = std::collections::HashMap::new();
        hmap.insert("bpm".to_string(), "2880".to_string());
        let r = crate::projects::slots::parse_tempo(&hmap);
        assert_eq!(120_u16, r.unwrap());
    }

    #[test]
    fn test_parse_tempo_correct_min() {
        let mut hmap = std::collections::HashMap::new();
        hmap.insert("bpm".to_string(), "720".to_string());
        let r = crate::projects::slots::parse_tempo(&hmap);
        assert_eq!(30_u16, r.unwrap());
    }

    #[test]
    fn test_parse_tempo_correct_max() {
        let mut hmap = std::collections::HashMap::new();
        hmap.insert("bpm".to_string(), "7200".to_string());
        let r = crate::projects::slots::parse_tempo(&hmap);
        assert_eq!(300_u16, r.unwrap());
    }

    #[test]
    fn test_parse_tempo_bad_value_type_default_return() {
        let mut hmap = std::collections::HashMap::new();
        hmap.insert("bpm".to_string(), "AAAFSFSFSSFfssafAA".to_string());
        let r = crate::projects::slots::parse_tempo(&hmap);
        assert_eq!(r.unwrap(), 120_u16);
    }

    #[test]
    fn test_parse_gain_correct() {
        let mut hmap = std::collections::HashMap::new();
        hmap.insert("gain".to_string(), "72".to_string());
        let r = crate::projects::slots::parse_gain(&hmap);
        assert_eq!(24_i8, r.unwrap());
    }

    #[test]
    fn test_parse_gain_bad_value_type_default_return() {
        let mut hmap = std::collections::HashMap::new();
        hmap.insert("gain".to_string(), "AAAFSFSFSSFfssafAA".to_string());
        let r = crate::projects::slots::parse_gain(&hmap);
        assert_eq!(r.unwrap(), 0_i8);
    }

    #[test]
    fn test_parse_trim_bars_correct() {
        let mut hmap = std::collections::HashMap::new();
        hmap.insert("trim_barsx100".to_string(), "100".to_string());
        let r = crate::projects::slots::parse_trim_bars(&hmap);
        assert_eq!(100, r.unwrap());
    }

    #[test]
    fn test_parse_trim_bars_bad_value_type_default_return() {
        let mut hmap = std::collections::HashMap::new();
        hmap.insert(
            "trim_barsx100".to_string(),
            "AAAFSFSFSSFfssafAA".to_string(),
        );
        let r = crate::projects::slots::parse_trim_bars(&hmap);
        assert_eq!(r.unwrap(), 0);
    }

    #[test]
    fn test_parse_loop_mode_correct_off() {
        let mut hmap = std::collections::HashMap::new();
        hmap.insert("loopmode".to_string(), "0".to_string());
        let r = crate::projects::slots::parse_loop_mode(&hmap);
        assert_eq!(
            r.unwrap(),
            crate::samples::options::SampleAttributeLoopMode::Off
        );
    }

    #[test]
    fn test_parse_loop_mode_correct_normal() {
        let mut hmap = std::collections::HashMap::new();
        hmap.insert("loopmode".to_string(), "1".to_string());
        let r = crate::projects::slots::parse_loop_mode(&hmap);
        assert_eq!(
            r.unwrap(),
            crate::samples::options::SampleAttributeLoopMode::Normal
        );
    }

    #[test]
    fn test_parse_loop_mode_correct_pingpong() {
        let mut hmap = std::collections::HashMap::new();
        hmap.insert("loopmode".to_string(), "2".to_string());
        let r = crate::projects::slots::parse_loop_mode(&hmap);
        assert_eq!(
            r.unwrap(),
            crate::samples::options::SampleAttributeLoopMode::PingPong
        );
    }

    #[test]
    fn test_parse_loop_mode_bad_value_type_default_return() {
        let mut hmap = std::collections::HashMap::new();
        hmap.insert("loopmode".to_string(), "AAAFSFSFSSFfssafAA".to_string());
        let r = crate::projects::slots::parse_loop_mode(&hmap);
        assert_eq!(
            r.unwrap(),
            crate::samples::options::SampleAttributeLoopMode::Off
        );
    }

    #[test]
    fn test_parse_tstretch_correct_off() {
        let mut hmap = std::collections::HashMap::new();
        hmap.insert("tsmode".to_string(), "0".to_string());
        let r = crate::projects::slots::parse_tstrech_mode(&hmap);
        assert_eq!(
            crate::samples::options::SampleAttributeTimestrechMode::Off,
            r.unwrap()
        );
    }

    #[test]
    fn test_parse_tstretch_correct_normal() {
        let mut hmap = std::collections::HashMap::new();
        hmap.insert("tsmode".to_string(), "2".to_string());
        let r = crate::projects::slots::parse_tstrech_mode(&hmap);
        assert_eq!(
            crate::samples::options::SampleAttributeTimestrechMode::Normal,
            r.unwrap()
        );
    }

    #[test]
    fn test_parse_tstretch_correct_beat() {
        let mut hmap = std::collections::HashMap::new();
        hmap.insert("tsmode".to_string(), "3".to_string());
        let r = crate::projects::slots::parse_tstrech_mode(&hmap);
        assert_eq!(
            crate::samples::options::SampleAttributeTimestrechMode::Beat,
            r.unwrap()
        );
    }

    #[test]
    fn test_parse_tstretch_bad_value_type_default_return() {
        let mut hmap = std::collections::HashMap::new();
        hmap.insert("tsmode".to_string(), "AAAFSFSFSSFfssafAA".to_string());
        let r = crate::projects::slots::parse_tstrech_mode(&hmap);
        assert_eq!(
            r.unwrap(),
            crate::samples::options::SampleAttributeTimestrechMode::Off
        );
    }

    #[test]
    fn test_parse_tquantize_correct_off() {
        let mut hmap = std::collections::HashMap::new();
        hmap.insert("trigquantization".to_string(), "255".to_string());
        let r = crate::projects::slots::parse_trig_quantize_mode(&hmap);
        assert_eq!(
            crate::samples::options::SampleAttributeTrigQuantizationMode::Direct,
            r.unwrap()
        );
    }

    #[test]
    fn test_parse_tquantize_correct_direct() {
        let mut hmap = std::collections::HashMap::new();
        hmap.insert("trigquantization".to_string(), "0".to_string());
        let r = crate::projects::slots::parse_trig_quantize_mode(&hmap);
        assert_eq!(
            crate::samples::options::SampleAttributeTrigQuantizationMode::PatternLength,
            r.unwrap()
        );
    }

    #[test]
    fn test_parse_tquantize_correct_onestep() {
        let mut hmap = std::collections::HashMap::new();
        hmap.insert("trigquantization".to_string(), "1".to_string());
        let r = crate::projects::slots::parse_trig_quantize_mode(&hmap);
        assert_eq!(
            crate::samples::options::SampleAttributeTrigQuantizationMode::OneStep,
            r.unwrap()
        );
    }

    #[test]
    fn test_parse_tquantize_correct_twostep() {
        let mut hmap = std::collections::HashMap::new();
        hmap.insert("trigquantization".to_string(), "2".to_string());
        let r = crate::projects::slots::parse_trig_quantize_mode(&hmap);
        assert_eq!(
            crate::samples::options::SampleAttributeTrigQuantizationMode::TwoSteps,
            r.unwrap()
        );
    }

    #[test]
    fn test_parse_tquantize_correct_threestep() {
        let mut hmap = std::collections::HashMap::new();
        hmap.insert("trigquantization".to_string(), "3".to_string());
        let r = crate::projects::slots::parse_trig_quantize_mode(&hmap);
        assert_eq!(
            crate::samples::options::SampleAttributeTrigQuantizationMode::ThreeSteps,
            r.unwrap()
        );
    }

    #[test]
    fn test_parse_tquantize_correct_fourstep() {
        let mut hmap = std::collections::HashMap::new();
        hmap.insert("trigquantization".to_string(), "4".to_string());
        let r = crate::projects::slots::parse_trig_quantize_mode(&hmap);
        assert_eq!(
            crate::samples::options::SampleAttributeTrigQuantizationMode::FourSteps,
            r.unwrap()
        );
    }

    // i'm not going to test every single option. we do that already elsewhere.

    #[test]
    fn test_parse_tquantize_bad_value_type_default_return() {
        let mut hmap = std::collections::HashMap::new();
        hmap.insert(
            "trigquantization".to_string(),
            "AAAFSFSFSSFfssafAA".to_string(),
        );
        let r = crate::projects::slots::parse_trig_quantize_mode(&hmap);
        assert_eq!(
            r.unwrap(),
            crate::samples::options::SampleAttributeTrigQuantizationMode::default()
        );
    }
}
