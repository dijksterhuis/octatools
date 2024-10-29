//! Slice data structs for sample attribute files (`SampleAttributes`).

use crate::audio::wavfile::WavFile;
use serde::{Deserialize, Serialize};
use std::error::Error;

/// Positions of a 'slice' within a single WAV file
/// (a sliced WAV file is multiple samples joined in series)

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub struct Slice {
    /// Start position for the `Slice`.
    pub trim_start: u32,

    /// End position for the `Slice`.
    pub trim_end: u32,

    /// Loop start position for the `Slice`.
    pub loop_start: u32,
}

impl Slice {
    /// Swap bytes for all struct fields

    pub fn as_bswapped(&self) -> Self {
        Self {
            trim_start: self.trim_start.swap_bytes(),
            trim_end: self.trim_end.swap_bytes(),
            loop_start: self.loop_start.swap_bytes(),
        }
    }

    /// Create a `Slice` object for an unchained wavfile.
    /// The starting `offset` position should be the sample index within the eventual chained wavfile.

    pub fn from_wavfile(wavfile: &WavFile, offset: u32) -> Result<Self, Box<dyn Error>> {
        Ok(Slice {
            trim_start: 0 + offset,
            trim_end: offset + wavfile.len,
            loop_start: 0xFFFFFFFF,
        })
    }
}

/// A collection of `Slice` objects.

pub struct Slices {
    /// `Slice` objects, must be 64 elements in length.
    pub slices: [Slice; 64],

    /// Number of non-zero valued `Slice` objects in the `slices` field array.
    pub count: u32,
}

impl Slices {
    /// Get a new `Vec` of Slices, given a `Vec` of `WavFile`s and a starting position offset.

    fn get_vec_from_wavfiles(
        wavfiles: &Vec<WavFile>,
        initial_offset: &u32,
    ) -> Result<Vec<Slice>, Box<dyn Error>> {
        let mut off = initial_offset.clone();
        let mut slices: Vec<Slice> = Vec::new();

        for w in wavfiles.iter() {
            slices.push(Slice::from_wavfile(w, off).unwrap());
            off += w.len as u32;
        }

        Ok(slices)
    }

    /// Get a new `Slices` struct, given a `Vec` of `WavFile`s.

    pub fn from_wavfiles(wavfiles: &Vec<WavFile>, offset: &u32) -> Result<Self, Box<dyn Error>> {
        let new_slices: _ = Self::get_vec_from_wavfiles(&wavfiles, &offset).unwrap();

        let default_slice = Slice {
            trim_end: 0,
            trim_start: 0,
            loop_start: 0,
        };

        let mut slices_arr: [Slice; 64] = [default_slice; 64];
        for (i, slice_vec) in new_slices.iter().enumerate() {
            slices_arr[i] = slice_vec.clone();
        }

        Ok(Self {
            slices: slices_arr,
            count: wavfiles.len() as u32,
        })
    }
}
