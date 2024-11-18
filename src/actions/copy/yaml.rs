//! Read/Write a YAML file config related to Octatrack compatible sample chains.
//! Reading a config and creating a sample chain is currently implemented.
//! TODO: Writing a new YAML config from an existing sample chain (edit existing chains via YAML).

use log::{debug, info};
use serde::{Deserialize, Serialize};
use serde_octatrack::FromPathBuf;
use serde_yml::Error as SerdeYmlError;
use std::path::PathBuf;

/// YAML section determining the input/output files for an individual sample chain.

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YamlCopyBankDetails {
    pub src: PathBuf,
    pub dest: PathBuf,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YamlCopyBankConfig {
    pub bank_copies: Vec<YamlCopyBankDetails>,
}

impl FromPathBuf for YamlCopyBankConfig {
    type T = Self;

    /// Read yaml config from file.

    fn from_pathbuf(path: &PathBuf) -> Result<Self::T, Box<dyn std::error::Error>> {
        debug!("Reading YAML config file: {:#?}", &path);

        let f = std::fs::File::open(path)?;
        let data: Result<Self, SerdeYmlError> = serde_yml::from_reader(f);

        info!("Read YAML config file: {:#?}", &path);

        Ok(data?)
    }
}
