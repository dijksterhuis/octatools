//! A Compact Flash Card (CF Card) index can be used to:
//! (1) check for change conflicts (samples already used in existing Octatrack Projects being overwritten or edited) when attempting to copy samples onto a CF card.
//! (2) inspect the current state of sample use across an CF Card.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use serde_octatrack::{FromPath, FromYamlFile, ToYamlFile};

use crate::octatrack_sets::OctatrackSet;
/// A single row of data written to the index file.
use crate::RBoxErr;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct CompactFlashScanCsvRow {
    /// Disk name of the card
    cfcard: String,

    /// Octatrack Set this row is a member of
    set: String,

    /// File name of an indexed audio file.
    audio_name: String,

    /// File path of an indexed audio file.
    audio_filepath: PathBuf,

    /// File path of an indexed `.ot` Octatrack sample metadata settings file.
    ot_filepath: Option<PathBuf>,

    /// Whether this these files are part of the Set's Audio Pool,
    /// or contained in a specific Octatrack Project.
    is_set_audio_pool: bool,

    /// The Octatrack Project these files are related to.
    project: Option<String>,
}

/// A compact flash card which we need to scan for audio files.

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct CompactFlashDrive {
    /// The path to the current compact flash card.
    cfcard_path: PathBuf,

    /// Octatrack Sets on the compact flash card.
    ot_sets: Vec<OctatrackSet>,
}

impl FromYamlFile for CompactFlashDrive {}
impl ToYamlFile for CompactFlashDrive {}

impl FromPath for CompactFlashDrive {
    type T = CompactFlashDrive;

    /// Crete a new struct by reading a file located at `path`.
    fn from_path(path: &Path) -> RBoxErr<Self::T> {
        let ot_sets = OctatrackSet::from_cfcard_pathbuf(path).unwrap();

        let cf = CompactFlashDrive {
            // todo: clone :/
            cfcard_path: path.to_path_buf(),
            ot_sets,
        };

        Ok(cf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Need to test the output
    #[test]
    fn test_indexing_cfcard_sets() {
        let cfcard_path = PathBuf::from("data/tests/drive/");
        let res: RBoxErr<CompactFlashDrive> = CompactFlashDrive::from_path(&cfcard_path);
        assert!(res.is_ok());
    }
}
