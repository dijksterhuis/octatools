//! Functions for generating YAML 'index' files from the CLI.

mod yaml;

use crate::RBoxErr;
use log::debug;
use serde_octatrack::{FromPath, ToYamlFile};
use std::path::PathBuf;
use yaml::cfcard::CompactFlashDrive;

pub fn create_file_index_yaml(
    cfcard_dir_path: &PathBuf,
    yaml_file_path: &Option<PathBuf>,
) -> RBoxErr<()> {
    debug!("Indexing CF card: path={cfcard_dir_path:#?}");
    let cf = CompactFlashDrive::from_path(cfcard_dir_path)?;

    if !yaml_file_path.is_none() {
        let _ = cf.to_yaml(&yaml_file_path.as_ref().unwrap());
    };

    Ok(())
}

#[cfg(test)]
mod test {
    use serde_octatrack::FromYamlFile;

    use super::*;

    #[test]
    fn test_drive_file_index_yaml_ok() {
        let indir = PathBuf::from("../data/tests/drive/DEMO-DRIVE-DATA/");
        let res = create_file_index_yaml(&indir, &None);
        println!("HELP {:#?}", res);
        assert!(res.is_ok());
    }

    #[test]
    fn test_drive_file_index_yaml_correct() {
        let indir = PathBuf::from("../data/tests/drive/DEMO-DRIVE-DATA/");
        let outyaml = PathBuf::from("/tmp/test-drive-file-inex-correctness.yaml");
        let testyaml = PathBuf::from("../data/tests/drive/test.yml");

        let _ = create_file_index_yaml(&indir, &Some(outyaml.clone()));

        let valid = CompactFlashDrive::from_yaml(&testyaml).unwrap();
        let written = CompactFlashDrive::from_yaml(&outyaml).unwrap();

        let _ = std::fs::remove_file(&outyaml);
        assert_eq!(written, valid);
    }
}
