//! Functions to list specific data.

use std::path::PathBuf;

use crate::common::RBoxErr;
use itertools::Itertools;
use serde_octatrack::{projects::Project, FromPathBuf};

/// List all the sample slots within an Octatrack Project, given a path to a Project data file
pub fn list_project_sample_slots(path: &PathBuf) -> RBoxErr<()> {
    for slot in Project::from_pathbuf(path)
        .expect("Could not load project file")
        .slots
        .iter()
        .sorted_by(|x, y| Ord::cmp(&x.slot_id, &y.slot_id))
    {
        println!("{:#?}", slot);
    }

    Ok(())
}
