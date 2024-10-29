//! Helper / Grouped configs for sample attribute files (`SampleAttributes`).

use std::error::Error;

use crate::octatrack::common::OptionEnumValueConvert;
use crate::octatrack::options::SampleAttributeLoopMode;
use crate::octatrack::samples::SampleAttributes;

/// An OT Sample's Trim settings

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct SampleTrimConfig {
    /// Start of full audio sample (n samples)
    pub start: u32,
    /// End of full audio sample (n samples)
    pub end: u32,
    /// Length of audio sample to play before stopping/looping playback (n samples)
    pub length: u32,
}

// impl SampleTrimConfig {
//     pub fn from_decoded(decoded: &SampleChain) -> Result<Self, Box<dyn Error>> {
//         let new = SampleTrimConfig {
//             start: decoded.trim_start,
//             end: decoded.trim_end,
//             length: decoded.trim_len,
//         };
//         Ok(new)
//     }
// }

/// An OT Sample's Loop settings

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct SampleLoopConfig {
    /// Loop start position for the audio sample (n samples).
    pub start: u32,

    /// Length of the loop for the audio sample (n samples).
    pub length: u32,

    /// Type of looping mode.
    pub mode: SampleAttributeLoopMode,
}

impl SampleLoopConfig {
    pub fn new(start: u32, length: u32, mode: SampleAttributeLoopMode) -> Self {
        SampleLoopConfig {
            start,
            length,
            mode,
        }
    }

    pub fn from_decoded(decoded: &SampleAttributes) -> Result<Self, Box<dyn Error>> {
        Ok(Self::new(
            decoded.loop_start,
            decoded.loop_len,
            SampleAttributeLoopMode::from_value(decoded.loop_mode)
                .unwrap_or(SampleAttributeLoopMode::Off),
        ))
    }
}
