//! Reading `bank??.*` files

mod parts;
mod patterns;

use log::{debug, error, info, trace, warn};
use std::{error::Error, fs::File, io::Read, io::Write, path::PathBuf};

use bincode;
use serde::{Deserialize, Serialize};
use serde_big_array::{Array, BigArray};

use crate::common::RBoxErr;
use crate::octatrack::{
    banks::{parts::Part, patterns::Pattern},
    common::{FromFileAtPathBuf, ToFileAtPathBuf},
};

/// A Bank.

#[derive(Serialize, Deserialize)]
pub struct Bank {
    /// Misc header data
    ///
    /// example data:
    /// ```
    /// FORM....DPS1BANK......
    /// 46 4f 52 4d 00 00 00 00 44 50 53 31 42 41 4e 4b
    /// 00 00 00 00 00 17
    /// ```
    #[serde(with = "BigArray")]
    pub header_data: [u8; 22],

    /// All Patterns within a Bank
    // #[serde(with = "BigArray")]
    pub patterns: Box<Array<Pattern, 16>>,

    // /// Parts level data
    // /// There are 8 PART sections.
    // /// One batch of 4x sections will be related to previous saved Part state for reloading.
    // #[serde(with = "BigArray")]
    pub parts: Box<Array<Part, 8>>,

    #[serde(with = "BigArray")]
    pub data_block_1: [u8; 5],

    /// The different Part names.
    /// Only ever four of them and always a 7 character length String.
    // #[serde(deserialize_with = "deserialize_string")]
    // pub part_names: [PartName; 4],

    #[serde(with = "BigArray")]
    pub part_names: [[u8; 7]; 4],

    // /// Whether parts have been saved or not?!
    // /// Need to check what the last value in the bank file is.
    // /// It looks like a mask for which parts are edited or not and not yet saved.
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
