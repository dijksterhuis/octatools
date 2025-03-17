//! Serialization and Deserialization of Bank files.

pub mod parts;
pub mod patterns;

use crate::{
    banks::{parts::Part, patterns::Pattern},
    DefaultsArrayBoxed,
};
use std::array::from_fn;

use octatools_derive::{Decodeable, DefaultsAsBoxedBigArray, Encodeable};
use serde::{Deserialize, Serialize};
use serde_big_array::{Array, BigArray};

/// Bank header data.
/// ```text
/// FORM....DPS1BANK......
/// 46 4f 52 4d 00 00 00 00 44 50 53 31 42 41 4e 4b 00 00 00 00 00 17
/// [70 79 82 77 0 0 0 0 68 80 83 49 66 65 78 75 0 0 0 0 0 23]
/// ```
const BANK_HEADER: [u8; 22] = [
    70, 79, 82, 77, 0, 0, 0, 0, 68, 80, 83, 49, 66, 65, 78, 75, 0, 0, 0, 0, 0, 23,
];

/// Default Part names (PART 1, PART 2 etc.) converted to u8 for ease of use.
const DEFAULT_PART_NAMES: [[u8; 7]; 4] = [
    [80, 65, 82, 84, 32, 49, 0], // "PART 1"
    [80, 65, 82, 84, 32, 50, 0], // "PART 2"
    [80, 65, 82, 84, 32, 51, 0], // "PART 3"
    [80, 65, 82, 84, 32, 52, 0], // "PART 4"
];

/// An Octatrack Bank. Contains data related to Parts and Patterns.
#[derive(
    Debug, Serialize, Deserialize, Clone, PartialEq, DefaultsAsBoxedBigArray, Decodeable, Encodeable,
)]
pub struct Bank {
    /// Misc header data for Banks.
    /// Always follows the same format.
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

impl Default for Bank {
    fn default() -> Self {
        Self {
            header_data: BANK_HEADER,
            patterns: Pattern::defaults(),
            parts_unsaved: Part::defaults(),
            parts_saved: Part::defaults(),
            unknown: from_fn(|_| 0),
            part_names: DEFAULT_PART_NAMES,
            remainder: from_fn(|_| 0),
        }
    }
}

/// Used with the `octatools inspect bytes bank` command.
/// Only really useful for debugging and / or reverse engineering purposes.
#[derive(Debug, Serialize, Deserialize, Decodeable)]
pub struct BankRawBytes {
    pub data: Box<Array<u8, 636113>>,
}
