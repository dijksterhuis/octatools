//! A project's metadata, e.g. the OS Version.


use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::octatrack::common::{
    FromString,
    ParseHashMapValueAs,
};

use crate::octatrack::projects::common::{
    string_to_hashmap,
    ProjectRawFileSection,
};


/// Project metadata read from a parsed Octatrack Project file

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ProjectMetadata {

    /// Type of file (always a 'project').
    /// Example ASCII data: `TYPE=OCTATRACK DPS-1 PROJECT`
    filetype: String,

    /// Unknown.
    /// Example ASCII data: `VERSION=19`
    project_version: u32,

    /// Version of the Octatrack OS (that the project was created with?).
    /// Example ASCII data: `OS_VERSION=R0177     1.40B`
    os_version: String,
}

impl ParseHashMapValueAs for ProjectMetadata {}

impl FromString for ProjectMetadata {

    type T = Self;

    /// Extract `OctatrackProjectMetadata` fields from the project file's ASCII data 

    fn from_string(data: &String) -> Result<Self, Box<dyn std::error::Error>> {

        let hmap: HashMap<String, String> = string_to_hashmap(&data, &ProjectRawFileSection::Meta)?;

        Ok(
            Self {
                filetype: Self::parse_hashmap_value::<String>(&hmap, "type")?,
                project_version: Self::parse_hashmap_value::<u32>(&hmap, "version")?,
                os_version: Self::parse_hashmap_value::<String>(&hmap, "os_version")?,
            },
        )
    }
}