//! Read/Write Octatrack sample attributes (`.ot`) files.

pub mod configs;
pub mod options;
pub mod slices;

use std::{error::Error, fs::File, io::prelude::*, io::Write, path::PathBuf};

use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

use crate::{
    samples::options::{SampleAttributeTimestrechMode, SampleAttributeTrigQuantizationMode},
    samples::{
        configs::{SampleLoopConfig, SampleTrimConfig},
        slices::{Slice, Slices},
    },
    FromPathBuf, OptionEnumValueConvert, RBoxErr, ToPathBuf,
};

/// Raw header bytes in an Octatrack `.ot` metadata settings file (Header always equates to: `FORM....DPS1SMPA`)
pub const HEADER_BYTES: [u8; 16] = [
    0x46, 0x4F, 0x52, 0x4D, 0x00, 0x00, 0x00, 0x00, 0x44, 0x50, 0x53, 0x31, 0x53, 0x4D, 0x50, 0x41,
];

/// Raw bytes written after the header in an Octatrack `.ot` metadata settings file.
pub const UNKNOWN_BYTES: [u8; 7] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00];

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
    fn swap_bytes(self) -> Result<Self::T, Box<dyn Error>>;
}

#[derive(Debug)]
enum SampleAttributeErrors {
    InvalidTempo,
    InvalidGain,
}
impl std::fmt::Display for SampleAttributeErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidGain => write!(f, "invalid gain value"),
            Self::InvalidTempo => write!(f, "invalid tempo value"),
        }
    }
}
impl std::error::Error for SampleAttributeErrors {}

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
    /// -24.0 db <= x <= +24 db range in the machine's UI,
    /// but 0 <= x <= 96 range when writing binary data file.
    pub gain: u16,

    /// Default trig quantization mode applied to the sample.
    /// See the `ot_sample::options::SampleTrigQuantizationModes` enum for suitable choices.
    pub quantization: u8,

    /// Where the trim start marker is placed for the sample, measured in bars.
    /// Default is 0 (start of sample).
    pub trim_start: u32,

    /// Where the trim end marker is placed for the sample.
    /// When the sample is being played in normal mode (i.e. not using slices),
    /// the Octatrack will not play samples past this point.
    /// By default, trim length should be equal to trim end,
    /// and probably loop length too for drum hit sample chains.
    pub trim_end: u32,

    /// Start position for any loops.
    /// Default should be the same as trim start.
    /// Measured in bars.
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
    fn swap_bytes(self) -> Result<Self::T, Box<dyn Error>> {
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
        let validated_gain: RBoxErr<u16> = if gain > &24.0 {
            Err(SampleAttributeErrors::InvalidGain.into())
        } else if gain < &-24.0 {
            Err(SampleAttributeErrors::InvalidGain.into())
        } else {
            // translate to 0_u16 <= x < 96_u16 from -24.0_d32 <= x <= + 24.0_f32
            // with one decimal place
            let new_gain_f32 = (2.0 * 10.0 * (gain + 24.0)).round() * 0.1;
            Ok(new_gain_f32 as u16)
        };

        let validated_tempo: RBoxErr<f32> = if tempo < &30.0 {
            Err(SampleAttributeErrors::InvalidTempo.into())
        } else if tempo > &300.0 {
            Err(SampleAttributeErrors::InvalidTempo.into())
        } else {
            Ok(*tempo)
        };

        Ok(Self {
            header: FULL_HEADER,
            gain: validated_gain?,
            stretch: stretch.value()?,
            tempo: validated_tempo? as u32,
            quantization: quantization.value()? as u8,
            trim_start: trim_config.start,
            trim_end: trim_config.end,
            trim_len: trim_config.length,
            loop_start: loop_config.start,
            loop_len: loop_config.length,
            loop_mode: loop_config.mode.value()? as u32,
            slices: slices.slices,
            slices_len: slices.count,
            checksum: 0,
        })
    }

    /// Encodes struct data to binary representation, after some pre-processing.
    ///
    /// Beore serializing, will:
    /// 1. modify tempo and gain values to machine ranges
    /// 2. swaps bytes of values (when current system is little-endian)
    /// 3. generate checksum value
    pub fn encode(&self) -> RBoxErr<Vec<u8>> {
        // clone instead of mutable borrwed reference -- don't want to bytes swap or
        // modify current values
        let mut bswapd = self.clone();

        // tempo is multiplied by 24 when written to encoded file
        // reference: Octainer
        bswapd.tempo = self.tempo * 24;

        // gan is normalised to the -24 <= x <= 24 range when written to encoded file
        // reference: Octainer
        bswapd.gain = self.gain + 48;

        // trim length is multiplied by 100 when written to encoded file
        // reference: Octainer
        // bswapd.trim_len = self.trim_len * 100;

        // loop length is multiplied by 100 when written to encoded file
        // reference: Octainer
        // bswapd.loop_len = self.loop_len * 100;

        if cfg!(target_endian = "little") {
            bswapd = bswapd.swap_bytes()?;
        }

        let mut bytes: Vec<u8> = bincode::serialize(&bswapd)?;

        // TODO: I'm only doing this to confirm a struct decoded from file and written
        // straight out is exactly the same as the read file (which it is).
        // so it's not file writes or encoding causing the problem with checksums
        if bswapd.checksum == 0 {
            let mut i: usize = 16;
            let mut checksum: u16 = 0;

            while i < bytes.len() - 2 {
                let incr = bytes[i] as u16;

                // TODO: Was getting overflow errors....
                // if u16::MAX - checksum < incr {break};
                checksum += incr;
                i += 1;
            }
            bswapd.checksum = checksum;
            if cfg!(target_endian = "little") {
                bswapd.checksum = bswapd.checksum.swap_bytes();
            }
            bytes = bincode::serialize(&bswapd)?;
        }

        Ok(bytes)
    }

    /// Decode raw bytes of a `.ot` data file into a new struct,
    /// swap byte values if system is little-endian then do some minor
    /// post-processing to get user friendly settings values.

    fn decode(bytes: &Vec<u8>) -> RBoxErr<Self> {
        let decoded: Self = bincode::deserialize(&bytes[..]).unwrap();
        let mut bswapd = decoded.clone();

        // swapping bytes is one required when running on little-endian systems
        if cfg!(target_endian = "little") {
            bswapd = decoded.swap_bytes().unwrap();
        }

        // tempo is multiplied by 24 when written to encoded file
        // reference: Octainer
        bswapd.tempo = bswapd.tempo / 24;

        // gan is normalised to the -24 <= x <= 24 range when written to encoded file
        // reference: Octainer
        bswapd.gain = bswapd.gain - 48;

        // trim length is multiplied by 100 when written to encoded file
        // reference: Octainer
        // bswapd.trim_len = bswapd.trim_len / 100;

        // loop length is multiplied by 100 when written to encoded file
        // reference: Octainer
        // bswapd.loop_len = bswapd.loop_len / 100;

        Ok(bswapd)
    }
}

impl FromPathBuf for SampleAttributes {
    type T = SampleAttributes;

    /// Crete a new struct by reading a file located at `path`.
    fn from_pathbuf(path: &PathBuf) -> Result<Self::T, Box<dyn Error>> {
        let mut infile = File::open(path)?;
        let mut bytes: Vec<u8> = vec![];
        let _: usize = infile.read_to_end(&mut bytes)?;

        let decoded = Self::decode(&bytes)?;

        Ok(decoded)
    }
}

impl ToPathBuf for SampleAttributes {
    /// Crete a new file at the path from the current struct
    fn to_pathbuf(&self, path: &PathBuf) -> RBoxErr<()> {
        let bytes: Vec<u8> = self.encode()?;
        let mut file: File = File::create(path)?;
        let _: RBoxErr<()> = file.write_all(&bytes).map_err(|e| e.into());

        Ok(())
    }
}

/// Used with the `octatools inspect bytes bank` command.
/// Only really useful for debugging and / or reverse engineering purposes.
#[derive(Debug, Serialize, Deserialize)]
pub struct SampleAttributesRawBytes {
    #[serde(with = "BigArray")]
    pub data: [u8; 816],
}

impl FromPathBuf for SampleAttributesRawBytes {
    type T = SampleAttributesRawBytes;

    /// Crete a new struct by reading a file located at `path`.
    fn from_pathbuf(path: &PathBuf) -> Result<Self::T, Box<dyn Error>> {
        let mut infile = File::open(path)?;
        let mut bytes: Vec<u8> = vec![];
        let _: usize = infile.read_to_end(&mut bytes)?;

        let new: Self = bincode::deserialize(&bytes[..])?;

        Ok(new)
    }
}

impl ToPathBuf for SampleAttributesRawBytes {
    fn to_pathbuf(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let bytes: Vec<u8> = bincode::serialize(&self)?;
        let mut file: File = File::create(path)?;
        let _: RBoxErr<()> = file.write_all(&bytes).map_err(|e| e.into());

        Ok(())
    }
}
