//! A 'Sample Directory' contains samples that a user might want to load onto an Octatrack compact flash card.

use std::path::PathBuf;
use std::{fs::File, io::Read};

use base64ct::{Base64, Encoding};
use log::{debug, error, info};
use md5::Digest;
use serde::{Deserialize, Serialize};
use std::fs::canonicalize;

use crate::audio::utils::scan_dir_path_for_audio_files;
use crate::common::RBoxErr;
use serde_octatrack::{FromYamlFile, ToYamlFile};

fn get_stem_from_pathbuf(pathbuf: &PathBuf) -> Result<String, ()> {
    debug!("Getting file path's file name: file={pathbuf:#?}");

    // TODO: More idiomatic way of handling None values
    let fname_osstr = pathbuf.file_stem();
    if fname_osstr.is_none() {
        error!("Could not get file path's file name: file={pathbuf:#?}");
        return Err(());
    }

    // TODO: More idiomatic way of handling None values
    let file_name_str = fname_osstr.unwrap().to_str();
    if file_name_str.is_none() {
        error!("Could not get file path's file name: file={pathbuf:#?}");
        return Err(());
    }

    let file_name = file_name_str.unwrap().to_string();

    debug!("Got file path's file name: file={pathbuf:#?} md5={file_name:#?}");

    Ok(file_name)
}

fn get_md5_hash_from_pathbuf(pathbuf: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    debug!("Getting md5 hash: file={pathbuf:#?}");

    let mut f: File = File::open(pathbuf)?;
    let mut buf: Vec<u8> = vec![];
    let _: usize = f.read_to_end(&mut buf)?;
    let md5_hash: Digest = md5::compute(buf);

    let mut buf = [0u8; 32];

    // TODO: hard exit on error
    let md5_hash_string = Base64::encode(&md5_hash[..], &mut buf).unwrap().to_string();

    debug!("Got md5 hash: file={pathbuf:#?} md5={md5_hash_string:#?}");

    Ok(md5_hash_string)
}

/// One audio file detected in a scanned directory of samples.

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct SamplesDirAudioFile {
    pub md5: String,
    pub path: PathBuf,
    pub name: String,
}

impl SamplesDirAudioFile {
    pub fn new(fp: PathBuf) -> RBoxErr<Self> {
        let md5_hash = get_md5_hash_from_pathbuf(&fp)?;

        // todo: unwrap failure
        let file_name = get_stem_from_pathbuf(&fp).unwrap();

        Ok(SamplesDirAudioFile {
            path: canonicalize(fp)?,
            md5: md5_hash,
            name: file_name,
        })
    }
}

/// An index of a "sample's directory" -- i.e. where someone stores their samples.

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct SamplesDirIndexFull {
    pub dirpath: PathBuf,
    pub samples: Vec<SamplesDirAudioFile>,
}

impl FromYamlFile for SamplesDirIndexFull {}
impl ToYamlFile for SamplesDirIndexFull {}

impl SamplesDirIndexFull {
    pub fn new(dirpath: &PathBuf) -> RBoxErr<Self> {
        info!("Generating new sample directory index ...");

        // TODO: Hard exit on failure
        let wav_file_paths: Vec<PathBuf> = scan_dir_path_for_audio_files(dirpath).unwrap();

        let samples: Vec<SamplesDirAudioFile> = wav_file_paths
            .into_iter()
            // TODO: Hard exit on failure
            .map(|fp: PathBuf| SamplesDirAudioFile::new(fp).unwrap())
            .collect();

        let index = SamplesDirIndexFull {
            dirpath: canonicalize(dirpath)?,
            samples,
        };

        info!("Generated new sample directory index.");

        Ok(index)
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct SamplesDirIndexSimple {
    pub dirpath: PathBuf,
    pub samples: Vec<PathBuf>,
}

impl FromYamlFile for SamplesDirIndexSimple {}
impl ToYamlFile for SamplesDirIndexSimple {}

impl SamplesDirIndexSimple {
    pub fn new(dirpath: &PathBuf) -> RBoxErr<Self> {
        info!("Generating simple sample directory index ...");

        let samples: Vec<PathBuf> = scan_dir_path_for_audio_files(dirpath)?;

        // todo: clone
        let index = SamplesDirIndexSimple {
            samples,
            dirpath: canonicalize(dirpath)?,
        };

        info!("Generated simple sample directory index.");

        Ok(index)
    }
}
