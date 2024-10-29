//! Read/Write a YAML file config related to Octatrack compatible sample chains.
//! Reading a config and creating a sample chain is currently implemented.
//! TODO: Writing a new YAML config from an existing sample chain (edit existing chains via YAML).

use log::{debug, info};
use serde::{Deserialize, Serialize};
use serde_yml::Error as SerdeYmlError;
use std::path::PathBuf;

use crate::audio::wavfile::WavFile;

/// YAML section determining the input/output files for an individual sample chain.

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YamlSamplesDirSamples {
    pub file_paths: Vec<PathBuf>,
}

pub trait ToYamlFile
where
    Self: serde::Serialize,
{
    type T;
    fn to_yaml(self: &Self, yaml_file_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        debug!("Writing data to YAML file: {:#?}", &yaml_file_path);

        let f = std::fs::File::open(&yaml_file_path)?;
        let written = serde_yml::to_writer(f, self)?;

        info!("Wrote data to YAML file: {:#?}", &yaml_file_path);
        Ok(written)
    }
}

pub trait FromYamlFile
where
    Self: for<'a> serde::Deserialize<'a>,
{
    type T;
    fn from_yaml(yaml_file_path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        debug!("Reading YAML file: {:#?}", &yaml_file_path);

        let f = std::fs::File::open(&yaml_file_path)?;
        let data: Result<Self, SerdeYmlError> = serde_yml::from_reader(f);

        info!("Read YAML file: {:#?}", &yaml_file_path);

        Ok(data?)
    }
}

impl YamlSamplesDirSamples {
    pub fn new(file_paths: Vec<PathBuf>) -> Self {
        YamlSamplesDirSamples {
            file_paths: file_paths,
        }
    }

    pub fn from_pathbufs(pathbufs: Vec<PathBuf>) -> Self {
        Self::new(pathbufs)
    }

    pub fn from_wavfiles(wavfiles: Vec<WavFile>) -> Self {
        let file_paths: Vec<PathBuf> = wavfiles.into_iter().map(|x: WavFile| x.file_path).collect();

        Self::from_pathbufs(file_paths)
    }

    /// Write data to a new YAML file.

    pub fn to_yaml(
        self: &Self,
        yaml_file_path: &PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        debug!(
            "Writing SampleDir index of compatible WAV files to: {:#?}",
            &yaml_file_path
        );

        let f = std::fs::File::open(yaml_file_path)?;
        let written = serde_yml::to_writer(f, self)?;

        info!(
            "Write SampleChain config to YAML file: {:#?}",
            &yaml_file_path
        );

        Ok(written)
    }

    /// Read yaml config from file.

    pub fn from_yaml(yaml_file_path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        debug!("Reading YAML config file: {:#?}", &yaml_file_path);

        let f = std::fs::File::open(yaml_file_path)?;
        let data: Result<Self, SerdeYmlError> = serde_yml::from_reader(f);

        info!("Read YAML config file: {:#?}", &yaml_file_path);

        Ok(data?)
    }
}
