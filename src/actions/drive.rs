//! Functions for generating YAML 'index' files from the CLI.

mod yaml;

use crate::RBoxErr;
use log::debug;
use serde_octatrack::{FromPath, ToYamlFile};
use std::path::PathBuf;
use yaml::cfcard::CompactFlashDrive;

pub fn create_index_compact_flash_drive_yaml(
    cfcard_dir_path: &PathBuf,
    yaml_file_path: &Option<PathBuf>,
) -> RBoxErr<()> {
    debug!("Indexing CF card: path={cfcard_dir_path:#?}");
    let cf = CompactFlashDrive::from_path(cfcard_dir_path)
        .unwrap_or_else(|_| panic!("Failed to create CF card index: path={cfcard_dir_path:#?}"));

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
