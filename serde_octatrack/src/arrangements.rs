//! Deserialization of `arr??.*` files to extract out and work with Arranger data.
//! Note that Serialization is not complete for arrangement files as there are some
//! intricacies relate to how Arranger Row data is written in files (will require
//! custom Ser/De trait implementations).
//!
//! ### How data is persisted in `*.work` and `*.strd` files
//!
//! `arr??.work` files are updated by using the PROJECT -> SYNC TO CARD operation.
//! The `arr??.strd` files are not changed by the PROJECT -> SYNC TO CARD operation.
//!
//! `arr??.strd` files are updated with the latest arrangement data
//! when performing the ARRANGER -> SAVE operation.
//! The `arr??.work` files are not changed by the ARRANGER -> SAVE operation.
//!
//! ### TODO
//!
//! - `ArrangeBlock.unknown_1` block
//! - `ArrangementFile` unknown blocks:
//!   - Unknown block 1
//!   - Unknown block 2

mod deserialize;
mod serialize;

use crate::DefaultsArrayBoxed;
use octatools_derive::{DefaultsAsBoxedBigArray, Decodeable, Encodeable};
use serde::{Deserialize, Serialize};
use serde_big_array::{Array, BigArray};

const ARRANGEMENT_FILE_HEADER: [u8; 22] = [
    70, 79, 82, 77, 0, 0, 0, 0, 68, 80, 83, 49, 65, 82, 82, 65, 0, 0, 0, 0, 0, 6,
];

// "OCTATOOLS-ARR"
const ARRANGEMENT_DEFAULT_NAME: [u8; 15] =
    [79, 67, 84, 65, 84, 79, 79, 76, 83, 45, 65, 82, 82, 32, 32];

// max length: 11336 bytes
/// Public representation of an `arr??.*` Arrangement file.
#[derive(Debug, Serialize, PartialEq, Encodeable, Decodeable)]
pub struct ArrangementFile {
    /// Header data:
    /// ```text
    /// ASCII: FORM....DPS1ARRA........
    /// Hex: 46 4f 52 4d 00 00 00 00 44 50 53 31 41 52 52 41 00 00 00 00 00 06
    /// U8: [70, 79, 82, 77, 0, 0, 0, 0, 68, 80, 83, 49, 65, 82, 82, 65, 0, 0, 0, 0, 0, 6]
    /// ```
    pub header: [u8; 22],

    /// Dunno. Example data:
    /// ```text
    /// [0, 0]
    /// ```
    pub unk1: [u8; 2],

    /// Current arrangement data in active use.
    /// This block is written when saving via Project Menu -> SYNC TO CARD.
    ///
    /// The second block is written when saving the arrangement via Arranger Menu -> SAVE.
    // #[serde(with = "BigArray")]
    pub arrangement_state_current: ArrangementBlock,

    /// Dunno. Example data:
    /// ```text
    /// [0, 0]
    /// ```
    pub unk2: [u8; 2],

    /// Arrangement data from the previous saved state.
    /// This block is written when saving the arrangement via Arranger Menu -> SAVE.
    pub arrangement_state_previous: ArrangementBlock,
    /// Example data:
    /// ```text
    /// Arrangement 1 has content: [1, 0, 0, 0, 0, 0, 0, 0]
    /// Arrangement 2 has content: [0, 1, 0, 0, 0, 0, 0, 0]
    /// Arrangement 7 & 8 have content: [0, 1, 0, 0, 0, 0, 1, 1]
    /// ```
    pub arrangements_active_flag: [u8; 8],
    /// Checksum for the file. Maybe a bit mask value? Example data:
    /// ```text
    /// 30 rows: [188, 168]
    /// 31 rows: [196, 196]
    /// 0 rows (just names): [7, 70]
    /// ```
    pub check_sum: [u8; 2],
}

impl Default for ArrangementFile {
    fn default() -> Self {
        Self {
            header: ARRANGEMENT_FILE_HEADER,
            unk1: [0, 0],
            arrangement_state_current: ArrangementBlock::default(),
            unk2: [0, 0],
            arrangement_state_previous: ArrangementBlock::default(),
            arrangements_active_flag: [0, 0, 0, 0, 0, 0, 0, 0],
            check_sum: [0, 0],
        }
    }
}

/// An Arrangement 'block'. 5650 bytes.
/// There are multiple arrangement states in `arr??.*` files for Arrangements,
/// seemingly due to the peculiarities of how the Octatrack stores data
/// (Project Menu -> SYNC TO CARD and Arranger Mnu -> SAVE both save to different
/// parts of the file / save to different files),
#[derive(Debug, Eq, PartialEq)]
pub struct ArrangementBlock {
    /// Name of the Arrangement in ASCII values, max length 15 characters
    pub name: [u8; 15], // String,

    /// Unknown data. No idea what this is. Usually [0, 0].
    pub unknown_1: [u8; 2],

    /// Number of active rows in the arrangement. Any parsed row data after this number of rows
    /// should be an `ArrangeRow::EmptyRow` variant.
    ///
    /// WARNING: Max number of `ArrangeRows` returns a zero value here!
    pub n_rows: u8,

    /// Rows of the arrangement. Maximum 256 rows possible.
    pub rows: Box<Array<ArrangeRow, 256>>,
}

impl Default for ArrangementBlock {
    fn default() -> Self {
        Self {
            name: ARRANGEMENT_DEFAULT_NAME,
            unknown_1: [0, 0],
            n_rows: 0,
            rows: ArrangeRow::defaults(),
        }
    }
}

/// A Row in the Arrangement.
#[derive(Debug, PartialEq, Eq, DefaultsAsBoxedBigArray)]
pub enum ArrangeRow {
    /// pattern choice and playback
    PatternRow {
        // row_type: u8,
        /// Which Pattern should be played at this point. Patterns are indexed from 0 (A01) -> 256 (P16).
        pattern_id: u8,
        /// How many times to play this arrangement row.
        repetitions: u8,
        // unused_1: u8,
        /// How track muting is applied during this arrangement row.
        mute_mask: u8,
        // unused_2: u8,
        /// First part of the Tempo mask for this row.
        /// Needs to be combined with `tempo_2` to work out the actual tempo (not sure how it works yet).
        tempo_1: u8,
        /// Second part of the Tempo mask for this row.
        /// Needs to be combined with `tempo_1` to work out the actual tempo (not sure how it works yet).
        tempo_2: u8,
        /// Which scene is assigned to Scene slot A when this arrangement row is playing.
        scene_a: u8,
        /// Which scene is assigned to Scene slot B when this arrangement row is playing.
        scene_b: u8,
        // unused_3: u8,
        /// Which trig to start Playing the pattern on.
        offset: u8,
        // unused_4: u8,
        /// How many trigs to play the pattern for.
        /// Note that this value always has `offset` added to it.
        /// So a length on the machine display of 64 when the offset is 32 will result in a value of 96 in the file data.
        length: u8,
        /// MIDI Track transposes for all 8 midi channels.
        /// 1 -> 48 values are positive transpose settings.
        /// 255 (-1) -> 207 (-48) values are negative transpose settings.
        midi_transpose: [u8; 8],
    },
    /// Loop/Jump/Halt rows are all essentially just loops. Example: Jumps are an infinite loop.
    /// So these are bundled into one type.
    ///
    /// Loops are `loop_count = 0 -> 65` and the `row_target` is any row before this one (`loop_count=0` is infinite looping).
    /// Halts are `loop_count = 0` and the `row_target` is this row.
    /// Jumps are `loop_count = 0` and the `row_target` is any row after this one.
    LoopOrJumpOrHaltRow {
        /// How many times to loop to the `row_target`. Only applies to loops.
        loop_count: u8,
        /// The row number to loop back to, jump to, or end at.
        row_target: u8,
    },
    /// A row of ASCII text data with 15 maximum length.
    ReminderRow(String),
    /// Row is not in use. Only used in an `ArrangementBlock` as a placeholder for null basically.
    EmptyRow(),
}

impl Default for ArrangeRow {
    fn default() -> Self {
        Self::EmptyRow()
    }
}

// max length: 11336 bytes
/// Used with the `octatools inspect bytes arrangement` command.
/// Only really useful for debugging and / or reverse engineering purposes.
#[derive(Debug, Serialize, Deserialize, Decodeable)]
pub struct ArrangementFileRawBytes {
    #[serde(with = "BigArray")]
    pub data: [u8; 11336],
}
