//! Project sample slots read from a parsed Octatrack Project file.
//! This only loads data from the project file.
//! Samples not added to a project sample lsit for sstatic/flex machines will not be loaded.
//! **NOTE**: any fields matching those in an Octatrack sample attributes file
//! may not have been writtten to an attributes file yet.
//! (these are project files loaded into memory when switching to the project)/

// Example data:
// [SAMPLE]\r\nTYPE=FLEX\r\nSLOT=001\r\nPATH=../AUDIO/flex.wav\r\nTRIM_BARSx100=173\r\nTSMODE=2\r\nLOOPMODE=1\r\nGAIN=48\r\nTRIGQUANTIZATION=-1\r\n[/SAMPLE]

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, convert::TryFrom, error::Error, path::PathBuf, str::FromStr};

use crate::{
    common::{
        FromHashMap, OptionEnumValueConvert, ParseHashMapValueAs, ProjectFromString,
        ProjectToString,
    },
    projects::options::ProjectSampleSlotType,
    samples::options::{
        SampleAttributeLoopMode, SampleAttributeTimestrechMode, SampleAttributeTrigQuantizationMode,
    },
    // utils::SampleFilePair,
};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ProjectSampleSlot {
    // TODO: Should recording buffers be treated as a separate slot type?
    /// Type of sample: STATIC or FLEX
    pub sample_type: ProjectSampleSlotType,

    /// String ID Number of the slot the sample is assigned to e.g. 001, 002, 003...
    /// Maximum of 128 entries for STATIC sample slots, but can be up to 136 for flex
    /// slots as there are 8 recorders + 128 flex slots.
    pub slot_id: u16,

    /// Relative path to the file on the card from the project directory.
    pub path: PathBuf,

    // /// The sample's file pair (audio file and optional attributes file).
    // pub file_pair: Option<SampleFilePair>,

    // TODO: This is optional -- not used for recording buffer 'flex' tracks
    /// Current bar trim (float). This is multiplied by 100 on the machine.
    pub trim_bars: f32,

    /// Current `SampleTimestrechModes` setting for the specific slot. Example: `TSMODE=2`
    pub timestrech_mode: SampleAttributeTimestrechMode,

    /// Current `SampleLoopModes` setting for the specific slot.
    pub loop_mode: SampleAttributeLoopMode,

    // TODO: This is optional -- not used for recording buffer 'flex' tracks
    /// Current `SampleTrigQuantizationModes` setting for this specific slot.
    pub trig_quantization_mode: SampleAttributeTrigQuantizationMode,

    // TODO: Need to scale this to -24.0 dB <= x <= 24.0 dB
    /// Sample gain. 48 is default as per sample attributes file. maximum 96, minimum 0.
    pub gain: u8,

    // TODO: Need to scale this down by 24.
    /// BPM of the sample in this slot.
    pub bpm: u16,
}

impl ParseHashMapValueAs for ProjectSampleSlot {}

// cannot use FromProjectStringData because it expects a lone Self result, rather than a Vec.
impl FromHashMap for ProjectSampleSlot {
    type A = String;
    type B = String;
    type T = ProjectSampleSlot;

    fn from_hashmap(hmap: &HashMap<Self::A, Self::B>) -> Result<Self::T, Box<dyn Error>> {
        // Flex Sample slots with ID > 128 are recording buffers
        // TODO: Make this part of the ProjectSampleSlotType from_value method?
        let mut sample_slot_type = hmap.get("type").unwrap().clone();
        let slot_id = Self::parse_hashmap_value::<u16>(&hmap, "slot")?;

        println!("{sample_slot_type:#?}");

        if sample_slot_type == "FLEX" && slot_id > 129 {
            sample_slot_type = "RECORDER".to_string();
        }

        let sample_type = ProjectSampleSlotType::from_value(sample_slot_type).unwrap();

        // let slot_id = hmap.get("slot").unwrap().clone().parse::<u16>().unwrap();

        let path = PathBuf::from_str(hmap.get("path").unwrap()).unwrap();

        // TODO: Will never find the respective OT file as
        // the ^ path is alwys relative to project dir on CF card

        // let mut file_pair = None;
        // if path.file_name() != PathBuf::from("").file_name() {
        //     file_pair = Some(SampleFilePair::from_audio_pathbuf(&path).unwrap());
        // }

        let trim_bars = hmap
            .get("trim_barsx100")
            .unwrap_or(&"0.0".to_string())
            .clone()
            .parse::<f32>()
            .unwrap()
            / 100.0;

        let loop_mode = SampleAttributeLoopMode::from_value(
            hmap.get("loopmode")
                .unwrap()
                .clone()
                .parse::<u32>()
                .unwrap(),
        )
        .unwrap();

        let timestrech_mode = SampleAttributeTimestrechMode::from_value(
            hmap.get("tsmode").unwrap().clone().parse::<u32>().unwrap(),
        )
        .unwrap();

        let tq_i16: i16 = hmap
            .get("trigquantization")
            .unwrap()
            .clone()
            .parse::<i16>()
            .unwrap();

        let tq_u32: u32 = u32::try_from(tq_i16).unwrap_or(255_u32);

        let trig_quantization_mode =
            SampleAttributeTrigQuantizationMode::from_value(tq_u32).unwrap();

        let gain = hmap.get("gain").unwrap().clone().parse::<u8>().unwrap();

        let bpm = hmap
            .get("bpm")
            .unwrap_or(&"2880".to_string())
            .clone()
            .parse::<u16>()
            .unwrap_or(2880)
            / 24_u16;

        let sample_struct = Self {
            sample_type,
            slot_id,
            path,
            // file_pair,
            trim_bars,
            timestrech_mode,
            loop_mode,
            trig_quantization_mode,
            gain,
            // bpm: hmap.get("bpm").unwrap().clone().parse::<u16>().unwrap(),
            bpm,
        };
        println!("New slot ead: {:#?}", sample_struct);

        Ok(sample_struct)
    }
}

impl ProjectFromString for ProjectSampleSlot {
    type T = Vec<Self>;

    /// Load project 'samples' data from the raw project ASCII file.
    fn from_string(data: &String) -> Result<Vec<Self>, Box<dyn std::error::Error>> {
        let footer_stripped = data
            .strip_suffix("\r\n\r\n############################\r\n\r\n")
            .unwrap();

        let data_window: Vec<&str> = footer_stripped
            .split("############################\r\n# Samples\r\n############################")
            .into_iter()
            .collect();

        let mut samples_string: Vec<&str> = data_window[1].split("[/SAMPLE]").into_iter().collect();

        // last one is always a blank string.
        samples_string.pop();

        let samples: Vec<Vec<Vec<&str>>> = samples_string
            .into_iter()
            .map(|sample: &str| {
                sample
                    .strip_prefix("\r\n\r\n[SAMPLE]\r\n")
                    .unwrap()
                    .strip_suffix("\r\n")
                    .unwrap()
                    .split("\r\n")
                    .into_iter()
                    .map(|x: &str| x.split("=").into_iter().collect_vec())
                    .filter(|x: &Vec<&str>| x.len() == 2)
                    .collect_vec()
            })
            .collect();

        println!("samples: {samples:#?}");

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

    fn to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        // Recording buffers are actually stored as FLEX slots with
        // a slot ID > 128.
        let sample_type = match self.sample_type {
            ProjectSampleSlotType::Static | ProjectSampleSlotType::Flex => {
                self.sample_type.value().unwrap()
            }
            ProjectSampleSlotType::RecorderBuffer => "FLEX".to_string(),
        };

        let mut s = "[SAMPLE]\r\n".to_string();
        s.push_str(format!("TYPE={}", sample_type).as_str());
        s.push_str("\r\n");
        s.push_str(format!("SLOT={}", self.slot_id).as_str());
        s.push_str("\r\n");
        s.push_str(format!("PATH={:#?}", self.path).replace("\"", "").as_str());
        s.push_str("\r\n");
        s.push_str(format!("TRIM_BARSx100={}", (self.trim_bars * 100.0) as u16).as_str());
        s.push_str("\r\n");
        s.push_str(format!("TSMODE={}", self.timestrech_mode.value().unwrap()).as_str());
        s.push_str("\r\n");
        s.push_str(format!("LOOPMODE={}", self.loop_mode.value().unwrap()).as_str());
        s.push_str("\r\n");
        s.push_str(format!("GAIN={}", self.gain).as_str());
        s.push_str("\r\n");
        s.push_str(
            format!(
                "TRIGQUANTIZATION={}",
                self.trig_quantization_mode.value().unwrap()
            )
            .as_str(),
        );
        s.push_str("\r\n[/SAMPLE]");

        Ok(s)
    }
}
