//! A project's metadata, e.g. the OS Version.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    common::{FromString, ParseHashMapValueAs},
    projects::common::{string_to_hashmap, ProjectRawFileSection},
};

/// Project metadata read from a parsed Octatrack Project file
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ProjectMetadata {
    /// Type of file (always a 'project').
    ///
    /// Example ASCII data:
    /// ```text
    /// TYPE=OCTATRACK DPS-1 PROJECT
    /// ```
    pub filetype: String,

    /// Unknown. Probably refers to an internal Elektron release version number.
    ///
    /// Example ASCII data:
    /// ```text
    /// VERSION=19
    /// ```
    pub project_version: u32,

    /// Version of the Octatrack OS (that the project was created with?).
    ///
    /// Example ASCII data:
    /// ```text
    /// OS_VERSION=R0177     1.40B
    /// ```
    pub os_version: String,
}

impl ParseHashMapValueAs for ProjectMetadata {}

impl FromString for ProjectMetadata {
    type T = Self;

    /// Extract `OctatrackProjectMetadata` fields from the project file's ASCII data

    fn from_string(data: &String) -> Result<Self, Box<dyn std::error::Error>> {
        let hmap: HashMap<String, String> = string_to_hashmap(&data, &ProjectRawFileSection::Meta)?;

        Ok(Self {
            filetype: Self::parse_hashmap_value::<String>(&hmap, "type")?,
            project_version: Self::parse_hashmap_value::<u32>(&hmap, "version")?,
            os_version: Self::parse_hashmap_value::<String>(&hmap, "os_version")?,
        })
    }
}
