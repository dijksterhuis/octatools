//! Functions to list specific data.

use std::path::PathBuf;

use serde_octatrack::{
    common::{FromFileAtPathBuf, RBoxErr},
    projects::{options::ProjectSampleSlotType, Project},
};

/// List all the sample slots within an Octatrack Project, given a path to a Project data file
pub fn list_project_sample_slots(path: &PathBuf) -> RBoxErr<()> {
    let project = Project::from_pathbuf(&path)?;

    let static_slots = project
        .slots
        .iter()
        .filter(|x| x.sample_type == ProjectSampleSlotType::Static);
    let flex_slots = project
        .slots
        .iter()
        .filter(|x| x.sample_type == ProjectSampleSlotType::Flex);
    // ignore recorder slots that don't have a flex sample assigned to them
    let recorder_slots = project.slots.iter().filter(|x| {
        x.sample_type == ProjectSampleSlotType::RecorderBuffer
            && x.path.to_str().unwrap_or("") != ""
    });

    if static_slots.clone().count() > 0 {
        for slot in static_slots {
            println!("{:#?}", slot);
        }
    }

    if flex_slots.clone().count() > 0 {
        for slot in flex_slots {
            println!("{:#?}", slot);
        }
    }

    if recorder_slots.clone().count() > 0 {
        for slot in recorder_slots {
            println!("{:#?}", slot);
        }
    }

    Ok(())
}
