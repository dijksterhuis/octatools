use std::{
    path::PathBuf,
    fs::File,
    io::Write,
};
use csv::Writer;
use log::{error, info, warn, debug};

use crate::otset::OctatrackSet;


#[derive(PartialEq, Debug, Clone, serde::Serialize)]
pub struct CompactFlashScanCsvRow {
    cfcard: String,
    set: String,
    audio_name: String,
    audio_filepath: PathBuf,
    ot_filepath: Option<PathBuf>,
    is_set_audio_pool: bool,
    project: Option<String>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct CompactFlashDrive {
    cfcard_path: PathBuf,
    ot_sets: Vec<OctatrackSet>,
}

impl CompactFlashDrive {
    pub fn new(path: &PathBuf, csv_file_path: &Option<&PathBuf>) -> Result<CompactFlashDrive, ()> {
        let ot_sets = OctatrackSet::from_cfcard_pathbuf(&path);

        let cf = CompactFlashDrive {
            cfcard_path: path.clone(),
            ot_sets: ot_sets.unwrap(),
        };

        if ! csv_file_path.is_none() {
            info!("Generating CF Card index file locally: {:#?}", csv_file_path);
            cf.to_csv(&csv_file_path.unwrap());
        }
    
        Ok(cf)
    }

    fn to_csv(&self, csv_filepath: &PathBuf) -> () {

        // let mut rows: Vec<CompactFlashScanCsvRow> = Vec::new();

        let sets = self.ot_sets.clone();

        let mut wtr = Writer::from_writer(vec![]);

        for s in sets {
            for audio_pool_sample in s.audio_pool.samples {

                let row = CompactFlashScanCsvRow {
                    cfcard: self.cfcard_path.file_name().unwrap().to_str().unwrap().to_string(),
                    set: s.name.clone(),
                    is_set_audio_pool: true,
                    project: None,
                    audio_filepath: audio_pool_sample.audio_path,
                    ot_filepath: audio_pool_sample.otfile_path,
                    audio_name: audio_pool_sample.name,
                };

                let __ = wtr.serialize(row);
            }

            for project in &s.projects {
                for project_sample in &project.samples {

                    let row = CompactFlashScanCsvRow {
                        cfcard: self.cfcard_path.file_name().unwrap().to_str().unwrap().to_string(),
                        set: s.name.clone(),
                        is_set_audio_pool: true,
                        project: None,
                        audio_filepath: project_sample.audio_path.clone(),
                        ot_filepath: project_sample.otfile_path.clone(),
                        audio_name: project_sample.name.clone(),
                    };
    
                    let _ = wtr.serialize(row);
                }
            }
        }

        let mut file = File::create(csv_filepath).unwrap();
        let res: Result<(), std::io::Error> = file
            .write_all(&wtr.into_inner().unwrap())
            .map_err(|e| e)
        ;


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
    fn dummy_test() {
        let cfcard_path = PathBuf::from("data/tests/index-cf/DEV-OTsm/");
        let csv_path = PathBuf::from("./.otsm-index-cf.csv");

        let res = CompactFlashDrive::new(&cfcard_path, &None);

        assert!(res.is_ok());
    }

}