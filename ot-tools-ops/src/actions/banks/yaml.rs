//! Read/Write a YAML file config related to Octatrack compatible sample chains.
//! Reading a config and creating a sample chain is currently implemented.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// YAML section determining the input/output files for an individual sample chain.

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YamlCopyDetails {
    pub project: PathBuf,
    pub bank_id: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YamlCopyBankDetails {
    pub src: YamlCopyDetails,
    pub dest: YamlCopyDetails,
    pub force: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YamlCopyBankConfig {
    pub bank_copies: Vec<YamlCopyBankDetails>,
}
