//! Read/Write a YAML file config related to Octatrack compatible sample chains.
//! Reading a config and creating a sample chain is currently implemented.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use serde_octatrack::samples::options::{
    SampleAttributeLoopMode, SampleAttributeTimestrechMode, SampleAttributeTrigQuantizationMode,
};

use serde_octatrack::Decode;

/// YAML section which globally affects all chains being created with the loaded config.

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YamlChainCreateGlobalSettings {
    pub normalize: bool,
    pub out_dir_path: PathBuf,
}

// Deliberately does not include the trim / loop length settings
// as they are mostly irrelevant for creating sample chains
/// YAML section controlling an individual chain's Octatrack sample settings.

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YamlChainCreateOctatrackSettings {
    pub bpm: f32,                                        // this will get multiplied by 24
    pub gain: f32,                                       // -24.0 <= x <= +24.0
    pub timestretch_mode: SampleAttributeTimestrechMode, // needs to be one of the enum values
    pub loop_mode: SampleAttributeLoopMode,
    pub quantization_mode: SampleAttributeTrigQuantizationMode,
}

/// YAML section determining the input/output files for an individual sample chain.

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YamlChainCreateSamplechains {
    pub octatrack_settings: YamlChainCreateOctatrackSettings,
    pub sample_file_paths: Vec<PathBuf>,
    // use this for the .wav + .ot file names
    pub chain_name: String,
}

/// A parsed YAML config for a single YAML file.

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YamlChainCreate {
    pub global_settings: YamlChainCreateGlobalSettings,
    pub chains: Vec<YamlChainCreateSamplechains>,
}

impl Decode for YamlChainCreate {}
