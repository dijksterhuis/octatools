//! Slice data structs for sample attribute files (`SampleAttributes`).

// use crate::audio::wavfile::WavFile;
use crate::samples::SwapBytes;
use crate::RBoxErr;
use serde::{Deserialize, Serialize};

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

impl SwapBytes for Slice {
    /// Swaps the bytes on all struct fields.
    /// **MUST BE CALLED BEFORE SERIALISATION WHEN SYSTEM IS LITTLE ENDIAN!**

    type T = Slice;

    /// Swap the bytes of all struct fields.
    /// Must be applied to the `SampleAttributes` file to deal with litle-endian/big-endian systems.
    fn swap_bytes(self) -> RBoxErr<Self::T> {
        let bswapped = Self {
            trim_start: self.trim_start.swap_bytes(),
            trim_end: self.trim_end.swap_bytes(),
            loop_start: self.loop_start.swap_bytes(),
        };

        Ok(bswapped)
    }
}

/// A collection of `Slice` objects.
pub struct Slices {
    /// `Slice` objects, must be 64 elements in length.
    pub slices: [Slice; 64],

    /// Number of non-zero valued `Slice` objects in the `slices` field array.
    pub count: u32,
}
