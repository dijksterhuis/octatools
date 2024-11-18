//! # `serde_octatrack`
//!
//! Serialization and Deserialization library for Elektron Octatrack data files, including
//!
//! - arrangement files -- `arr??.*`
//! - bank files -- `bank??.*`
//! - project files -- `project.*`
//! - sample attribute files -- `*.ot`
//!
//! The code in this library is quite rough still.
//! Do not expect anything robust just yet.

pub mod arrangements;
pub mod banks;
pub mod constants;
pub mod projects;
pub mod samples;
pub mod utils;

use std::{error::Error, fmt::Debug, path::PathBuf};

// todo: sized errors so not necessary to keep Boxing error enum varients
/// Shorthand type alias for a Result with a Boxed Error
type RBoxErr<T> = Result<T, Box<dyn Error>>;

/// Global error variants
#[derive(Debug, PartialEq, Eq)]
pub enum SerdeOctatrackErrors {
    /// An 'Options' Enum (e.g. `SampleAttributesLoopMode`) does not have a matching variant for this value
    NoMatchingOptionEnumValue,
    /// Could not parse a sample slot string data when loading a project
    ProjectSampleSlotParsingError,
    /// I know an error exists here, but I'm busy yak shaving something else at the moment.
    TodoError,
}
impl std::fmt::Display for SerdeOctatrackErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::NoMatchingOptionEnumValue => {
                write!(f, "no matching enum option for the provided value")
            }
            Self::ProjectSampleSlotParsingError => {
                write!(f, "count not load sample slot from project string data")
            }
            Self::TodoError => {
                write!(
                    f,
                    "this error is handled, but an error variant is not created yet"
                )
            }
        }
    }
}
impl std::error::Error for SerdeOctatrackErrors {}

/// Trait to convert between Enum option instances and their corresponding value.
trait OptionEnumValueConvert {
    /// One of the enum types within the `octatrack::options` module.
    type T;

    /// Input type for `from_value` and return type for `value` method.
    type V;

    /// Get an Enum instance from a numeric value.
    fn from_value(v: &Self::V) -> RBoxErr<Self::T>;

    /// Get a numeric value for an Enum instance.
    fn value(&self) -> RBoxErr<Self::V>;
}

/// Trait to use when a new struct can be deserialised from some file or directory tree located at the specified path.
pub trait FromPathBuf {
    /// Type for `Self`
    type T;

    /// Crete a new struct by reading a file located at `path`.
    fn from_pathbuf(path: &PathBuf) -> Result<Self::T, Box<dyn std::error::Error>>;
}

/// Trait to use when a new file(s) can be written at the specifed path by serializing a struct
pub trait ToPathBuf {
    /// Crete a new file at the path file location struct by serializing struct data.
    fn to_pathbuf(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>>;
}
