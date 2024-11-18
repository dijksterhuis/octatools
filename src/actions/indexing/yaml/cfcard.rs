//! A Compact Flash Card (CF Card) index can be used to:
//! (1) check for change conflicts (samples already used in existing Octatrack Projects being overwritten or edited) when attempting to copy samples onto a CF card.
//! (2) inspect the current state of sample use across an CF Card.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use serde_octatrack::{FromPathBuf, ToPathBuf};

/// A single row of data written to the index file.
use crate::common::{FromYamlFile, RBoxErr, ToYamlFile};
use crate::octatrack_sets::OctatrackSet;

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

impl FromPathBuf for CompactFlashDrive {
    type T = CompactFlashDrive;

    /// Crete a new struct by reading a file located at `path`.
    fn from_pathbuf(path: &PathBuf) -> RBoxErr<Self::T> {
        let ot_sets = OctatrackSet::from_cfcard_pathbuf(path).unwrap();

        let cf = CompactFlashDrive {
            // todo: clone :/
            cfcard_path: path.clone(),
            ot_sets,
        };

        Ok(cf)
    }
}


/*
# TODO

- directory structures and dummy contents for multiple integration tests
  (include negative testing!)
- grab list of acceptable audio file extensions from OctaChainer
- use ^ OTChainer audio file extension list instead of exclusionary search
- read audio file data and OT file data into the CSV file
- test on the OT whether adding a sample to static/flex slots creates a new OT file
  (there are a lot of wav files with no OT file ... makes life simpler if not needed!)
- unit tests where possible!
- think about a better data store format than CSV -- sqlite?
  (csv works well for cli, less good for GUI ... focus on CLI w/ CSV for now)
- general clean up, better results and error handling
- turn this whole thing into a lib? with separate projects for cli and gui?
- OT_IO: read multiple wavs and create a **sample chain OT file** has been done...
  Need to handle writing of the wavs, with similar testing -- how to isolate tests?
- Move integration tests to main.rs? or lib.rs? they test parts of the whole thing,
  rather than individual units within a module.
- Read up on results, error handling etc in rust. need to see about how to safely
  handle edge cases i've not caught.
- CLI design: commands + actions + flags a la `ot-sm sync pull`
  or `ot-sm chain ./file_a.wav outfile --tempo 124.5 --gain 1.0 --quantize DIRECT`.
*/

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Need to test the output
    #[test]
    fn test_indexing_cfcard_sets() {
        let cfcard_path = PathBuf::from("data/tests/index-cf/");
        let res: RBoxErr<CompactFlashDrive> = CompactFlashDrive::from_pathbuf(&cfcard_path);
        assert!(res.is_ok());
    }
}
