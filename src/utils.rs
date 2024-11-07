use crate::audio::wavfile::WavFile;
use serde_octatrack::common::RBoxErr;
use serde_octatrack::constants::DEFAULT_SAMPLE_RATE;
use serde_octatrack::samples::slices::{Slice, Slices};
use std::error::Error;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::common::RVoidError;
use serde_octatrack::projects::slots::ProjectSampleSlots;


/// Create a `Slice` object for an unchained wavfile.
/// The starting `offset` position should be the sample index within the eventual chained wavfile.
pub fn create_slice_from_wavfile(wavfile: &WavFile, offset: u32) -> Result<Slice, Box<dyn Error>> {
    Ok(Slice {
        trim_start: 0 + offset,
        trim_end: offset + wavfile.len,
        loop_start: 0xFFFFFFFF,
    })
}

/// Get a new `Vec` of Slices, given a `Vec` of `WavFile`s and a starting position offset.
pub fn get_vec_from_wavfiles(
    wavfiles: &Vec<WavFile>,
    initial_offset: &u32,
) -> Result<Vec<Slice>, Box<dyn Error>> {
    let mut off = initial_offset.clone();
    let mut slices: Vec<Slice> = Vec::new();

    for w in wavfiles.iter() {
        slices.push(create_slice_from_wavfile(w, off).unwrap());
        off += w.len as u32;
    }

    Ok(slices)
}

/// Get a new `Slices` struct, given a `Vec` of `WavFile`s.
pub fn create_slices_from_wavfiles(
    wavfiles: &Vec<WavFile>,
    offset: u32,
) -> Result<Slices, Box<dyn Error>> {
    let new_slices: _ = get_vec_from_wavfiles(&wavfiles, &offset).unwrap();

    let default_slice = Slice {
        trim_end: 0,
        trim_start: 0,
        loop_start: 0,
    };

    let mut slices_arr: [Slice; 64] = [default_slice; 64];
    for (i, slice_vec) in new_slices.iter().enumerate() {
        slices_arr[i] = slice_vec.clone();
    }

    Ok(Slices {
        slices: slices_arr,
        count: wavfiles.len() as u32,
    })
}

// TODO: Change to taking number of samples as argument
// so we can pass in a single wav or a vvector worth of wavs
// otherwise we have to mess around with switching about types

// TODO: Move to octatrack_common?

/// Calculate the effective number of bars for a sample / slice.
/// Assumes four beats per bar.

pub fn get_otsample_nbars_from_wavfiles(wavs: &Vec<WavFile>, tempo_bpm: &f32) -> RBoxErr<u32> {
    let total_samples: u32 = wavs.iter().map(|x| x.len as u32).sum();
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

    pub fn from_pathbufs(audio_fp: &PathBuf, ot_fp: &Option<PathBuf>) -> RVoidError<Self> {
        Ok(Self {
            name: audio_fp.file_stem().unwrap().to_str().unwrap().to_string(),
            audio_filepath: audio_fp.clone(),
            attributes_filepath: ot_fp.clone(),
        })
    }

    /// Create a new `OctatrackSampleFile` only from  the audio file path

    pub fn from_audio_pathbuf(audio_fp: &PathBuf) -> RVoidError<Self> {
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
    active: Vec<ProjectSampleSlots>,

    /// Samples in a project directory, but not loaded into a sample slot.
    inactive: Vec<SampleFilePair>,
}
