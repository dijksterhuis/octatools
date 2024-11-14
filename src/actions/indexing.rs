//! Functions for generating YAML 'index' files from the CLI.

mod yaml;

use log::{debug, trace};

use std::path::PathBuf;

use serde_octatrack::common::{FromFileAtPathBuf, RBoxErr};

use crate::common::ToYamlFile;
use yaml::cfcard::CompactFlashDrive;
use yaml::samplesdir::{SamplesDirIndexFull, SamplesDirIndexSimple};

pub fn create_index_compact_flash_drive_yaml(
    cfcard_dir_path: &PathBuf,
    yaml_file_path: &Option<PathBuf>,
) -> RBoxErr<()> {
    debug!("Indexing CF card: path={cfcard_dir_path:#?}");
    let cf = CompactFlashDrive::from_pathbuf(cfcard_dir_path)
        .expect(format!("Failed to create CF card index: path={cfcard_dir_path:#?}").as_str());

    // todo: as_ref usage? seems to be due to using an option in CLI definitions
    if !yaml_file_path.is_none() {
        let yml = yaml_file_path
            .as_ref()
            .expect("No option provided, cannot write to None!");
        debug!("Writing CF card index to yaml file: path={yml:#?}");
        let _ = cf.to_yaml(yml);
    };

    Ok(())
}

pub fn create_index_samples_dir_simple(
    samples_dir_path: &PathBuf,
    yaml_file_path: &Option<PathBuf>,
) -> RBoxErr<()> {
    debug!("Indexing samples directory with 'simple' output: path={samples_dir_path:#?}");
    let sample_index = SamplesDirIndexSimple::new(samples_dir_path)
        .expect(format!("Failed to create SamplesDir index: path={samples_dir_path:#?}").as_str());

    // TODO: clone
    if !yaml_file_path.is_none() {
        let yml = yaml_file_path
            .as_ref()
            .expect("No option provided, cannot write to None!");

        debug!("Writing SamplesDir index to yaml file: path={yml:#?}");
        let _ = sample_index.to_yaml(yml);
    };

    Ok(())
}

pub fn create_index_samples_dir_full(
    samples_dir_path: &PathBuf,
    yaml_file_path: &Option<PathBuf>,
) -> RBoxErr<()> {
    let sample_index = SamplesDirIndexFull::new(samples_dir_path)
        .expect(format!("Failed to create SamplesDir index: path={samples_dir_path:#?}").as_str());

    // TODO: clone
    if !yaml_file_path.is_none() {
        let yml = yaml_file_path
            .as_ref()
            .expect("No option provided, cannot write to None!");

        debug!("Writing SamplesDir index to yaml file: path={yml:#?}");
        let _ = sample_index.to_yaml(yml);
    }
    Ok(())
}
