//! Read/Write a YAML file config related to Octatrack compatible sample chains.
//! Reading a config and creating a sample chain is currently implemented.

use octatools_derive::Decodeable;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// YAML section determining the input/output files for an individual sample chain.

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YamlCopyDetails {
    pub project: PathBuf,
    pub bank: PathBuf,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YamlCopyBankDetails {
    pub src: YamlCopyDetails,
    pub dest: YamlCopyDetails,
}

#[derive(Debug, Serialize, Deserialize, Clone, Decodeable)]
pub struct YamlCopyBankConfig {
    pub bank_copies: Vec<YamlCopyBankDetails>,
}
