use serde_octatrack::{projects::Project, FromPath, ToYamlFile};

use crate::{actions::load_from_yaml, common::RBoxErr};
use itertools::Itertools;
use std::path::{Path, PathBuf};

/// Show deserialised representation of a Project for a given project file at `path`
pub fn show_project(path: &PathBuf) -> RBoxErr<()> {
    let b = Project::from_path(path).expect("Could not load project file");
    println!("{b:#?}");
    Ok(())
}

/// List all the sample slots within an Octatrack Project, given a path to a Project data file
pub fn list_project_sample_slots(path: &PathBuf) -> RBoxErr<()> {
    for slot in Project::from_path(path)
        .expect("Could not load project file")
        .slots
        .iter()
        .sorted_by(|x, y| Ord::cmp(&x.slot_id, &y.slot_id))
    {
        println!("{:#?}", slot);
    }

    Ok(())
}

/// Load Project file data from a YAML file
pub fn load_project(yaml_path: &Path, outfile: &Path) -> RBoxErr<()> {
    load_from_yaml::<Project>(yaml_path, outfile)
}

/// Dump Project file data to a YAML file
pub fn dump_project(path: &Path, yaml_path: &Path) -> RBoxErr<()> {
    let b = Project::from_path(path).expect("Could not load project file");
    let _ = b.to_yaml(yaml_path);
    Ok(())
}
