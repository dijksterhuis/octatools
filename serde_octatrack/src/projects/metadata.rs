//! A project's metadata, e.g. the OS Version.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::projects::{
    parse_hashmap_string_value, string_to_hashmap, ProjectFromString, ProjectRawFileSection,
    ProjectToString,
};

/*
Example data:
[META]\r\nTYPE=OCTATRACK DPS-1 PROJECT\r\nVERSION=19\r\nOS_VERSION=R0177     1.40B\r\n[/META]
------
[META]
TYPE=OCTATRACK DPS-1 PROJECT
VERSION=19
OS_VERSION=R0177     1.40B
[/META]
*/

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

impl Default for ProjectMetadata {
    fn default() -> Self {
        Self {
            filetype: "OCTATRACK DPS-1 PROJECT".to_string(),
            project_version: 19,
            os_version: "R0177     1.40B".to_string(),
        }
    }
}

impl ProjectFromString for ProjectMetadata {
    type T = Self;

    /// Extract `OctatrackProjectMetadata` fields from the project file's ASCII data
    fn from_string(data: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let hmap: HashMap<String, String> = string_to_hashmap(data, &ProjectRawFileSection::Meta)?;

        Ok(Self {
            filetype: parse_hashmap_string_value::<String>(&hmap, "type", None)?,
            project_version: parse_hashmap_string_value::<u32>(&hmap, "version", None)?,
            os_version: parse_hashmap_string_value::<String>(&hmap, "os_version", None)?,
        })
    }
}

impl ProjectToString for ProjectMetadata {
    /// Extract `OctatrackProjectMetadata` fields from the project file's ASCII data
    fn to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut s = "".to_string();
        s.push_str("[META]\r\n");
        s.push_str(format!("TYPE={}", self.filetype).as_str());
        s.push_str("\r\n");
        s.push_str(format!("VERSION={}", self.project_version).as_str());
        s.push_str("\r\n");
        s.push_str(format!("OS_VERSION={}", self.os_version).as_str());
        s.push_str("\r\n[/META]");

        Ok(s)
    }
}
