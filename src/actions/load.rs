//! Creating binary data files from YAML files

use std::path::Path;

use crate::common::RBoxErr;
use serde_octatrack::{
    arrangements::ArrangementFile, banks::Bank, projects::Project, samples::SampleAttributes,
    FromYamlFile, ToPath,
};

/// Load binary file data from a YAML file
pub fn load_from_yaml<T: FromYamlFile + ToPath>(yaml_path: &Path, outfile: &Path) -> RBoxErr<()> {
    let b = T::from_yaml(yaml_path).expect("Could not load YAML file");
    b.to_path(outfile).expect("Could not write data to file");
    Ok(())
}

/// Load Bank file data from a YAML file
pub fn load_bank(yaml_path: &Path, outfile: &Path) -> RBoxErr<()> {
    load_from_yaml::<Bank>(yaml_path, outfile)
}

/// Load Project file data from a YAML file
pub fn load_project(yaml_path: &Path, outfile: &Path) -> RBoxErr<()> {
    load_from_yaml::<Project>(yaml_path, outfile)
}

/// Load Sample Attributes file data from a YAML file
pub fn load_ot_file(yaml_path: &Path, outfile: &Path) -> RBoxErr<()> {
    load_from_yaml::<SampleAttributes>(yaml_path, outfile)
}

/// Load Arrangement file data from a YAML file
pub fn load_arrangement(yaml_path: &Path, outfile: &Path) -> RBoxErr<()> {
    unimplemented!("Need to deal with intermediate struct conversions.")
}
