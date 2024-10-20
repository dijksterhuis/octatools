//! A 'Sample Directory' contains samples that a user might want to load onto an Octatrack compact flash card.

use std::{io::Read, fs::File, io::Write};
use std::path::PathBuf;

use csv::Writer;
use md5::Digest;
use base64ct::{Base64, Encoding};
use log::{error, info, warn, debug};
use serde::{Serialize, Deserialize};

use crate::audio::wavfile::scan_dir_path_for_wavfiles;


/// One audio file detected in a scanned directory of samples.

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct SamplesDirAudioFile {
    md5: String,
    path: PathBuf,
    name: String,
}

/// An index of a "sample's directory" -- i.e. where someone stores their samples.

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct SamplesDirIndex {
    dirpath: PathBuf,
    index_file_path: Option<PathBuf>,
    samples: Vec<SamplesDirAudioFile>,
}


impl SamplesDirIndex {

    fn scandir(dirpath: &PathBuf) -> Result<Vec<SamplesDirAudioFile>, ()> {
        let wav_file_paths: Vec<PathBuf> = scan_dir_path_for_wavfiles(&dirpath).unwrap();

        info!("Getting md5 hashes of files ...");
        let mut samples: Vec<SamplesDirAudioFile> = Vec::new();
        for wav_file_path in wav_file_paths {

            debug!("Getting md5 hash: file={:#?}", wav_file_path);

            let mut f: File = File::open(&wav_file_path).unwrap();
            let mut buf: Vec<u8> = vec![];
            let _: usize = f.read_to_end(&mut buf).unwrap();    
            let md5_hash: Digest = md5::compute(buf);

            let mut buf = [0u8; 32];
            // let hash_hex = base16ct::lower::encode_str(&md5_hash[..], &mut buf).unwrap();

            // this doc is wrong, but i hope this works
            // https://github.com/RustCrypto/hashes/tree/master?tab=readme-ov-file
            let base64_hash = Base64
                ::encode(&md5_hash[..], &mut  buf)
                .unwrap()
            ;

            let wav_file_name = wav_file_path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
            ;

            debug!(
                "Got md5 hash: file={:#?} md5={:#?}", 
                wav_file_path, 
                base64_hash,
            );

            let sample = SamplesDirAudioFile {
                path: wav_file_path,
                md5: base64_hash.to_string(),
                name: wav_file_name
            };

            samples.push(sample);
        };
    
        info!("Got md5 hashes of files.");

        Ok(samples)

    }

    // TODO: Refactor to write CSV index lines in scandir (if argument provided).
    pub fn new(dirpath: &PathBuf, csv_file_path: &Option<PathBuf>) -> Result<SamplesDirIndex, ()> {

        info!("Generating new sample directory index ...");

        let samples = SamplesDirIndex::scandir(&dirpath).unwrap();

        if ! csv_file_path.is_none() {

            let mut wtr = Writer::from_writer(vec![]);

            for sample in &samples {
                let __ = wtr.serialize(sample);
            };
        
            let csv_fp = csv_file_path.clone();
            let mut file = File::create(csv_fp.unwrap()).unwrap();

            info!(
                "Writing sample index to CSV: path={:#?}",
                &csv_file_path.clone().unwrap(),
            );

            let _res: Result<(), std::io::Error> = file
                .write_all(&wtr.into_inner().unwrap())
                .map_err(|e| e)
            ;
        };

        info!("Generated new sample directory index.");

        Ok(
            SamplesDirIndex {
                dirpath: dirpath.clone(),
                index_file_path: csv_file_path.clone(),
                samples: samples.clone(),
            }
        )

    }

}

