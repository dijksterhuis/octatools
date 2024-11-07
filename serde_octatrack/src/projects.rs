//! Parse Octatrack `project.*` data files.

pub mod common;
pub mod metadata;
pub mod settings;
pub mod slots;
pub mod states;
pub mod options;

use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::path::PathBuf;

use crate::common::{FromFileAtPathBuf, FromString, RBoxErr, RVoidError, ToFileAtPathBuf};

use crate::projects::{
    metadata::ProjectMetadata, settings::ProjectSettings, slots::ProjectSampleSlots,
    states::ProjectStates,
};

// TODO: Move to some utils file
// TODO: Error type
fn get_pathbuf_fname_as_string(path: &PathBuf) -> RVoidError<String> {
    let name = path
        .clone()
        .file_name()
        .unwrap_or(&OsStr::new("err"))
        .to_str()
        .unwrap_or("err")
        .to_string();

    if name == "err" {
        return Err(());
    };
    Ok(name)
}


/// A parsed representation of an Octatrack Project file (`project.work` or `project.strd`).

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Project {
    /// Metadata key-value pairs from a Project file.
    pub metadata: ProjectMetadata,

    /// Settings key-value pairs from a Project file.
    pub settings: ProjectSettings,

    /// States key-value pairs from a Project file.
    pub states: ProjectStates,

    /// Slots key-value pairs from a Project file.
    pub slots: Vec<ProjectSampleSlots>,
}

impl FromFileAtPathBuf for Project {
    type T = Project;

    /// Read and parse an Octatrack project file (`project.work` or `project.strd`)
    fn from_pathbuf(path: PathBuf) -> RBoxErr<Self> {
        let s = std::fs::read_to_string(&path)?;

        let metadata = ProjectMetadata::from_string(&s)?;
        let states = ProjectStates::from_string(&s)?;
        let settings = ProjectSettings::from_string(&s)?;
        // TODO: Get sample file pairs, pop the ones that are active, the rest are inactive.
        let slots = ProjectSampleSlots::from_string(&s)?;

        Ok(Self {
            metadata,
            settings,
            states,
            slots,
        })
    }
}

impl ToFileAtPathBuf for Project {
    fn to_pathbuf(&self, path: PathBuf) -> RBoxErr<()> {
        todo!()
    }
}

#[cfg(test)]
mod test_integration {
    use super::*;

    #[test]
    fn test_read_a_project_work_file() {
        let test_file_pathbuf =
            PathBuf::from("data/tests/index-cf/DEV-OTsm/FLEX-ONESTRTEND/project.work");
        assert!(Project::from_pathbuf(test_file_pathbuf).is_ok());
    }

    #[test]
    fn test_read_a_project_strd_file() {
        let test_file_pathbuf =
            PathBuf::from("data/tests/index-cf/DEV-OTsm/FLEX-ONESTRTEND/project.strd");
        assert!(Project::from_pathbuf(test_file_pathbuf).is_ok());
    }
}
