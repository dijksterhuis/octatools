//! Serialization and Deserialization of Bank files.

pub mod parts;
pub mod patterns;

use std::{error::Error, fs::File, io::Read, io::Write, path::Path};

use bincode;
use serde::{Deserialize, Serialize};
use serde_big_array::{Array, BigArray};

use crate::{
    banks::{parts::Part, patterns::Pattern},
    FromPath, FromYamlFile, RBoxErr, ToPath, ToYamlFile,
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

    /// Unsaved Part data for a Bank.
    ///
    /// Part state prior to before saving a Part is captured here.
    pub parts_unsaved: Box<Array<Part, 4>>,

    /// Saved Part data for a Bank.
    ///
    /// Part state once the Part has been saved is stored here.
    pub parts_saved: Box<Array<Part, 4>>,

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

impl FromPath for Bank {
    type T = Bank;

    /// Crete a new struct by reading a file located at `path`.
    fn from_path(path: &Path) -> Result<Self::T, Box<dyn Error>> {
        let mut infile = File::open(path)?;
        let mut bytes: Vec<u8> = vec![];
        let _: usize = infile.read_to_end(&mut bytes)?;

        let new: Self = bincode::deserialize(&bytes[..])?;

        Ok(new)
    }
}

impl ToPath for Bank {
    fn to_path(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let bytes: Vec<u8> = bincode::serialize(&self)?;
        let mut file: File = File::create(path)?;
        let _: RBoxErr<()> = file.write_all(&bytes).map_err(|e| e.into());

        Ok(())
    }
}

impl ToYamlFile for Bank {}
impl FromYamlFile for Bank {}

/// Used with the `octatools inspect bytes bank` command.
/// Only really useful for debugging and / or reverse engineering purposes.
#[derive(Debug, Serialize, Deserialize)]
pub struct BankRawBytes {
    pub data: Box<Array<u8, 636113>>,
}

impl FromPath for BankRawBytes {
    type T = BankRawBytes;

    /// Crete a new struct by reading a file located at `path`.
    fn from_path(path: &Path) -> Result<Self::T, Box<dyn Error>> {
        let mut infile = File::open(path)?;
        let mut bytes: Vec<u8> = vec![];
        let _: usize = infile.read_to_end(&mut bytes)?;

        let new: Self = bincode::deserialize(&bytes[..])?;

        Ok(new)
    }
}

impl ToPath for BankRawBytes {
    fn to_path(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let bytes: Vec<u8> = bincode::serialize(&self)?;
        let mut file: File = File::create(path)?;
        let _: RBoxErr<()> = file.write_all(&bytes).map_err(|e| e.into());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::banks::Bank;
    use crate::{FromPath, ToPath};
    use std::path::PathBuf;

    #[test]
    fn test_read_bank_file_no_errors() {
        let bank_file_path: PathBuf =
            PathBuf::from("data/tests/blank-project/bank01.work");
        let _: Bank = Bank::from_path(&bank_file_path.as_path()).unwrap();
        assert!(true);
    }

    #[test]
    fn test_read_and_write_bank_file_no_errors() {
        let src_file_path: PathBuf =
            PathBuf::from("data/tests/blank-project/bank01.work");
        let dst_file_path: PathBuf = PathBuf::from("/tmp/bank01.work");
        let bank: Bank = Bank::from_path(&src_file_path).unwrap();
        let _ = bank.to_path(&dst_file_path.as_path());
        assert!(true);
    }
}
