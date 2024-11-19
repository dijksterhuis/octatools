//! Module for commonly used traits/functions/structs etc.

use log::{debug, info};
use serde::{Deserialize, Serialize};
use serde_yml::Error as SerdeYmlError;
use std::path::PathBuf;

use std::error::Error;

pub type RBoxErr<T> = Result<T, Box<dyn Error>>;
pub type RVoidError<T> = Result<T, ()>;

#[derive(Debug)]
pub enum OctatoolErrors {
    PathNotADirectory,
    Unknown,
}
impl std::fmt::Display for OctatoolErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::PathNotADirectory => write!(f, "pathbuf is not a directory"),
            Self::Unknown => write!(f, "unknown error (please investigate/report)"),
        }
    }
}
impl std::error::Error for OctatoolErrors {}

pub trait ToYamlFile
where
    Self: Serialize,
{
    fn to_yaml(&self, yaml_file_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        debug!("Writing data to YAML file: {:#?}", yaml_file_path);

        debug!("Opened file: {:#?}", yaml_file_path);

        serde_yml::to_writer(
            std::fs::File::options()
                .read(true)
                .write(true)
                .create_new(true)
                .open(yaml_file_path)
                .unwrap(),
            self,
        )?;
        debug!("Write file: {:#?}", yaml_file_path);

        info!("Wrote data to YAML file: {:#?}", yaml_file_path);
        Ok(())
    }
}

pub trait FromYamlFile
where
    Self: for<'a> Deserialize<'a>,
{
    fn from_yaml(yaml_file_path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        debug!("Reading YAML file: {:#?}", yaml_file_path);

        let f = std::fs::File::open(yaml_file_path)?;
        let data: Result<Self, SerdeYmlError> = serde_yml::from_reader(f);

        info!("Read YAML file: {:#?}", yaml_file_path);

        Ok(data?)
    }
}
