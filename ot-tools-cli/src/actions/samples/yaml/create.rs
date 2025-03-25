//! Read/Write a YAML file config related to Octatrack compatible sample chains.
//! Reading a config and creating a sample chain is currently implemented.

use crate::actions::samples::{FileFormatOpts, SampleChainOpts, SliceProcOpts};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// YAML section which globally affects all chains being created with the loaded config.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YamlChainCreateGlobalSettings {
    /// directory path to write output chain files to
    pub out_dir_path: PathBuf,
}

/// YAML section determining the input/output files for an individual sample chain.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YamlChainCreateSampleChains {
    /// The base file name of the output file pair
    pub chain_name: String,
    /// Options that can be provided for global octatrack chain settings
    pub octatrack_settings: Option<SampleChainOpts>,
    /// Per-slice audio modifications to apply
    pub audio_processing: Option<SliceProcOpts>,
    /// Output file format: 16-bit WAV / 24-bit WAV
    pub audio_format: Option<FileFormatOpts>,
    pub audio_file_paths: Vec<PathBuf>,
}

/// A parsed YAML config for a single YAML file.

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YamlChainCreate {
    pub global_settings: YamlChainCreateGlobalSettings,
    pub chains: Vec<YamlChainCreateSampleChains>,
}
