//! Serialization and Deserialization of Bank files.

pub mod parts;
pub mod patterns;

use crate::{
    banks::{parts::Parts, patterns::Pattern},
    CheckHeader, DefaultsArrayBoxed, IsDefault,
};
use std::array::from_fn;

use ot_tools_derive::{Decodeable, DefaultsAsBoxedBigArray, Encodeable};
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

/// Default Part names (ONE, TWO, THREE, FOUR) converted to u8 for ease of use.
const DEFAULT_PART_NAMES: [[u8; 7]; 4] = [
    [0x4f, 0x4e, 0x45, 0x00, 0x00, 0x00, 0x00], // "ONE"
    [0x54, 0x57, 0x4f, 0x00, 0x00, 0x00, 0x00], // "TWO"
    [0x54, 0x48, 0x52, 0x45, 0x45, 0x00, 0x00], // "THREE"
    [0x46, 0x4f, 0x55, 0x52, 0x00, 0x00, 0x00], // "FOUR"
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

    /// All part data for this bank, includes currently unsaved and previously saved state
    pub parts: Parts,

    /// Unknown what these bytes refer to.
    #[serde(with = "BigArray")]
    pub unknown: [u8; 5],

    /// Names for each Part within the Bank.
    /// Maximum 7 character length.
    #[serde(with = "BigArray")]
    pub part_names: [[u8; 7]; 4],

    /// Seems to be related to whether the Bank has been modified or saved?
    #[serde(with = "BigArray")]
    pub checksum: [u8; 2],
}

impl Default for Bank {
    fn default() -> Self {
        Self {
            header_data: BANK_HEADER,
            patterns: Pattern::defaults(),
            parts: Parts::default(),
            unknown: from_fn(|_| 0),
            part_names: DEFAULT_PART_NAMES,
            checksum: from_fn(|_| 0),
        }
    }
}

impl IsDefault for Bank {
    fn is_default(&self) -> bool {
        let default = &mut Bank::default();
        // until i work out the checksums, set the default checksum to equal the
        // current instance's values
        default.checksum = self.checksum;
        default == self
    }
}

impl CheckHeader for Bank {
    fn check_header(&self) -> bool {
        self.header_data == BANK_HEADER
    }
}

/// Used with the `ot-tools-cli inspect bytes bank` command.
/// Only really useful for debugging and / or reverse engineering purposes.
#[derive(Debug, Serialize, Deserialize, Decodeable)]
pub struct BankRawBytes {
    pub data: Box<Array<u8, 636113>>,
}
