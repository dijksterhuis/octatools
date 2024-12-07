//! A Compact Flash Card (CF Card) index can be used to:
//! (1) check for change conflicts (samples already used in existing Octatrack Projects being overwritten or edited) when attempting to copy samples onto a CF card.
//! (2) inspect the current state of sample use across an CF Card.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use serde_octatrack::{FromPath, FromYamlFile, ToYamlFile};

use crate::octatrack_sets::OctatrackSetFiles;
/// A single row of data written to the index file.
use crate::RBoxErr;

/// A compact flash card which we need to scan for audio files.

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct CompactFlashDrive {
    /// The path to the current compact flash card.
    cfcard_path: PathBuf,

    /// Octatrack Sets on the compact flash card.
    ot_sets: Vec<OctatrackSetFiles>,
}

impl FromYamlFile for CompactFlashDrive {}
impl ToYamlFile for CompactFlashDrive {}

impl FromPath for CompactFlashDrive {
    type T = CompactFlashDrive;

    /// Crete a new struct by reading a file located at `path`.
    fn from_path(path: &Path) -> RBoxErr<Self::T> {
        let ot_sets = OctatrackSetFiles::from_cfcard_pathbuf(path)?;

        let cf = CompactFlashDrive {
            cfcard_path: path.to_path_buf(),
            ot_sets,
        };

        Ok(cf)
    }
}

#[cfg(test)]
mod test {
    use serde_octatrack::FromYamlFile;

    use super::*;

    #[test]
    fn from_yaml_ok() {
        let testyaml = PathBuf::from("data/tests/drive/test.yml");

        let r = CompactFlashDrive::from_yaml(&testyaml);
        assert!(r.is_ok());
    }
}
