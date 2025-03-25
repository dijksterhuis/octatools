//! Module for various utility functions and structs.

use crate::audio::wav::WavFile;
use crate::RBoxErr;
use ot_tools_lib::{constants::DEFAULT_SAMPLE_RATE, projects::slots::ProjectSampleSlot};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

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
pub fn get_bin_nbars_ileaved_wavfiles(
    wavs: &[WavFile],
    tempo_bpm: &f32,
    n_channels: u16,
) -> RBoxErr<u32> {
    let total_samples_interleaved: u32 = wavs.iter().map(|x| x.len).sum();
    let real_sample_length = total_samples_interleaved / n_channels as u32;
    let beats = real_sample_length as f32 / (DEFAULT_SAMPLE_RATE as f32 * 60.0 * 4.0);
    let bars = (tempo_bpm * beats * 100.0).round();
    Ok(bars as u32)
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
    /// Name of this Sample (file basename)
    pub name: String,

    /// Explicit path to the **audio** file.
    pub audio_filepath: PathBuf,

    /// Explicit path to the **Octatrack attributes** file.
    pub attributes_filepath: Option<PathBuf>,
}

impl SampleFilePair {
    /// Create a new `OctatrackSampleFile` from the audio file path
    /// and an optional attributes file path.
    #[allow(dead_code)] // I want to keep this in case it becomes useful in future.
    pub fn from_pathbufs(audio_fp: &Path, ot_fp: &Option<PathBuf>) -> RBoxErr<Self> {
        Ok(Self {
            name: audio_fp.file_stem().unwrap().to_str().unwrap().to_string(),
            audio_filepath: audio_fp.to_owned(),
            attributes_filepath: ot_fp.clone(),
        })
    }

    /// Create a new `OctatrackSampleFile` only from  the audio file path
    pub fn from_audio_pathbuf(audio_fp: &Path) -> RBoxErr<Self> {
        // TODO: optimise this? so many clones
        let mut ot_file_path = audio_fp.to_path_buf();
        ot_file_path.set_extension("ot");

        let mut ot_file_pathbuf = Some(ot_file_path.clone());
        if !ot_file_path.exists() {
            ot_file_pathbuf = None
        };

        Ok(Self {
            name: audio_fp.file_stem().unwrap().to_str().unwrap().to_string(),
            audio_filepath: audio_fp.to_owned(),
            attributes_filepath: ot_file_pathbuf,
        })
    }
}

// TODO: Delete me
/// All samples related to the project
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ProjectSamples {
    /// Samples loaded into project sample slots
    active: Vec<ProjectSampleSlot>,

    /// Samples in a project directory, but not loaded into a sample slot.
    inactive: Vec<SampleFilePair>,
}

#[cfg(test)]
#[allow(unused_imports)]
mod test {
    mod nbars_from_wav {

        use crate::audio::wav::WavFile;
        use crate::utils::get_otsample_nbars_from_wavfile;
        use ot_tools_lib::samples::slices::Slice;
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
        use crate::utils::get_bin_nbars_ileaved_wavfiles;
        use ot_tools_lib::samples::slices::Slice;
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

            assert!(get_bin_nbars_ileaved_wavfiles(&wavs, &120.0, 2).is_ok())
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

            let nbarsx100 = get_bin_nbars_ileaved_wavfiles(&wavs, &120.0, 2).unwrap();
            assert_eq!(nbarsx100, 158)
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

            let nbarsx100 = get_bin_nbars_ileaved_wavfiles(&wavs, &150.0, 2).unwrap();
            assert_eq!(nbarsx100, 198)
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

            let nbarsx100 = get_bin_nbars_ileaved_wavfiles(&wavs, &200.0, 2).unwrap();
            assert_eq!(nbarsx100, 264)
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

            let nbarsx100 = get_bin_nbars_ileaved_wavfiles(&wavs, &300.0, 2).unwrap();
            assert_eq!(nbarsx100, 396)
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

            let nbarsx100 = get_bin_nbars_ileaved_wavfiles(&wavs, &30.0, 2).unwrap();
            assert_eq!(nbarsx100, 40)
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
