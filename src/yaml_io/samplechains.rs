//! Read/Write a YAML file config related to Octatrack compatible sample chains.
//! Reading a config and creating a sample chain is currently implemented.
//! TODO: Writing a new YAML config from an existing sample chain (edit existing chains via YAML).

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use serde_octatrack::samples::options::{
    SampleAttributeLoopMode, SampleAttributeTimestrechMode, SampleAttributeTrigQuantizationMode,
};

use crate::common::{FromYamlFile, ToYamlFile};

// TODO: normalization
// TODO: fades
// TODO: reverses?
// TODO: gated?
// TODO: etc.
// TODO: should normalization be per chain? separate settings struct?

/// YAML section which globally affects all chains being created with the loaded config.

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YamlChainConfigGlobalSettings {
    pub normalize: bool,
    pub out_dir_path: PathBuf,
}

// Deliberately does not include the trim / loop length settings
// as they are mostly irrelevant for creating sample chains
/// YAML section controlling an individual chain's Octatrack sample settings.

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YamlChainConfigChainsOctatrackSettings {
    pub bpm: f32,                                        // this will get multiplied by 24
    pub gain: f32,                                       // -24.0 <= x <= +24.0
    pub timestretch_mode: SampleAttributeTimestrechMode, // needs to be one of the enum values
    pub loop_mode: SampleAttributeLoopMode,
    pub quantization_mode: SampleAttributeTrigQuantizationMode,
}

/// YAML section determining the input/output files for an individual sample chain.

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YamlChainConfigSamplechain {
    pub octatrack_settings: YamlChainConfigChainsOctatrackSettings,
    pub sample_file_paths: Vec<PathBuf>,
    // use this for the .wav + .ot file names
    pub chain_name: String,
}

/// A parsed YAML config for a single YAML file.

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YamlChainConfig {
    pub global_settings: YamlChainConfigGlobalSettings,
    pub chains: Vec<YamlChainConfigSamplechain>,
}

impl FromYamlFile for YamlChainConfig {}
impl ToYamlFile for YamlChainConfig {}

//     /// Write data to a new YAML file.

//     pub fn to_yaml(self: &Self, yaml_file_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {

//         debug!("Writing SampleChain config to YAML file: {:#?}", &yaml_file_path);

//         let f = std::fs::File::open(yaml_file_path)?;
//         let written = serde_yml::to_writer(f, self)?;

//         info!("Write SampleChain config to YAML file: {:#?}", &yaml_file_path);

//         Ok(written)
//     }

//     /// **TODO**: Deconstruct existing samplechain file pairs into a YAML config file.
//     /// Useful for editing sample chains created on device.

//     pub fn from_samplechain_files(sample_files: &Vec<OctatrackSampleFilePair>, outdir: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
//         todo!()
//     }

//     /// Read yaml config from file.

//     pub fn from_yaml(yaml_file_path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {

//         debug!("Reading YAML config file: {:#?}", &yaml_file_path);

//         let f = std::fs::File::open(yaml_file_path)?;
//         let data: Result<Self, SerdeYmlError> = serde_yml::from_reader(f);

//         info!("Read YAML config file: {:#?}", &yaml_file_path);

//         Ok(data?)

//     }
// }
