//! A Compact Flash Card (CF Card) index can be used to:
//! (1) check for change conflicts (samples already used in existing Octatrack Projects being overwritten or edited) when attempting to copy samples onto a CF card.
//! (2) inspect the current state of sample use across an CF Card.

use crate::octatrack_sets::OctatrackSetFiles;
use crate::RBoxErr;
use octatools_derive::{Decodeable, Encodeable};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// A compact flash card which we need to scan for audio files.

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Encodeable, Decodeable)]
pub struct CompactFlashDrive {
    /// The path to the current compact flash card.
    cfcard_path: PathBuf,

    /// Octatrack Sets on the compact flash card.
    ot_sets: Vec<OctatrackSetFiles>,
}

impl CompactFlashDrive {
    /// Crete a new struct by reading a file located at `path`.
    pub fn from_path(path: &Path) -> RBoxErr<Self> {
        let ot_sets = OctatrackSetFiles::from_cfcard_pathbuf(path)?;

        let cf = CompactFlashDrive {
            cfcard_path: path.to_path_buf(),
            ot_sets,
        };

        Ok(cf)
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod test {
    use super::*;
    use serde_octatrack::yaml_file_to_type;

    #[test]
    fn from_yaml_ok() {
        let testyaml = PathBuf::from("../data/tests/drive/test.yml");

        let r = yaml_file_to_type::<CompactFlashDrive>(&testyaml);
        assert!(r.is_ok());
    }
}
