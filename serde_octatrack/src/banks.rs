//! Serialization and Deserialization of Bank files.

pub mod parts;
pub mod patterns;

use crate::{
    banks::{parts::Part, patterns::Pattern},
    Decode, Encode,
};
use serde::{Deserialize, Serialize};
use serde_big_array::{Array, BigArray};

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

impl Encode for Bank {}
impl Decode for Bank {}

/// Used with the `octatools inspect bytes bank` command.
/// Only really useful for debugging and / or reverse engineering purposes.
#[derive(Debug, Serialize, Deserialize)]
pub struct BankRawBytes {
    pub data: Box<Array<u8, 636113>>,
}

impl Encode for BankRawBytes {}
impl Decode for BankRawBytes {}
