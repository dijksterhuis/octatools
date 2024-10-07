use std::{io::Read, fs::File, io::Write};
use std::path::PathBuf;

use csv::Writer;
use md5::Digest;
use base64ct::{Base64, Encoding};
use log::{error, info, warn, debug};
use serde::{Serialize, Deserialize};

use crate::wavfile::scan_dir_path_for_wavfiles;


#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct SamplesDirAudioFile {
    path: PathBuf,
    md5: String,
}


#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct SamplesDirIndex {
    dirpath: PathBuf,
    index_file_path: Option<PathBuf>,
    samples: Vec<SamplesDirAudioFile>,
}


impl SamplesDirIndex {

    fn scandir(dirpath: &PathBuf) -> Result<Vec<SamplesDirAudioFile>, ()> {
        let wav_file_paths: Vec<PathBuf> = scan_dir_path_for_wavfiles(&dirpath).unwrap();

        let mut samples: Vec<SamplesDirAudioFile> = Vec::new();
        for wav_file_path in wav_file_paths {
    
            let mut f: File = File::open(&wav_file_path).unwrap();
            let mut buf: Vec<u8> = vec![];
            let _: usize = f.read_to_end(&mut buf).unwrap();    
            let md5_hash: Digest = md5::compute(buf);

            let mut buf = [0u8; 16];
            // let hash_hex = base16ct::lower::encode_str(&md5_hash[..], &mut buf).unwrap();

            // this doc is wrong, but i hope this works
            // https://github.com/RustCrypto/hashes/tree/master?tab=readme-ov-file
            let base64_hash = Base64::encode(&md5_hash[..], &mut  buf).unwrap();

            let sample = SamplesDirAudioFile {
                path: wav_file_path,
                md5: base64_hash.to_string(),
            };

            samples.push(sample);
        };
    
        Ok(samples)

    }
    pub fn new(dirpath: &PathBuf, csv_file_path: &Option<PathBuf>) -> Result<SamplesDirIndex, ()> {

        info!("Generating new sample directory index ...");

        let samples = SamplesDirIndex::scandir(&dirpath).unwrap();

        let mut wtr = Writer::from_writer(vec![]);

        for sample in &samples {
            let __ = wtr.serialize(sample);
        };

        if ! csv_file_path.is_none() {

            let csv = csv_file_path.clone().unwrap();
            let mut file = File::create(csv).unwrap();
            let res: Result<(), std::io::Error> = file
                .write_all(&wtr.into_inner().unwrap())
                .map_err(|e| e)
            ;
        };

        info!("Generated new sample directory index ...");

        Ok(
            SamplesDirIndex {
                dirpath: dirpath.clone(),
                index_file_path: csv_file_path.clone(),
                samples: samples.clone(),
            }
        )

    }

}

