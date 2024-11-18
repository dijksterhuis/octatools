//! Helper / Grouped configs for sample attribute files (`SampleAttributes`).

use crate::RBoxErr;
use crate::{
    samples::options::SampleAttributeLoopMode, samples::SampleAttributes, OptionEnumValueConvert,
};

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

impl SampleTrimConfig {
    pub fn from_decoded(decoded: &SampleAttributes) -> RBoxErr<Self> {
        let new = SampleTrimConfig {
            start: decoded.trim_start,
            end: decoded.trim_end,
            length: decoded.trim_len,
        };
        Ok(new)
    }
}

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
    pub fn new(start: u32, length: u32, mode: SampleAttributeLoopMode) -> RBoxErr<Self> {
        Ok(Self {
            start,
            length,
            mode,
        })
    }

    pub fn from_decoded(decoded: &SampleAttributes) -> RBoxErr<Self> {
        Ok(Self::new(
            decoded.loop_start,
            decoded.loop_len,
            SampleAttributeLoopMode::from_value(&decoded.loop_mode)
                .unwrap_or(SampleAttributeLoopMode::Off),
        )?)
    }
}

#[cfg(test)]
mod tests {

    mod test_sample_loop_config {
        mod test_new {

            use crate::samples::configs::{SampleAttributeLoopMode, SampleLoopConfig};

            #[test]
            fn test_new_sample_loop_config_loop_off() {
                assert_eq!(
                    SampleLoopConfig::new(0, 10, SampleAttributeLoopMode::Off).unwrap(),
                    SampleLoopConfig {
                        start: 0,
                        length: 10,
                        mode: SampleAttributeLoopMode::Off
                    }
                );
            }

            #[test]
            fn test_new_sample_loop_config_umin_start_umax_length() {
                assert_eq!(
                    SampleLoopConfig::new(u32::MIN, u32::MAX, SampleAttributeLoopMode::Off)
                        .unwrap(),
                    SampleLoopConfig {
                        start: u32::MIN,
                        length: u32::MAX,
                        mode: SampleAttributeLoopMode::Off
                    }
                );
            }

            #[test]
            fn test_new_sample_loop_config_loop_normal() {
                assert_eq!(
                    SampleLoopConfig::new(0, 10, SampleAttributeLoopMode::Normal).unwrap(),
                    SampleLoopConfig {
                        start: 0,
                        length: 10,
                        mode: SampleAttributeLoopMode::Normal
                    }
                );
            }

            #[test]
            fn test_new_sample_loop_config_loop_pingpong() {
                assert_eq!(
                    SampleLoopConfig::new(0, 10, SampleAttributeLoopMode::PingPong).unwrap(),
                    SampleLoopConfig {
                        start: 0,
                        length: 10,
                        mode: SampleAttributeLoopMode::PingPong
                    }
                );
            }
        }
    }

    mod test_from_decoded {

        use crate::{
            samples::{
                configs::{SampleAttributeLoopMode, SampleLoopConfig},
                slices::Slice,
                SampleAttributes,
            },
            OptionEnumValueConvert,
        };

        #[test]
        fn test_umin_start_umax_len() {
            let decoded = SampleAttributes {
                header: [0_u8; 23],
                tempo: 128000,
                trim_len: 0,
                loop_len: u32::MAX,
                stretch: 0,
                loop_mode: SampleAttributeLoopMode::Off.value().unwrap(),
                gain: 0,
                quantization: 0,
                trim_start: 0,
                trim_end: 0,
                loop_start: u32::MIN,
                slices: [Slice {
                    trim_start: 0,
                    trim_end: 0,
                    loop_start: 0,
                }; 64],
                slices_len: 0,
                checksum: 0,
            };

            assert_eq!(
                SampleLoopConfig::from_decoded(&decoded).unwrap(),
                SampleLoopConfig {
                    start: u32::MIN,
                    length: u32::MAX,
                    mode: SampleAttributeLoopMode::Off
                }
            );
        }

        #[test]
        fn test_loop_off() {
            let decoded = SampleAttributes {
                header: [0_u8; 23],
                tempo: 128000,
                trim_len: 0,
                loop_len: 10,
                stretch: 0,
                loop_mode: SampleAttributeLoopMode::Off.value().unwrap(),
                gain: 0,
                quantization: 0,
                trim_start: 0,
                trim_end: 0,
                loop_start: 0,
                slices: [Slice {
                    trim_start: 0,
                    trim_end: 0,
                    loop_start: 0,
                }; 64],
                slices_len: 0,
                checksum: 0,
            };

            assert_eq!(
                SampleLoopConfig::from_decoded(&decoded).unwrap(),
                SampleLoopConfig {
                    start: 0,
                    length: 10,
                    mode: SampleAttributeLoopMode::Off
                }
            );
        }

        #[test]
        fn test_loop_normal() {
            let decoded = SampleAttributes {
                header: [0_u8; 23],
                tempo: 128000,
                trim_len: 0,
                loop_len: 10,
                stretch: 0,
                loop_mode: SampleAttributeLoopMode::Normal.value().unwrap(),
                gain: 0,
                quantization: 0,
                trim_start: 0,
                trim_end: 0,
                loop_start: 0,
                slices: [Slice {
                    trim_start: 0,
                    trim_end: 0,
                    loop_start: 0,
                }; 64],
                slices_len: 0,
                checksum: 0,
            };

            assert_eq!(
                SampleLoopConfig::from_decoded(&decoded).unwrap(),
                SampleLoopConfig {
                    start: 0,
                    length: 10,
                    mode: SampleAttributeLoopMode::Normal
                }
            );
        }

        #[test]
        fn test_loop_pingpong() {
            let decoded = SampleAttributes {
                header: [0_u8; 23],
                tempo: 128000,
                trim_len: 0,
                loop_len: 10,
                stretch: 0,
                loop_mode: SampleAttributeLoopMode::PingPong.value().unwrap(),
                gain: 0,
                quantization: 0,
                trim_start: 0,
                trim_end: 0,
                loop_start: 0,
                slices: [Slice {
                    trim_start: 0,
                    trim_end: 0,
                    loop_start: 0,
                }; 64],
                slices_len: 0,
                checksum: 0,
            };

            assert_eq!(
                SampleLoopConfig::from_decoded(&decoded).unwrap(),
                SampleLoopConfig {
                    start: 0,
                    length: 10,
                    mode: SampleAttributeLoopMode::PingPong
                }
            );
        }
    }
}
