//! Read a YAML file config to batch deconstruct sliced sample chains into constituent slice samples.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// YAML section which globally affects all chains being created with the loaded config.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YamlChainDeconstructConfigGlobalSettings {
    pub out_dir_path: PathBuf,
}

// Deliberately does not include the trim / loop length settings
// as they are mostly irrelevant for creating sample chains
/// YAML section controlling an individual chain's Octatrack sample settings.

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YamlChainDeconstructFilePairs {
    pub sample: PathBuf,
    pub otfile: PathBuf,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YamlChainDeconstruct {
    pub global_settings: YamlChainDeconstructConfigGlobalSettings,
    pub chains: Vec<YamlChainDeconstructFilePairs>,
}
