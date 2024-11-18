//! Serialization and Deserialization of Bank files.

pub mod parts;
pub mod patterns;

use std::{error::Error, fs::File, io::Read, io::Write, path::PathBuf};

use bincode;
use serde::{Deserialize, Serialize};
use serde_big_array::{Array, BigArray};

use crate::{
    banks::{parts::Part, patterns::Pattern},
    FromPathBuf, RBoxErr, ToPathBuf,
};

/// An Octatrack Bank. Contains data related to Parts and Patterns.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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
    // note -- stack overflow if tring to use #[serde(with = "BigArray")]
    pub patterns: Box<Array<Pattern, 16>>,

    /// Part data for a Bank.
    /// Note: There are 8 `PART` sections.
    /// One batch of 4x `PART` sections will be related to previous saved Part state for reloading.
    // note -- stack overflow if tring to use #[serde(with = "BigArray")]
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

impl FromPathBuf for Bank {
    type T = Bank;

    /// Crete a new struct by reading a file located at `path`.
    fn from_pathbuf(path: &PathBuf) -> Result<Self::T, Box<dyn Error>> {
        let mut infile = File::open(path)?;
        let mut bytes: Vec<u8> = vec![];
        let _: usize = infile.read_to_end(&mut bytes)?;

        let new: Self = bincode::deserialize(&bytes[..])?;

        Ok(new)
    }
}

impl ToPathBuf for Bank {
    fn to_pathbuf(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let bytes: Vec<u8> = bincode::serialize(&self)?;
        let mut file: File = File::create(path)?;
        let _: RBoxErr<()> = file.write_all(&bytes).map_err(|e| e.into());

        Ok(())
    }
}

/// Used with the `octatools inspect bytes bank` command.
/// Only really useful for debugging and / or reverse engineering purposes.
#[derive(Debug, Serialize, Deserialize)]
pub struct BankRawBytes {
    pub data: Box<Array<u8, 636113>>,
}

impl FromPathBuf for BankRawBytes {
    type T = BankRawBytes;

    /// Crete a new struct by reading a file located at `path`.
    fn from_pathbuf(path: &PathBuf) -> Result<Self::T, Box<dyn Error>> {
        let mut infile = File::open(path)?;
        let mut bytes: Vec<u8> = vec![];
        let _: usize = infile.read_to_end(&mut bytes)?;

        let new: Self = bincode::deserialize(&bytes[..])?;

        Ok(new)
    }
}

impl ToPathBuf for BankRawBytes {
    fn to_pathbuf(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let bytes: Vec<u8> = bincode::serialize(&self)?;
        let mut file: File = File::create(path)?;
        let _: RBoxErr<()> = file.write_all(&bytes).map_err(|e| e.into());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::banks::Bank;
    use crate::{FromPathBuf, ToPathBuf};
    use std::path::PathBuf;

    #[test]
    fn test_read_bank_file_no_errors() {
        let bank_file_path: PathBuf =
            PathBuf::from("data/tests/index-cf/DEV-OTsm/BLANK/bank01.work");
        let _: Bank = Bank::from_pathbuf(&bank_file_path).unwrap();
        assert!(true);
    }

    #[test]
    fn test_read_and_write_bank_file_no_errors() {
        let src_file_path: PathBuf =
            PathBuf::from("data/tests/index-cf/DEV-OTsm/BLANK/bank01.work");
        let dst_file_path: PathBuf = PathBuf::from("/tmp/bank01.work");
        let bank: Bank = Bank::from_pathbuf(&src_file_path).unwrap();
        let _ = bank.to_pathbuf(&dst_file_path);
        assert!(true);
    }
}
