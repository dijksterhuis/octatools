//! Read/Write Octatrack sample attributes (`.ot`) files.

pub mod configs;
pub mod options;
pub mod slices;

use crate::{
    samples::options::{SampleAttributeTimestrechMode, SampleAttributeTrigQuantizationMode},
    samples::{
        configs::{SampleLoopConfig, SampleTrimConfig},
        slices::{Slice, Slices},
    },
    Decode, Encode, OptionEnumValueConvert, RBoxErr,
};
use octatools_derive::Decodeable;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

// in `hexdump -C` format:
// ```
// FORM....DPS1SMPA
// ......
// ```
/// Raw header bytes and post-header spacer bytes in an Octatrack `.ot` metadata settings file
pub const FULL_HEADER: [u8; 23] = [
    0x46, 0x4F, 0x52, 0x4D, 0x00, 0x00, 0x00, 0x00, 0x44, 0x50, 0x53, 0x31, 0x53, 0x4D, 0x50, 0x41,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00,
];

/// Trait for adding the `.swap_bytes()` method.
pub trait SwapBytes {
    /// Type for `Self`
    type T;

    /// Swap the bytes of all struct fields.
    /// Must be applied to the `SampleAttributes` file to deal with litle-endian/big-endian systems.
    fn swap_bytes(self) -> RBoxErr<Self::T>;
}

#[derive(Debug)]
enum SampleAttributeErrors {
    TempoOutOfBounds,
    GainOutOfBounds,
}
impl std::fmt::Display for SampleAttributeErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::GainOutOfBounds => write!(f, "invalid gain, must be in range -24.0 <= x <= 24.0"),
            Self::TempoOutOfBounds => write!(
                f,
                "invalid tempo value, must be in range 30.0 <= x <= 300.0"
            ),
        }
    }
}
impl std::error::Error for SampleAttributeErrors {}

/// Convert from a human (device UI) representation of gain (-24.0 <= x <= 24.0)
/// to binary data file representation of gain (0 <= x <= 96).
/// Note that binary gain uses two bytes (hence u16).
fn bin_gain_from_human(human_gain: &f32) -> RBoxErr<u16> {
    // don't use contains range trick here. need to explicitly check range bounds.
    #[allow(clippy::manual_range_contains)]
    if human_gain < &-24.0 || human_gain > &24.0 {
        Err(SampleAttributeErrors::GainOutOfBounds.into())
    } else {
        let new_gain_f32 = (2.0 * (10.0 * (human_gain + 24.0)).round()) * 0.1;
        Ok(new_gain_f32 as u16)
    }
}

/// Convert from a human (device UI) representation of tempo (30.0 <= x <= 300.0)
/// to binary data file representation of gain (720 <= x <= 7200)
/// Note that binary tempo uses four bytes (hence u32).
fn bin_tempo_from_human(human_tempo: &f32) -> RBoxErr<u32> {
    // don't use contains range trick here. need to explicitly check range bounds.
    #[allow(clippy::manual_range_contains)]
    if human_tempo < &30.0 || human_tempo > &300.0 {
        Err(SampleAttributeErrors::TempoOutOfBounds.into())
    } else {
        let bin_tempo_f32 = human_tempo * 24.0;
        Ok(bin_tempo_f32 as u32)
    }
}

/// The checksum value on sample attributes files is two bytes and is just the
/// sum of all non-checksum u8 bytes.
fn calculate_checksum_sample_attr_bytes(bytes: &[u8]) -> RBoxErr<u16> {
    // should always be 832 bytes
    let checksum_bytes = &bytes[16..bytes.len() - 2];

    /*
    the checksum value for the samples attributes file is the sum of all
    bytes, excluding header and checksum bytes, stored as a u16 (two last bytes
    in the binary file).

    however, the checksum can overflow beyond u16. in that case, the checksum
    is `x modulo U16::MAX` (wrap around on the type).
    */

    let chk: u32 = checksum_bytes
        .iter()
        .map(|x| *x as u32)
        .sum::<u32>()
        .rem_euclid(u16::MAX as u32 + 1);

    Ok(chk as u16)
}

/// Struct to create a valid Octatrack `.ot` file.
/// General metadata for the sample's configuration on the OT
/// and the slice array with pointer positions for the sliced WAV.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct SampleAttributes {
    /// Header
    pub header: [u8; 23],

    /// Tempo is always the machine UI's BPM multiplied by 24
    pub tempo: u32,

    /// Number of bars for the sample trim length marker.
    /// By default, trim length should be equal to trim end,
    /// and probably loop length too for drum hit sample chains.
    pub trim_len: u32,

    /// Number of bars for the sample loop length marker.
    /// By default, loop length should be equal to trim length for sample chains.
    pub loop_len: u32,

    /// Default timestrech algorithm applied to the sample.
    /// See the `ot_sample::options::SampleTimestrechModes` enum for suitable choices.
    pub stretch: u32,

    /// Default loop mode applied to the sample.
    /// See the `ot_sample::options::SampleLoopModes` enum for suitable choices.
    pub loop_mode: u32,

    /// Gain of the sample.
    /// -24.0 db <= x <= +24 db range in the machine's UI, with increments of 0.5 db changes.
    /// 0 <= x <= 96 range in binary data file.
    pub gain: u16,

    /// Default trig quantization mode applied to the sample.
    /// See the `ot_sample::options::SampleTrigQuantizationModes` enum for suitable choices.
    pub quantization: u8,

    /// Where the trim start marker is placed for the sample, measured in bars.
    /// Default is 0 (start of sample).
    /// TODO: measured in bars? or samples?
    pub trim_start: u32,

    /// Where the trim end marker is placed for the sample.
    /// When the sample is being played in normal mode (i.e. not using slices),
    /// the Octatrack will not play samples past this point.
    /// By default, trim length should be equal to trim end,
    /// and probably loop length too for drum hit sample chains.
    /// TODO: measured in bars? or samples?
    pub trim_end: u32,

    /// Start position for any loops. Default should be the same as trim start.
    /// Measured in bars.
    /// A note from the Octatrack manual on loop point/start behaviour:
    /// > If a loop point is set, the sample will play from the start point to the
    /// > end point, then loop from the loop point to the end point
    pub loop_start: u32,

    /// 64 length array containing `Slice`s.
    /// See the `Slice` struct for more details.
    /// Any empty slice positions should have zero-valued struct fields.
    #[serde(with = "BigArray")]
    pub slices: [Slice; 64],

    /// Number of usuable `Slice`s in this sample.
    /// Used by the Octatrack to ignore zero-valued `Slice`s in the `slices` array when loading the sample.
    pub slices_len: u32,

    /// Checksum value for the struct.
    /// This must be calculated **after** the struct is created on little-endian systems
    /// (requires byte swapping all struct fields to get the correct checksum value).
    pub checksum: u16,
}

impl SwapBytes for SampleAttributes {
    /// Swaps the bytes on all struct fields.
    /// **MUST BE CALLED BEFORE SERIALISATION WHEN SYSTEM IS LITTLE ENDIAN!**
    type T = SampleAttributes;

    /// Swap the bytes of all struct fields.
    /// Must be applied to the `SampleAttributes` file to deal with litle-endian/big-endian systems.
    fn swap_bytes(self) -> RBoxErr<Self::T> {
        let mut bswapped_slices: [Slice; 64] = self.slices;

        for (i, slice) in self.slices.iter().enumerate() {
            bswapped_slices[i] = slice.swap_bytes().unwrap();
        }

        let bswapped = Self {
            header: FULL_HEADER,
            tempo: self.tempo.swap_bytes(),
            trim_len: self.trim_len.swap_bytes(),
            loop_len: self.loop_len.swap_bytes(),
            stretch: self.stretch.swap_bytes(),
            loop_mode: self.loop_mode.swap_bytes(),
            gain: self.gain.swap_bytes(),
            quantization: self.quantization.swap_bytes(),
            trim_start: self.trim_start.swap_bytes(),
            trim_end: self.trim_end.swap_bytes(),
            loop_start: self.loop_start.swap_bytes(),
            slices: bswapped_slices,
            slices_len: self.slices_len.swap_bytes(),
            checksum: self.checksum.swap_bytes(),
        };

        Ok(bswapped)
    }
}

impl SampleAttributes {
    pub fn new(
        tempo: &f32,
        stretch: &SampleAttributeTimestrechMode,
        quantization: &SampleAttributeTrigQuantizationMode,
        gain: &f32,
        trim_config: &SampleTrimConfig,
        loop_config: &SampleLoopConfig,
        slices: &Slices,
    ) -> RBoxErr<Self> {
        Ok(Self {
            header: FULL_HEADER,
            gain: bin_gain_from_human(gain)?,
            stretch: stretch.value()?,
            tempo: bin_tempo_from_human(tempo)?,
            quantization: quantization.value()? as u8,
            trim_start: trim_config.start,
            trim_end: trim_config.end,
            trim_len: trim_config.length, // bin lengths are multiplied by 100?
            loop_start: loop_config.start,
            loop_len: loop_config.length, // bin lengths are multiplied by 100?
            loop_mode: loop_config.mode.value()?,
            slices: slices.slices,
            slices_len: slices.count,
            checksum: 0,
        })
    }
}

// For samples, need to run special decode method as need to flip bytes depending on endianness
impl Decode for SampleAttributes {
    /// Decode raw bytes of a `.ot` data file into a new struct,
    /// swap byte values if system is little-endian then do some minor
    /// post-processing to get user-friendly settings values.
    fn decode(bytes: &[u8]) -> RBoxErr<Self> {
        let decoded: Self = bincode::deserialize(bytes)?;
        let mut bswapd = decoded.clone();

        // swapping bytes is one required when running on little-endian systems
        if cfg!(target_endian = "little") {
            bswapd = decoded.swap_bytes()?;
        }

        Ok(bswapd)
    }
}

// For sample data, need swap bytes depending on endianness and calculate a checksum
impl Encode for SampleAttributes {
    /// Encodes struct data to binary representation, after some pre-processing.
    ///
    /// Before serializing, will:
    /// 1. modify tempo and gain values to machine ranges
    /// 2. swaps bytes of values (when current system is little-endian)
    /// 3. generate checksum value
    fn encode(&self) -> RBoxErr<Vec<u8>> {
        let mut bswapd = self.clone();

        if cfg!(target_endian = "little") {
            bswapd = bswapd.swap_bytes()?;
        }

        let mut bytes: Vec<u8> = bincode::serialize(&bswapd)?;

        // no checksum created yet
        if bswapd.checksum == 0 {
            bswapd.checksum = calculate_checksum_sample_attr_bytes(&bytes)?;
            if cfg!(target_endian = "little") {
                bswapd.checksum = bswapd.checksum.swap_bytes();
            }
            bytes = bincode::serialize(&bswapd)?;
        }

        Ok(bytes)
    }
}

/// Used with the `octatools inspect bytes bank` command.
/// Only really useful for debugging and / or reverse engineering purposes.
#[derive(Debug, Serialize, Deserialize, Decodeable)]
pub struct SampleAttributesRawBytes {
    #[serde(with = "BigArray")]
    pub data: [u8; 832],
}

#[cfg(test)]
mod test {
    mod checksum {

        mod array_based {
            use crate::samples::calculate_checksum_sample_attr_bytes;
            use std::array::from_fn;

            #[test]
            fn ok_no_overflow_all_255() {
                let bytes: [u8; 832] = from_fn(|_| 255);
                assert_eq!(
                    calculate_checksum_sample_attr_bytes(&bytes).unwrap(),
                    10962,
                    "overflows should wrap"
                );
            }
            #[test]
            fn ok_no_overflow() {
                let bytes: [u8; 832] = from_fn(|_| 78);

                assert_eq!(
                    calculate_checksum_sample_attr_bytes(&bytes).unwrap(),
                    63492,
                    "random value that shouldn't overflow at all"
                );
            }

            #[test]
            fn ok_64_slice_len() {
                let bytes: [u8; 832] = from_fn(|_| 64);
                assert_eq!(calculate_checksum_sample_attr_bytes(&bytes).unwrap(), 52096,);
            }

            #[test]
            fn ok_one_bytes() {
                let bytes: [u8; 832] = from_fn(|_| 1);
                assert_eq!(
                    calculate_checksum_sample_attr_bytes(&bytes).unwrap(),
                    814,
                    "low values definitely don't overflow"
                );
            }

            #[test]
            fn ok_zeroed() {
                let bytes: [u8; 832] = from_fn(|_| 0);
                assert_eq!(
                    calculate_checksum_sample_attr_bytes(&bytes).unwrap(),
                    0,
                    "unknown if this is correct behaviour"
                );
            }
        }

        mod file_based {
            use crate::read_type_from_bin_file;
            use crate::samples::{calculate_checksum_sample_attr_bytes, SampleAttributes};
            use std::path::PathBuf;

            #[test]
            fn zero_slices_trig_quant_one_step() {
                let path = PathBuf::from("../data/tests/samples/checksum/0slices-tq1step.ot");
                let data = read_type_from_bin_file::<SampleAttributes>(&path).unwrap();
                let bytes: Vec<u8> = bincode::serialize(&data).unwrap();
                assert_eq!(calculate_checksum_sample_attr_bytes(&bytes).unwrap(), 743);
            }

            #[test]
            fn zero_slices() {
                let path = PathBuf::from("../data/tests/samples/checksum/0slices.ot");
                let data = read_type_from_bin_file::<SampleAttributes>(&path).unwrap();
                let bytes: Vec<u8> = bincode::serialize(&data).unwrap();
                assert_eq!(calculate_checksum_sample_attr_bytes(&bytes).unwrap(), 997);
            }

            #[test]
            fn one_slice() {
                let path = PathBuf::from("../data/tests/samples/checksum/1slices.ot");
                let data = read_type_from_bin_file::<SampleAttributes>(&path).unwrap();
                let bytes: Vec<u8> = bincode::serialize(&data).unwrap();
                assert_eq!(calculate_checksum_sample_attr_bytes(&bytes).unwrap(), 2082);
            }
            #[test]
            fn two_slices() {
                let path = PathBuf::from("../data/tests/samples/checksum/2slices.ot");
                let data = read_type_from_bin_file::<SampleAttributes>(&path).unwrap();
                let bytes: Vec<u8> = bincode::serialize(&data).unwrap();
                assert_eq!(calculate_checksum_sample_attr_bytes(&bytes).unwrap(), 3673);
            }
            #[test]
            fn four_slices() {
                let path = PathBuf::from("../data/tests/samples/checksum/4slices.ot");
                let data = read_type_from_bin_file::<SampleAttributes>(&path).unwrap();
                let bytes: Vec<u8> = bincode::serialize(&data).unwrap();
                assert_eq!(calculate_checksum_sample_attr_bytes(&bytes).unwrap(), 7235);
            }
            #[test]
            fn eight_slices() {
                let path = PathBuf::from("../data/tests/samples/checksum/8slices.ot");
                let data = read_type_from_bin_file::<SampleAttributes>(&path).unwrap();
                let bytes: Vec<u8> = bincode::serialize(&data).unwrap();
                assert_eq!(calculate_checksum_sample_attr_bytes(&bytes).unwrap(), 13943);
            }
            #[test]
            fn sixteen_slices() {
                let path = PathBuf::from("../data/tests/samples/checksum/16slices.ot");
                let data = read_type_from_bin_file::<SampleAttributes>(&path).unwrap();
                let bytes: Vec<u8> = bincode::serialize(&data).unwrap();
                assert_eq!(calculate_checksum_sample_attr_bytes(&bytes).unwrap(), 26111);
            }
            #[test]
            fn thirty_two_slices() {
                let path = PathBuf::from("../data/tests/samples/checksum/32slices.ot");
                let data = read_type_from_bin_file::<SampleAttributes>(&path).unwrap();
                let bytes: Vec<u8> = bincode::serialize(&data).unwrap();
                assert_eq!(calculate_checksum_sample_attr_bytes(&bytes).unwrap(), 50405);
            }
            #[test]
            fn forty_eight_slices() {
                let path = PathBuf::from("../data/tests/samples/checksum/48slices.ot");
                let data = read_type_from_bin_file::<SampleAttributes>(&path).unwrap();
                let bytes: Vec<u8> = bincode::serialize(&data).unwrap();
                assert_eq!(calculate_checksum_sample_attr_bytes(&bytes).unwrap(), 10787);
            }
            #[test]
            fn sixty_two_slices() {
                let path = PathBuf::from("../data/tests/samples/checksum/62slices.ot");
                let data = read_type_from_bin_file::<SampleAttributes>(&path).unwrap();
                let bytes: Vec<u8> = bincode::serialize(&data).unwrap();
                assert_eq!(calculate_checksum_sample_attr_bytes(&bytes).unwrap(), 35738);
            }
            #[test]
            fn sixty_three_slices() {
                let path = PathBuf::from("../data/tests/samples/checksum/63slices.ot");
                let data = read_type_from_bin_file::<SampleAttributes>(&path).unwrap();
                let bytes: Vec<u8> = bincode::serialize(&data).unwrap();
                assert_eq!(calculate_checksum_sample_attr_bytes(&bytes).unwrap(), 35086);
            }
            #[test]
            fn sixty_four_slices_correct() {
                let path = PathBuf::from("../data/tests/samples/checksum/64slices.ot");
                let data = read_type_from_bin_file::<SampleAttributes>(&path).unwrap();
                let bytes: Vec<u8> = bincode::serialize(&data).unwrap();
                assert_eq!(calculate_checksum_sample_attr_bytes(&bytes).unwrap(), 34295,);
            }
        }
    }

    mod bin_tempo_from_human {
        use crate::samples::bin_tempo_from_human;

        #[test]
        fn err_oob_too_low() {
            assert!(bin_tempo_from_human(&29.9).is_err());
        }
        #[test]
        fn err_oob_too_high() {
            assert!(bin_tempo_from_human(&300.1).is_err());
        }

        #[test]
        fn ok_30() {
            assert_eq!(bin_tempo_from_human(&30.0).unwrap(), 720);
        }
        #[test]
        fn ok_300() {
            assert_eq!(bin_tempo_from_human(&300.0).unwrap(), 7200);
        }

        #[test]
        fn ok_120() {
            assert_eq!(bin_tempo_from_human(&120.0).unwrap(), 2880);
        }

        #[test]
        fn ok_100() {
            assert_eq!(bin_tempo_from_human(&100.0).unwrap(), 2400);
        }
    }

    mod bin_gain_from_human {
        use crate::samples::bin_gain_from_human;

        #[test]
        fn err_oob_too_low() {
            assert!(bin_gain_from_human(&-24.1).is_err());
        }
        #[test]
        fn err_oob_too_high() {
            assert!(bin_gain_from_human(&24.1).is_err());
        }

        #[test]
        fn ok_negative_24() {
            assert_eq!(bin_gain_from_human(&-24.0).unwrap(), 0);
        }
        #[test]
        fn ok_positive_24() {
            assert_eq!(bin_gain_from_human(&24.0).unwrap(), 96);
        }

        #[test]
        fn ok_positive_23_half() {
            assert_eq!(bin_gain_from_human(&23.5).unwrap(), 95);
        }

        #[test]
        fn ok_negative_23_half() {
            assert_eq!(bin_gain_from_human(&-23.5).unwrap(), 1);
        }

        #[test]
        fn ok_zero() {
            assert_eq!(bin_gain_from_human(&0.0).unwrap(), 48);
        }

        #[test]
        fn ok_zero_half() {
            assert_eq!(bin_gain_from_human(&0.5).unwrap(), 49);
        }

        #[test]
        fn ok_zero_minus_half() {
            assert_eq!(bin_gain_from_human(&-0.5).unwrap(), 47);
        }
    }

    mod create_new {

        use crate::samples::options::{
            SampleAttributeLoopMode, SampleAttributeTimestrechMode,
            SampleAttributeTrigQuantizationMode,
        };
        use crate::samples::{SampleAttributes, SampleLoopConfig, SampleTrimConfig, Slice, Slices};
        fn create_mock_configs_blank() -> (SampleTrimConfig, SampleLoopConfig, Slices) {
            let trim_config = SampleTrimConfig {
                start: 0,
                end: 0,
                length: 0,
            };

            let loop_config = SampleLoopConfig {
                start: 0,
                length: 0,
                mode: SampleAttributeLoopMode::Normal,
            };

            let default_slice = Slice {
                trim_start: 0,
                trim_end: 0,
                loop_start: 0xFFFFFFFF,
            };

            let slices: [Slice; 64] = [default_slice; 64];

            let slice_conf = Slices { slices, count: 0 };

            (trim_config, loop_config, slice_conf)
        }

        #[test]
        fn err_oob_tempo() {
            let (trim_conf, loop_conf, slices) = create_mock_configs_blank();

            let composed_chain = SampleAttributes::new(
                &10000.0,
                &SampleAttributeTimestrechMode::Off,
                &SampleAttributeTrigQuantizationMode::PatternLength,
                &0.0,
                &trim_conf,
                &loop_conf,
                &slices,
            );

            assert!(composed_chain.is_err());
        }

        #[test]
        fn err_invalid_gain() {
            let (trim_conf, loop_conf, slices) = create_mock_configs_blank();

            let composed_chain = SampleAttributes::new(
                &125.0,
                &SampleAttributeTimestrechMode::Off,
                &SampleAttributeTrigQuantizationMode::PatternLength,
                &300.0,
                &trim_conf,
                &loop_conf,
                &slices,
            );

            assert!(composed_chain.is_err());
        }
    }
}
