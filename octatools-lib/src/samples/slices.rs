//! Slice data structs for sample attribute files (`SampleAttributes`).

use crate::samples::SwapBytes;
use crate::RBoxErr;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
enum SliceError {
    InvalidLoopPoint,
    InvalidTrim,
}
impl std::fmt::Display for SliceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidLoopPoint => write!(f, "slice loop point invalid"),
            Self::InvalidTrim => write!(f, "slice trim settings invalid"),
        }
    }
}
impl std::error::Error for SliceError {
    fn description(&self) -> &str {
        match *self {
            SliceError::InvalidLoopPoint => "slice loop point invalid",
            SliceError::InvalidTrim => "slice trim settings invalid",
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        match *self {
            SliceError::InvalidLoopPoint => None,
            SliceError::InvalidTrim => None,
        }
    }
}

/// Positions of a 'slice' within a single WAV file.
/// IMPORTANT: slice points are not measured in bars like `SampleAttributes`,
/// but instead use *audio sample* positions from the audio file.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub struct Slice {
    /// Start position for the `Slice`.
    pub trim_start: u32,

    /// End position for the `Slice`.
    pub trim_end: u32,

    /// Loop start position for the `Slice`. This is actually `Loop Point` in
    /// the Octatrack manual.
    /// > If a loop point is set, the sample will play from the start point to the
    /// > end point, then loop from the loop point to the end point
    ///
    /// Note that a `0xFFFFFFFF` value disables the loop point for the slice.
    pub loop_start: u32,
}

const SLICE_LOOP_POINT_DEFAULT: u32 = 0xFFFFFFFF;

impl Slice {
    pub fn new(trim_start: u32, trim_end: u32, loop_start: Option<u32>) -> RBoxErr<Self> {
        if trim_start > trim_end {
            return Err(SliceError::InvalidTrim.into());
        }

        // default is disabled
        let loop_point = loop_start.unwrap_or(SLICE_LOOP_POINT_DEFAULT);

        if loop_point != SLICE_LOOP_POINT_DEFAULT && !(trim_start..trim_end).contains(&loop_point) {
            return Err(SliceError::InvalidLoopPoint.into());
        }

        Ok(Self {
            trim_start,
            trim_end,
            loop_start: loop_point,
        })
    }
}

impl SwapBytes for Slice {
    type T = Slice;
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
#[derive(Debug)]
pub struct Slices {
    /// `Slice` objects, must be 64 elements in length.
    pub slices: [Slice; 64],

    /// Number of non-zero valued `Slice` objects in the `slices` field array.
    pub count: u32,
}

#[cfg(test)]
mod test {

    use crate::samples::slices::Slice;

    #[test]
    fn ok_no_offset_no_loop() {
        let valid = Slice {
            trim_start: 0,
            trim_end: 1000,
            loop_start: 0xFFFFFFFF,
        };

        let s = Slice::new(0, 1000, None);

        assert!(s.is_ok());
        assert_eq!(valid, s.unwrap());
    }

    #[test]
    fn ok_loop_point() {
        let valid = Slice {
            trim_start: 0,
            trim_end: 1000,
            loop_start: 0,
        };

        let s = Slice::new(0, 1000, Some(0));

        assert!(s.is_ok());
        assert_eq!(valid, s.unwrap());
    }

    #[test]
    fn err_loop_end() {
        let s = Slice::new(0, 1000, Some(1000));
        assert!(s.is_err());
        assert_eq!(s.unwrap_err().to_string(), "slice loop point invalid");
    }

    #[test]
    fn err_trim() {
        let s = Slice::new(1001, 1000, None);
        assert!(s.is_err());
        assert_eq!(s.unwrap_err().to_string(), "slice trim settings invalid");
    }

    #[test]
    fn ok_offset_100_no_loop() {
        let valid = Slice {
            trim_start: 100,
            trim_end: 1100,
            loop_start: 0xFFFFFFFF,
        };

        let s = Slice::new(100, 1100, None);

        assert!(s.is_ok());
        assert_eq!(valid, s.unwrap());
    }

    #[test]
    fn ok_offset_100_with_loop_200() {
        let valid = Slice {
            trim_start: 100,
            trim_end: 1100,
            loop_start: 200,
        };

        let s = Slice::new(100, 1100, Some(200));

        assert!(s.is_ok());
        assert_eq!(valid, s.unwrap());
    }
}
