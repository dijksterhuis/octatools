//! # `ot-tools-ops`
//!
//! Functions to inspect or modify on Elektron Octatrack sets/projects and the files contained within
//!
//! If you just want to download the command line interface, go to the
//! [GitHub Releases page](https://github.com/dijksterhuis/ot-tools/releases/latest).

pub mod actions;
pub mod audio;
pub mod utils;

use itertools::Itertools;
use ot_tools_io::projects::Project;
use ot_tools_io::{Decode, Encode};
use regex::Regex;

pub type RBoxErr<T> = Result<T, Box<dyn std::error::Error>>;
pub type RVoidError<T> = Result<T, ()>;

#[derive(Debug)]
pub enum OctatoolErrors {
    PathDoesNotExist,
    PathIsNotADirectory,
    PathIsNotAFile,
    PathIsNotASet,
    CliInvalidPartIndex,
    CliMissingPartIndex,
    CliInvalidPatternIndex,
    CliMissingPatternIndex,
    InvalidFilenameOrExtension,
    InvalidOsVersion,
    // not in use yet
    CliInvalidTrackIndex,
    Unknown,
}
impl std::fmt::Display for OctatoolErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::PathDoesNotExist => write!(f, "path does not exist"),
            Self::PathIsNotADirectory => write!(f, "path is not a directory"),
            Self::PathIsNotAFile => write!(f, "path is not a file"),
            Self::PathIsNotASet => write!(
                f,
                "path is not an Octatrack set directory (no 'AUDIO' subdirectory found)"
            ),
            Self::CliMissingPartIndex => write!(
                f,
                "Missing part number(s) - part number(s) between 1-4 (inclusive) must be be provided"
            ),
            Self::CliInvalidPartIndex => write!(
                f,
                "Invalid part number(s) - only part numbers between 1-4 (inclusive) can be provided"
            ),
            Self::CliMissingPatternIndex => write!(
                f,
                "Missing pattern number(s) - pattern number(s) between 1-16 (inclusive) must be provided"
            ),
            Self::CliInvalidPatternIndex => write!(
                f,
                "Invalid pattern number(s) - only numbers between 1-16 (inclusive) can be provided"
            ),
            Self::InvalidFilenameOrExtension => write!(f, "path does not have a file extension"),
            Self::InvalidOsVersion => write!(f, "ot-tools only supports the following OS versions: {:?}", ALLOWED_OS_VERSIONS),
            // not in use yet
            Self::CliInvalidTrackIndex => write!(
                f,
                "Invalid track number(s) - only numbers between 1-8 can be provided"
            ),
            Self::Unknown => write!(f, "unknown error (please investigate/report)"),
        }
    }
}
impl std::error::Error for OctatoolErrors {}

// TODO: Need other strings for 1.40A and 1.40C
pub const ALLOWED_OS_VERSIONS: [&str; 3] = ["1.40A", "1.40B", "1.40C"];

/// Check the project OS version indicator is valid for the current version of ot-tools.
/// NOTE: this does not check the 'release' identifier string
/// (at least i *think* it's a release ident: `R0177`).
pub fn validate_project_version(project: &Project) -> bool {
    let re = Regex::new(r"\s+").unwrap();
    let split = re.split(&*project.metadata.os_version).collect_vec();
    ALLOWED_OS_VERSIONS.contains(&split[1])
}

#[cfg(test)]
mod test_proj_version {
    use crate::validate_project_version;
    use ot_tools_io::projects::Project;

    #[test]
    fn true_140a() {
        let mut proj = Project::default();
        proj.metadata.os_version = "R0999     1.40A".to_string();
        assert!(validate_project_version(&proj))
    }
    #[test]
    fn true_140b() {
        let mut proj = Project::default();
        proj.metadata.os_version = "R0999     1.40B".to_string();
        assert!(validate_project_version(&proj))
    }
    #[test]
    fn true_140c() {
        let mut proj = Project::default();
        proj.metadata.os_version = "R0999     1.40C".to_string();
        assert!(validate_project_version(&proj))
    }

    #[test]
    fn false_140d() {
        let mut proj = Project::default();
        proj.metadata.os_version = "R0999     1.40D".to_string();
        assert!(!validate_project_version(&proj))
    }

    #[test]
    fn false_139z() {
        let mut proj = Project::default();
        proj.metadata.os_version = "R0999     1.39Z".to_string();
        assert!(!validate_project_version(&proj))
    }

    #[test]
    fn false_100a() {
        let mut proj = Project::default();
        proj.metadata.os_version = "R0999     1.00A".to_string();
        assert!(!validate_project_version(&proj))
    }
}
