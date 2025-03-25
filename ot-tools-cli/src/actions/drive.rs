//! Functions for generating YAML 'index' files from the CLI.

mod yaml;
use ot_tools_lib::type_to_yaml_file;

use crate::RBoxErr;
use log::debug;
use std::path::PathBuf;
use yaml::cfcard::CompactFlashDrive;

pub fn create_file_index_yaml(
    cfcard_dir_path: &PathBuf,
    yaml_file_path: &Option<PathBuf>,
) -> RBoxErr<()> {
    debug!("Indexing CF card: path={cfcard_dir_path:#?}");
    let cf = CompactFlashDrive::from_path(cfcard_dir_path)?;

    if !yaml_file_path.is_none() {
        type_to_yaml_file(&cf, yaml_file_path.as_ref().unwrap())?;
    };

    Ok(())
}

#[cfg(test)]
#[allow(unused_imports)]
mod test {
    use super::*;
    use ot_tools_lib::yaml_file_to_type;

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
        let outyaml = std::env::temp_dir().join("test-drive-file-inex-correctness.yaml");
        let testyaml = PathBuf::from("../data/tests/drive/test.yml");

        let _ = create_file_index_yaml(&indir, &Some(outyaml.clone()));

        let valid = yaml_file_to_type::<CompactFlashDrive>(&testyaml).unwrap();
        let written = yaml_file_to_type::<CompactFlashDrive>(&outyaml).unwrap();

        let _ = std::fs::remove_file(&outyaml);
        assert_eq!(written, valid);
    }
}
