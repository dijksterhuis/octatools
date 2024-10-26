//! A Compact Flash Card (CF Card) index can be used to:
//! (1) check for change conflicts (samples already used in existing Octatrack Projects being overwritten or edited) when attempting to copy samples onto a CF card.
//! (2) inspect the current state of sample use across an CF Card.

use std::{
    path::PathBuf,
    fs::File,
    io::Write,
};
use csv::Writer;
use serde::{Serialize, Deserialize};

use crate::octatrack::sets::OctatrackSet;
use crate::common::{FromYamlFile, ToYamlFile};
/// A single row of data written to the index file.

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

    pub index_file_path: Option<PathBuf>,

    /// The path to the current compact flash card.
    cfcard_path: PathBuf,

    /// Octatrack Sets on the compact flash card. 
    ot_sets: Vec<OctatrackSet>,
}

impl FromYamlFile for CompactFlashDrive{}
impl ToYamlFile for CompactFlashDrive{}

impl CompactFlashDrive {

    /// Index a compact flash card by scanning through each `OctatrackSet` under the given `PathBuf`'s directory tree.
    
    pub fn from_pathbuf(cfcard_path: PathBuf, index_file_path: Option<PathBuf>) -> Result<CompactFlashDrive, ()> {

        // TODO: Hard exit on failure
        let ot_sets = OctatrackSet::from_cfcard_pathbuf(&cfcard_path).unwrap();

        let cf = CompactFlashDrive {
            index_file_path,
            cfcard_path,
            ot_sets,
        };

        Ok(cf)
    }

    // /// Create a local index file for a `CompactFlashDrive` which has been indexed.
    // /// Audio Pool records are written first, then individual Projects
    
    // fn to_csv(&self, csv_filepath: &PathBuf) -> Result<(), std::io::Error> {

    //     let sets = self.ot_sets.clone();
    //     let mut wtr = Writer::from_writer(vec![]);

    //     for s in sets {
    //         for audio_pool_sample in s.audio_pool.samples {

    //             let row = CompactFlashScanCsvRow {
    //                 cfcard: self.cfcard_path.file_name().unwrap().to_str().unwrap().to_string(),
    //                 set: s.name.clone(),
    //                 is_set_audio_pool: true,
    //                 project: None,
    //                 audio_filepath: audio_pool_sample.audio_path,
    //                 ot_filepath: audio_pool_sample.otfile_path,
    //                 audio_name: audio_pool_sample.name,
    //             };

    //             let _ = wtr.serialize(row);
    //         }

    //         for project in &s.projects {
    //             for project_sample in &project.samples {

    //                 let row = CompactFlashScanCsvRow {
    //                     cfcard: self.cfcard_path.file_name().unwrap().to_str().unwrap().to_string(),
    //                     set: s.name.clone(),
    //                     is_set_audio_pool: true,
    //                     project: None,
    //                     audio_filepath: project_sample.audio_path.clone(),
    //                     ot_filepath: project_sample.otfile_path.clone(),
    //                     audio_name: project_sample.name.clone(),
    //                 };
    
    //                 let _ = wtr.serialize(row);
    //             }
    //         }
    //     }

    //     let mut file = File::create(csv_filepath).unwrap();
    //     let write_result: Result<(), std::io::Error> = file
    //         .write_all(&wtr.into_inner().unwrap())
    //         .map_err(|e| e)
    //     ;

    //     write_result
    // }

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
    fn dummy_test() {
        let cfcard_path = PathBuf::from("data/tests/index-cf/DEV-OTsm/");
        let _csv_path = PathBuf::from("./.otsm-index-cf.csv");

        let res: Result<CompactFlashDrive, ()> = CompactFlashDrive
            ::from_pathbuf(cfcard_path, None)
        ;

        assert!(res.is_ok());
    }

}