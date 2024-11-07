//! Serialization and Deserialization of Bank files.

pub mod parts;
pub mod patterns;

use log::{debug, error, info, trace, warn};
use std::{error::Error, fs::File, io::Read, io::Write, path::PathBuf};

use bincode;
use serde::{Deserialize, Serialize};
use serde_big_array::{Array, BigArray};

use crate::{
    banks::{parts::Part, patterns::Pattern},
    common::RBoxErr,
    common::{FromFileAtPathBuf, ToFileAtPathBuf},
};

/// An Octatrack Bank. Contains data related to Parts and Patterns.
#[derive(Serialize, Deserialize)]
pub struct Bank {
    /// Misc header data for Banks.
    /// Always follows the same format.
    ///
    /// Example data:
    /// ```text
    /// FORM....DPS1BANK......
    /// 46 4f 52 4d 00 00 00 00 44 50 53 31 42 41 4e 4b
    /// 00 00 00 00 00 17
    /// ```
    #[serde(with = "BigArray")]
    pub header_data: [u8; 22],

    /// Pattern data for a Bank.
    pub patterns: Box<Array<Pattern, 16>>,

    /// Part data for a Bank.
    /// Note: There are 8 `PART` sections.
    /// One batch of 4x `PART` sections will be related to previous saved Part state for reloading.
    pub parts: Box<Array<Part, 8>>,

    /// Unknown what these bytes refer to.
    #[serde(with = "BigArray")]
    pub unknown: [u8; 5],

    /// Names for each Part within the Bank.
    /// Maximum 7 character length.
    #[serde(with = "BigArray")]
    pub part_names: [[u8; 7]; 4],

    /// Seems to be related to whether the Bank has been modified or saved?
    #[serde(with = "BigArray")]
    pub remainder: [u8; 2],
}

impl FromFileAtPathBuf for Bank {
    type T = Bank;

    /// Crete a new struct by reading a file located at `path`.
    fn from_pathbuf(path: PathBuf) -> Result<Self::T, Box<dyn Error>> {
        trace!("Reading Bank file data: {path:#?}");
        let mut infile = File::open(&path)?;
        let mut bytes: Vec<u8> = vec![];
        let _: usize = infile.read_to_end(&mut bytes)?;
        debug!("Read Bank file data: {path:#?}");

        trace!("Deserializing Bank file: {path:#?}");
        let new: Self = bincode::deserialize(&bytes[..])?;
        debug!("Deserialized Bank file: {path:#?}");

        Ok(new)
    }
}

impl ToFileAtPathBuf for Bank {
    fn to_pathbuf(&self, path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        trace!("Serializing Bank data ...");
        let bytes = bincode::serialize(&self)?;
        debug!("Serialized Bank data.");

        trace!("Writing Bank data ...");
        let mut file = File::create(path)?;
        let _: RBoxErr<()> = file.write_all(&bytes).map_err(|e| e.into());
        debug!("Written Bank data.");

        Ok(())
    }
}
