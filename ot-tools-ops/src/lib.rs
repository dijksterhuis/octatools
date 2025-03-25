//! # `ot-tools-ops`
//!
//! Functions to inspect or modify on Elektron Octatrack sets/projects and the files contained within
//!
//! If you just want to download the command line interface, go to the
//! [GitHub Releases page](https://github.com/dijksterhuis/ot-tools/releases/latest).

pub mod actions;
pub mod audio;
pub mod utils;

use ot_tools_io::{Decode, Encode};

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
