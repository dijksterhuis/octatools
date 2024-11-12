//! Parse Octatrack `project.*` data files.

pub mod common;
pub mod metadata;
pub mod options;
pub mod settings;
pub mod slots;
pub mod states;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::common::{FromFileAtPathBuf, FromString, RBoxErr, ToFileAtPathBuf};

use crate::projects::{
    metadata::ProjectMetadata, options::ProjectSampleSlotType, settings::ProjectSettings,
    slots::ProjectSampleSlot, states::ProjectStates,
};

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
    pub slots: Vec<ProjectSampleSlot>,
}

impl Project {
    pub fn update_sample_slot_id(
        &mut self,
        old_slot_id: &u8,
        new_slot_id: &u8,
        sample_type: Option<ProjectSampleSlotType>,
    ) -> () {
        let type_filt = sample_type.unwrap_or(ProjectSampleSlotType::Static);

        let sample_slot_filt: Vec<ProjectSampleSlot> = self
            .slots
            .clone()
            .into_iter()
            .filter(|x| x.slot_id == *old_slot_id as u16 && x.sample_type == type_filt)
            .collect();

        // no samples assigned to slots
        if sample_slot_filt.len() > 0 {
            let mut sample_slot = sample_slot_filt[0].clone();

            sample_slot.slot_id = *new_slot_id as u16;
            self.slots[*old_slot_id as usize] = sample_slot;
        }
    }
}

impl FromFileAtPathBuf for Project {
    type T = Project;

    /// Read and parse an Octatrack project file (`project.work` or `project.strd`)
    fn from_pathbuf(path: PathBuf) -> RBoxErr<Self> {
        let s = std::fs::read_to_string(&path)?;

        let metadata = ProjectMetadata::from_string(&s)?;
        let states = ProjectStates::from_string(&s)?;
        let settings = ProjectSettings::from_string(&s)?;
        // todo? Get sample file pairs, pop the ones that are active, the rest are inactive.
        let slots = ProjectSampleSlot::from_string(&s)?;

        Ok(Self {
            metadata,
            settings,
            states,
            slots,
        })
    }
}

impl ToFileAtPathBuf for Project {
    #[allow(unused_variables)]
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
