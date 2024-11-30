//! Dumping binary data files to YAML files

use std::path::Path;

use crate::common::RBoxErr;
use serde_octatrack::{
    arrangements::ArrangementFile, banks::Bank, projects::Project, samples::SampleAttributes,
    FromPath, ToYamlFile,
};

/// Dump Bank file data to a YAML file
pub fn dump_bank(bank_path: &Path, yaml_path: &Path) -> RBoxErr<()> {
    let b = Bank::from_path(bank_path).expect("Could not load bank file");
    let _ = b.to_yaml(yaml_path);
    Ok(())
}

/// Dump Project file data to a YAML file
pub fn dump_project(path: &Path, yaml_path: &Path) -> RBoxErr<()> {
    let b = Project::from_path(path).expect("Could not load project file");
    let _ = b.to_yaml(yaml_path);
    Ok(())
}

/// Dump Sample Attributes file data to a YAML file
pub fn dump_ot_file(path: &Path, yaml_path: &Path) -> RBoxErr<()> {
    let b = SampleAttributes::from_path(path).expect("Could not load ot file");
    let _ = b.to_yaml(yaml_path);
    Ok(())
}

/// Dump Arrangement file data to a YAML file
pub fn dump_arrangement(path: &Path, yaml_path: &Path) -> RBoxErr<()> {
    unimplemented!("Need to deal with intermediate struct conversions.")
}
