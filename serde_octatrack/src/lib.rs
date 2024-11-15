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
pub mod common;
pub mod constants;
pub mod projects;
pub mod samples;
pub mod utils;
