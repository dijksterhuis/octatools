//! Module for various utility functions and structs.

use crate::audio::wav::WavFile;
use crate::RBoxErr;
use serde::{Deserialize, Serialize};
use serde_octatrack::{
    constants::DEFAULT_SAMPLE_RATE,
    projects::slots::ProjectSampleSlot,
    samples::slices::{Slice, Slices},
};
use std::path::PathBuf;

/// Create a `Slice` object for an unchained wavfile.
/// The starting `offset` position should be the sample index within the eventual chained wavfile.
pub fn create_slice_from_wavfile(wavfile: &WavFile, trim_start: u32) -> RBoxErr<Slice> {
    Ok(Slice {
        trim_start,
        trim_end: trim_start + wavfile.len,
        loop_start: 0xFFFFFFFF,
    })
}

/// Get a new `Slices` struct, given a `Vec` of `WavFile`s.
pub fn create_slices_from_wavfiles(wavfiles: &Vec<WavFile>, offset: u32) -> RBoxErr<Slices> {
    let mut new_slices: Vec<Slice> = Vec::new();
    let mut off = offset;

    for w in wavfiles.iter() {
        new_slices.push(create_slice_from_wavfile(w, off).unwrap());
        off += w.len;
    }

    let default_slice = Slice {
        trim_end: 0,
        trim_start: 0,
        loop_start: 0,
    };

    let mut slices_arr: [Slice; 64] = [default_slice; 64];
    for (i, slice_vec) in new_slices.iter().enumerate() {
        slices_arr[i] = *slice_vec;
    }

    Ok(Slices {
        slices: slices_arr,
        count: wavfiles.len() as u32,
    })
}

/// Calculate the effective number of bars for a single wav file.
/// Assumes four beats per bar.
pub fn get_otsample_nbars_from_wavfile(wav: &WavFile, tempo_bpm: &f32) -> RBoxErr<u32> {
    let beats = wav.len as f32 / (DEFAULT_SAMPLE_RATE as f32 * 60.0 * 4.0);
    let mut bars = ((tempo_bpm * 4.0 * beats) + 0.5) * 0.25;
    bars -= bars % 0.25;
    Ok((bars * 100.0) as u32)
}

/// Calculate the effective number of bars for a vec of wav files.
/// Assumes four beats per bar.
pub fn get_otsample_nbars_from_wavfiles(wavs: &Vec<WavFile>, tempo_bpm: &f32) -> RBoxErr<u32> {
    let total_samples: u32 = wavs.iter().map(|x| x.len).sum();
    let beats = total_samples as f32 / (DEFAULT_SAMPLE_RATE as f32 * 60.0 * 4.0);
    let mut bars = ((tempo_bpm * 4.0 * beats) + 0.5) * 0.25;
    bars -= bars % 0.25;
    Ok((bars * 100.0) as u32)
}

/// Each 'sample' can have two files present on an Octatrack:
/// the audio file and the corresponding `.ot` attributes file.
/// This struct represents one 'sample' as a combination of those two file paths.
///
/// Note: The `samples` module is reserved for ser/de of `SampleAttributes` files (`.ot` files),
/// and this struct is only relevant for Projects anyway.

// NOTE: samples can be stored either in the set's audio pool or in the project directory

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct SampleFilePair {
    /// Name of this Sample (file basenames)
    pub name: String,

    /// Explicit path to the **audio** file.
    pub audio_filepath: PathBuf,

    /// Explicit path to the **Octatrack attributes** file.
    pub attributes_filepath: Option<PathBuf>,
}

impl SampleFilePair {
    /// Create a new `OctatrackSampleFile` from the audio file path
    /// and an optional attributes file path.

    pub fn from_pathbufs(audio_fp: &PathBuf, ot_fp: &Option<PathBuf>) -> RBoxErr<Self> {
        Ok(Self {
            name: audio_fp.file_stem().unwrap().to_str().unwrap().to_string(),
            audio_filepath: audio_fp.clone(),
            attributes_filepath: ot_fp.clone(),
        })
    }

    /// Create a new `OctatrackSampleFile` only from  the audio file path

    pub fn from_audio_pathbuf(audio_fp: &PathBuf) -> RBoxErr<Self> {
        // TODO: optimise this? so many clones
        let mut ot_file_path = audio_fp.clone();
        ot_file_path.set_extension("ot");

        let mut ot_file_pathbuf = Some(ot_file_path.clone());
        if !ot_file_path.exists() {
            ot_file_pathbuf = None
        };

        Ok(Self {
            name: audio_fp.file_stem().unwrap().to_str().unwrap().to_string(),
            audio_filepath: audio_fp.clone(),
            attributes_filepath: ot_file_pathbuf,
        })
    }
}

/// All samples related to the project

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ProjectSamples {
    /// Samples loaded into project sample slots
    active: Vec<ProjectSampleSlot>,

    /// Samples in a project directory, but not loaded into a sample slot.
    inactive: Vec<SampleFilePair>,
}

mod test {

    mod slice_from_wav {

        use crate::audio::wav::WavFile;
        use crate::utils::create_slice_from_wavfile;
        use serde_octatrack::samples::slices::Slice;
        use serde_octatrack::FromPath;
        use std::path::PathBuf;

        #[test]
        fn no_offset_ok() {
            let fp = PathBuf::from("../data/tests/misc/test.wav");
            let wav = WavFile::from_path(&fp).unwrap();

            let valid = Slice {
                trim_start: 0,
                trim_end: wav.len,
                loop_start: 0xFFFFFFFF,
            };

            assert!(create_slice_from_wavfile(&wav, 0).is_ok())
        }

        #[test]
        fn no_offset_validated() {
            let fp = PathBuf::from("../data/tests/misc/test.wav");
            let wav = WavFile::from_path(&fp).unwrap();

            let valid = Slice {
                trim_start: 0,
                trim_end: wav.len,
                loop_start: 0xFFFFFFFF,
            };

            assert_eq!(create_slice_from_wavfile(&wav, 0).unwrap(), valid)
        }

        #[test]
        fn offset_100_validated() {
            let fp = PathBuf::from("../data/tests/misc/test.wav");
            let wav = WavFile::from_path(&fp).unwrap();

            let valid = Slice {
                trim_start: 100,
                trim_end: 100 + wav.len,
                loop_start: 0xFFFFFFFF,
            };

            assert_eq!(create_slice_from_wavfile(&wav, 100).unwrap(), valid)
        }
    }

    mod slices_from_wavs {

        use crate::audio::wav::WavFile;
        use crate::utils::create_slices_from_wavfiles;
        use serde_octatrack::samples::slices::Slice;
        use serde_octatrack::FromPath;
        use std::path::PathBuf;

        #[test]
        fn no_offset_ok() {
            let fp = PathBuf::from("../data/tests/misc/test.wav");
            let wav = WavFile::from_path(&fp).unwrap();
            let wavs = [
                wav.clone(),
                wav.clone(),
                wav.clone(),
                wav.clone(),
                wav.clone(),
            ]
            .to_vec();

            assert!(create_slices_from_wavfiles(&wavs, 0).is_ok())
        }

        #[test]
        fn offset_100_ok() {
            let fp = PathBuf::from("../data/tests/misc/test.wav");
            let wav = WavFile::from_path(&fp).unwrap();
            let wavs = [
                wav.clone(),
                wav.clone(),
                wav.clone(),
                wav.clone(),
                wav.clone(),
            ]
            .to_vec();

            assert!(create_slices_from_wavfiles(&wavs, 100).is_ok())
        }

        #[test]
        fn offset_30000_ok() {
            let fp = PathBuf::from("../data/tests/misc/test.wav");
            let wav = WavFile::from_path(&fp).unwrap();
            let wavs = [
                wav.clone(),
                wav.clone(),
                wav.clone(),
                wav.clone(),
                wav.clone(),
            ]
            .to_vec();

            assert!(create_slices_from_wavfiles(&wavs, 30000).is_ok())
        }
    }

    mod nbars_from_wav {

        use crate::audio::wav::WavFile;
        use crate::utils::get_otsample_nbars_from_wavfile;
        use serde_octatrack::samples::slices::Slice;
        use serde_octatrack::FromPath;
        use std::path::PathBuf;

        #[test]
        fn simple_ok() {
            let fp = PathBuf::from("../data/tests/misc/test.wav");
            let wav = WavFile::from_path(&fp).unwrap();

            assert!(get_otsample_nbars_from_wavfile(&wav, &120.0).is_ok())
        }

        #[test]
        fn simple_120bpm_valid() {
            let fp = PathBuf::from("../data/tests/misc/test.wav");
            let wav = WavFile::from_path(&fp).unwrap();

            let nbarsx100 = get_otsample_nbars_from_wavfile(&wav, &120.0).unwrap();
            assert_eq!(nbarsx100, 75)
        }

        #[test]
        fn simple_300bpm_valid() {
            let fp = PathBuf::from("../data/tests/misc/test.wav");
            let wav = WavFile::from_path(&fp).unwrap();

            let nbarsx100 = get_otsample_nbars_from_wavfile(&wav, &300.0).unwrap();
            assert_eq!(nbarsx100, 150)
        }

        #[test]
        fn simple_150bpm_valid() {
            let fp = PathBuf::from("../data/tests/misc/test.wav");
            let wav = WavFile::from_path(&fp).unwrap();

            let nbarsx100 = get_otsample_nbars_from_wavfile(&wav, &150.0).unwrap();
            assert_eq!(nbarsx100, 75)
        }

        #[test]
        fn simple_200bpm_valid() {
            let fp = PathBuf::from("../data/tests/misc/test.wav");
            let wav = WavFile::from_path(&fp).unwrap();

            let nbarsx100 = get_otsample_nbars_from_wavfile(&wav, &200.0).unwrap();
            assert_eq!(nbarsx100, 100)
        }

        #[test]
        fn simple_30bpm_valid() {
            let fp = PathBuf::from("../data/tests/misc/test.wav");
            let wav = WavFile::from_path(&fp).unwrap();

            let nbarsx100 = get_otsample_nbars_from_wavfile(&wav, &30.0).unwrap();
            assert_eq!(nbarsx100, 25)
        }
    }

    mod nbars_from_wavs {

        use crate::audio::wav::WavFile;
        use crate::utils::get_otsample_nbars_from_wavfiles;
        use serde_octatrack::samples::slices::Slice;
        use serde_octatrack::FromPath;
        use std::path::PathBuf;

        #[test]
        fn simple_ok() {
            let fp = PathBuf::from("../data/tests/misc/test.wav");
            let wav = WavFile::from_path(&fp).unwrap();
            let wavs = [
                wav.clone(),
                wav.clone(),
                wav.clone(),
                wav.clone(),
                wav.clone(),
            ]
            .to_vec();

            assert!(get_otsample_nbars_from_wavfiles(&wavs, &120.0).is_ok())
        }

        #[test]
        fn simple_120bpm_valid() {
            let fp = PathBuf::from("../data/tests/misc/test.wav");
            let wav = WavFile::from_path(&fp).unwrap();
            let wavs = [
                wav.clone(),
                wav.clone(),
                wav.clone(),
                wav.clone(),
                wav.clone(),
            ]
            .to_vec();

            let nbarsx100 = get_otsample_nbars_from_wavfiles(&wavs, &120.0).unwrap();
            assert_eq!(nbarsx100, 325)
        }

        #[test]
        fn simple_300bpm_valid() {
            let fp = PathBuf::from("../data/tests/misc/test.wav");
            let wav = WavFile::from_path(&fp).unwrap();
            let wavs = [
                wav.clone(),
                wav.clone(),
                wav.clone(),
                wav.clone(),
                wav.clone(),
            ]
            .to_vec();

            let nbarsx100 = get_otsample_nbars_from_wavfiles(&wavs, &300.0).unwrap();
            assert_eq!(nbarsx100, 800)
        }

        #[test]
        fn simple_150bpm_valid() {
            let fp = PathBuf::from("../data/tests/misc/test.wav");
            let wav = WavFile::from_path(&fp).unwrap();
            let wavs = [
                wav.clone(),
                wav.clone(),
                wav.clone(),
                wav.clone(),
                wav.clone(),
            ]
            .to_vec();

            let nbarsx100 = get_otsample_nbars_from_wavfiles(&wavs, &150.0).unwrap();
            assert_eq!(nbarsx100, 400)
        }

        #[test]
        fn simple_200bpm_valid() {
            let fp = PathBuf::from("../data/tests/misc/test.wav");
            let wav = WavFile::from_path(&fp).unwrap();
            let wavs = [
                wav.clone(),
                wav.clone(),
                wav.clone(),
                wav.clone(),
                wav.clone(),
            ]
            .to_vec();

            let nbarsx100 = get_otsample_nbars_from_wavfiles(&wavs, &200.0).unwrap();
            assert_eq!(nbarsx100, 525)
        }

        #[test]
        fn simple_30bpm_valid() {
            let fp = PathBuf::from("../data/tests/misc/test.wav");
            let wav = WavFile::from_path(&fp).unwrap();
            let wavs = [
                wav.clone(),
                wav.clone(),
                wav.clone(),
                wav.clone(),
                wav.clone(),
            ]
            .to_vec();

            let nbarsx100 = get_otsample_nbars_from_wavfiles(&wavs, &30.0).unwrap();
            assert_eq!(nbarsx100, 75)
        }
    }

    mod sample_file_pair {

        use crate::utils::SampleFilePair;
        use std::path::PathBuf;

        #[test]
        fn test_read_audio_file_only_ok() {
            let fp = PathBuf::from("../data/tests/misc/test.wav");
            let r = SampleFilePair::from_audio_pathbuf(&fp);
            assert!(r.is_ok())
        }

        #[test]
        fn test_read_file_pair_ok() {
            let fp = PathBuf::from("../data/tests/misc/pair.wav");
            let r = SampleFilePair::from_audio_pathbuf(&fp);
            assert!(r.is_ok())
        }
    }
}
