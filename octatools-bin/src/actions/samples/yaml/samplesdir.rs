//! A 'Sample Directory' contains samples that a user might want to load onto an Octatrack compact flash card.

use std::path::PathBuf;
use std::{fs::File, io::Read};

use crate::audio::utils::scan_dir_path_for_audio_files;
use crate::{OctatoolErrors, RBoxErr};
use base64ct::{Base64, Encoding};
use log::{debug, info};
use md5::Digest;
use serde::{Deserialize, Serialize};
use serde_octatrack::{Decode, Encode};
use std::fs::canonicalize;

fn get_stem_from_pathbuf(pathbuf: &PathBuf) -> RBoxErr<String> {
    debug!("Getting file stem from pathbuf: file={pathbuf:#?}");

    if !pathbuf.exists() {
        return Err(Box::new(OctatoolErrors::PathDoesNotExist));
    }
    if pathbuf.is_dir() {
        return Err(Box::new(OctatoolErrors::PathIsNotAFile));
    }
    let fname = pathbuf.file_stem().unwrap().to_str().unwrap().to_string();
    debug!("Got file stem from pathbuf: file={pathbuf:#?}");
    Ok(fname)
}

fn get_md5_hash_from_pathbuf(pathbuf: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    debug!("Getting md5 hash: file={pathbuf:#?}");

    let mut f: File = File::open(pathbuf)?;
    let mut buf: Vec<u8> = vec![];
    let _: usize = f.read_to_end(&mut buf)?;
    let md5_hash: Digest = md5::compute(buf);

    let mut buf = [0u8; 32];

    // TODO: hard exit on error
    /*
    the trait bound `InvalidLengthError: StdError` is not satisfied

    the following other types implement trait `FromResidual<R>`:
    `Result<T, F>` implements `FromResidual<Result<Infallible, E>>`
    `Result<T, F>` implements `FromResidual<Yeet<E>>`

    required for `Box<dyn StdError>` to implement `From<InvalidLengthError>`
    required for `Result<std::string::String, Box<dyn StdError>>` to implement `FromResidual<Result<Infallible, InvalidLengthError>>`rustc(Click for full compiler diagnostic)
    */

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

impl Decode for SamplesDirIndexFull {}
impl Encode for SamplesDirIndexFull {}

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

impl Decode for SamplesDirIndexSimple {}
impl Encode for SamplesDirIndexSimple {}

impl SamplesDirIndexSimple {
    pub fn new(dirpath: &PathBuf) -> RBoxErr<Self> {
        info!("Generating simple sample directory index ...");

        let samples: Vec<PathBuf> = scan_dir_path_for_audio_files(dirpath)?;

        let index = SamplesDirIndexSimple {
            samples,
            dirpath: canonicalize(dirpath)?,
        };

        info!("Generated simple sample directory index.");

        Ok(index)
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod test {
    use super::*;

    mod test_get_stem_from_pathbuf {
        use super::*;

        #[test]
        fn fail_when_noexist() {
            assert!(get_stem_from_pathbuf(&PathBuf::from("sfklsdfjsdkfdjsdslfdljfdlkj")).is_err())
        }

        #[test]
        fn fail_when_file_is_dir() {
            assert!(get_stem_from_pathbuf(&PathBuf::from("./src/")).is_err())
        }
    }
}
