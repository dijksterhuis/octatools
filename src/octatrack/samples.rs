//! Read/Write Octatrack sample attributes (`.ot`) files.

pub mod configs;
pub mod slices;

use std::{
    fs::File,
    io::prelude::*,
    path::PathBuf,
};

use bincode::ErrorKind;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

use crate::results::*;
use crate::audio::wavfile::WavFile;
use crate::constants::DEFAULT_SAMPLE_RATE;

use crate::octatrack::common::OptionEnumValueConvert;
use crate::octatrack::options::{
    SampleAttributeTimestrechMode,
    SampleAttributeTrigQuantizationMode,
};

use crate::octatrack::samples::{
    configs::{
        SampleLoopConfig,
        SampleTrimConfig,
    },
    slices::{Slice, Slices},
};


/// Raw header bytes in an Octatrack `.ot` metadata settings file (Header always equates to: `FORM....DPS1SMPA`)
pub const HEADER_BYTES: [u8; 16] = [0x46,0x4F,0x52,0x4D,0x00,0x00,0x00,0x00,0x44,0x50,0x53,0x31,0x53,0x4D,0x50,0x41];

/// Raw bytes written after the header in an Octatrack `.ot` metadata settings file.
pub const UNKNOWN_BYTES: [u8; 7] = [0x00,0x00,0x00,0x00,0x00,0x02,0x00];

// in `hexdump -C` format:
// ```
// FORM....DPS1SMPA
// ......
// ```
/// Raw header bytes and post-header spacer bytes in an Octatrack `.ot` metadata settings file

pub const FULL_HEADER: [u8; 23] = [
    0x46,
    0x4F,
    0x52,
    0x4D,
    0x00,
    0x00,
    0x00,
    0x00,
    0x44,
    0x50,
    0x53,
    0x31,
    0x53,
    0x4D,
    0x50,
    0x41,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x02,
    0x00
];



// TODO: Change to taking number of samples as argument
// so we can pass in a single wav or a vvector worth of wavs
// otherwise we have to mess around with switching about types

// TODO: Move to octatrack_common?

/// Calculate the effective number of bars for a sample / slice.
/// Assumes four beats per bar. 

pub fn get_otsample_nbars_from_wavfiles(wavs: &Vec<WavFile>, tempo_bpm: &f32) -> U32Result {
    let total_samples: u32 = wavs.iter().map(|x| x.len as u32).sum();
    let beats = total_samples as f32 / (DEFAULT_SAMPLE_RATE as f32 * 60.0 * 4.0);
    let mut bars = ((tempo_bpm * 4.0 * beats) + 0.5) * 0.25;
    bars -= bars % 0.25;
    Ok((bars * 100.0) as u32)
}

/// Each 'sample' can have two files present on an Octatrack: 
/// the audio file and the corresponding `.ot` attributes file.
/// This struct represents one 'sample' as a combination of those two file paths.

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
    
    pub fn from_pathbufs(audio_fp: &PathBuf, ot_fp: &Option<PathBuf>) -> Result<Self, ()> {

        Ok(
            Self {
                name: audio_fp.file_stem().unwrap().to_str().unwrap().to_string(),
                audio_filepath: audio_fp.clone(),
                attributes_filepath: ot_fp.clone()
            }
        )
    }

    /// Create a new `OctatrackSampleFile` only from  the audio file path 

    pub fn from_audio_pathbuf(audio_fp: &PathBuf) -> Result<Self, ()> {

        // TODO: optimise this? so many clones
        let mut ot_file_path = audio_fp.clone();
        ot_file_path.set_extension("ot");

        let mut ot_file_pathbuf = Some(ot_file_path.clone());
        if ! ot_file_path.exists() {ot_file_pathbuf = None};

        Ok(
            Self {
                name: audio_fp.file_stem().unwrap().to_str().unwrap().to_string(),
                audio_filepath: audio_fp.clone(),
                attributes_filepath: ot_file_pathbuf
            }
        )
    }
}


/// Struct to create a valid Octatrack `.ot` file. 
/// General metadata for the sample's configuration on the OT 
/// and the slice array with pointer positions for the sliced WAV.

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct SampleAttributes {

    /// Header is always: `FORM....DPS1SMPA`
    pub header: [u8; 16],

    /// Blank values then a single 2 value: `......` (dunno why the 2 value)
    pub blank: [u8; 7],

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

impl SampleAttributes {
    pub fn new(
        tempo: &f32,
        stretch: &SampleAttributeTimestrechMode,
        quantization: &SampleAttributeTrigQuantizationMode,
        gain: &f32,
        trim_config: &SampleTrimConfig,
        loop_config: &SampleLoopConfig,
        slices: &Slices,
    ) -> Result<Self, ()> {

        println!("GAIN CHANGES: {:#?}", gain);

        if *gain > 24.0 {
            println!(">24");
            return Err(())
        }

        if *gain < -24.0 {
            println!("<24");
            return Err(())
        }

        // translate to 0_u16 <= x < 96_u16 from -24.0_d32 <= x <= + 24.0_f32 
        // with one decimal place
        let gain_u16 = (((2.0 * 10.0 * (gain + 24.0)).round()) / 10.0) as u16;

        println!("GAIN CHANGES: {:#?} {:#?}", gain, gain_u16);

        if *tempo > 300.0 { return Err(())};

        // validate that we've got acceptable options
        let loop_res = loop_config.mode.value();
        let stretch_res = stretch.value();
        let quantise_res = quantization.value();

        if loop_res.is_err() {return Err(loop_res.err().unwrap())} 
        if stretch_res.is_err() {return Err(stretch_res.err().unwrap())} 
        if quantise_res.is_err() {return Err(quantise_res.err().unwrap())} 

        Ok(
            Self {
                header: HEADER_BYTES,
                blank: UNKNOWN_BYTES,
                gain: gain_u16,
                stretch: stretch_res.unwrap(),
                tempo: *tempo as u32,
                quantization: quantise_res.unwrap() as u8,
                trim_start: trim_config.start,
                trim_end: trim_config.end,
                trim_len: trim_config.length,
                loop_start: loop_config.start,
                loop_len: loop_config.length,
                loop_mode: loop_res.unwrap() as u32,
                slices: slices.slices,
                slices_len: slices.count,
                checksum: 0,
            }
        )
    }

    /// Swaps the bytes on all struct fields.
    /// **MUST BE CALLED BEFORE SERIALISATION WHEN SYSTEM IS LITTLE ENDIAN!**
    
    pub fn to_bswapped(&mut self) -> Self {

        let mut bswapped_slices: [Slice; 64] = self.slices.clone();

        for (i, slice) in self.slices.iter().enumerate() {
            bswapped_slices[i] = slice.as_bswapped();
        }

        let bswapped = Self {
            header: HEADER_BYTES,
            blank: UNKNOWN_BYTES,
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

        bswapped
        
    }

    /// Encodes struct data to binary representation, after some pre-processing.
    ///
    /// Swaps byte (when little-endian system), generates the checksum, 
    /// then encodes to binary representation. 
    
    pub fn encode(&self) -> VecU8Result {

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
            bswapd = bswapd.to_bswapped();
        }

        let mut bytes: Vec<u8> = bincode::serialize(&bswapd).unwrap();

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
            bytes = bincode::serialize(&bswapd).unwrap();
        }

        Ok(bytes)

    }

    /// Write encoded struct data to file
    /// Swaps byte (when little-endian system), generates a checksum, 
    /// then encodes to binary representation which can be written to file. 
    
    pub fn to_file(&self, path: &PathBuf) -> VoidResultBoxed {

        use std::fs::File;
        use std::io::Write;

        let bytes: Vec<u8> = self.encode().unwrap();

        let mut file = File::create(path).unwrap();
        let res: VoidResultBoxed = file
            .write_all(&bytes)
            .map_err(|e| e.into())
        ;

        res
    }

    /// Decode raw bytes of a `.ot` data file into a new struct, 
    /// swap byte values if system is little-endian then do some minor 
    /// post-processing to get user friendly settings values.
    
    fn decode(bytes: &Vec<u8>) -> Result<Self, Box<ErrorKind>> {

        let mut decoded: Self = bincode::deserialize(&bytes[..]).unwrap();
        // NOTE: bswapping is one required when running on little-endian systems

        let mut bswapd = decoded.clone();

        if cfg!(target_endian = "little") {
            bswapd = decoded.to_bswapped();
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

    /// Read an `.ot` file into a new struct.
    // TODO: `path` should be a `PathBuf`

    pub fn from_file(path: &str) -> Result<Self, Box<ErrorKind>> {

        let mut infile = File::open(path)?;
        let mut bytes: Vec<u8> = vec![];
        let _: usize = infile.read_to_end(&mut bytes)?;

        let decoded = Self::decode(&bytes).unwrap();

        Ok(decoded)
    }
}


#[cfg(test)]
mod test_unit_sample_chain {
    use super::*;

    mod from_wav {
        use super::*;

        fn mock_create_wav(length: u32) -> Vec<u8> {
            let mut wav: Vec<u8> = Vec::new();
            for _ in 0..length {wav.push(0_u8)}

            wav
        }

        #[ignore]
        #[test]
        fn test_create_new_sample_chain() {

            let wav = mock_create_wav(600000);

            let mut slices: [Slice; 64];

            // get sample length (array length)
            // get number of bars
            // set defaults for stuff like gain
            // create dummy slice array
            todo!()
        }    
    }

    mod from_file {

        #[ignore]
        #[test]
        fn test_do_thing() {
            todo!()
        }    
    }

}


/// Use input files from `resouces/test-data/` to create an OT file output
/// and compare it to what should exist.
/// Read relevant WAV files, create an OT file of some description, write
/// the OT file then compare it to the known good output from OctaChainer. 

#[cfg(test)]
mod test_integration {

    mod test_integration_sample_chain_create_vs_read {

        use std::error::Error;
        use std::path::PathBuf;
        use walkdir::{DirEntry, WalkDir};

        use crate::audio::wavfile::WavFile;

        use crate::octatrack::options::{
            SampleAttributeTimestrechMode,
            SampleAttributeTrigQuantizationMode,
            SampleAttributeLoopMode,
        };

        use crate::octatrack::samples::{
            configs::{
                SampleLoopConfig,
                SampleTrimConfig,    
            },
            slices::{
                Slice,
                Slices,
            },
            SampleAttributes,
            get_otsample_nbars_from_wavfiles
        };
        
        fn walkdir_filter_is_wav(entry: &DirEntry) -> bool {
            entry.file_name()
                    .to_str()
                    .map(|s| s.ends_with(".wav"))
                    .unwrap_or(false)
        }

        fn get_test_wav_paths(path: &str) -> Result<Vec<PathBuf>, Box<dyn Error>> {

            let paths_iter: _ = WalkDir::new(path)
                .sort_by_file_name()
                .max_depth(1)
                .min_depth(1)
                .into_iter()
                .filter_entry(|e| walkdir_filter_is_wav(e))
            ;

            let mut fpaths: Vec<PathBuf> = Vec::new();            
            for entry in paths_iter {
                let unwrapped = entry.unwrap();
                let fpath= unwrapped.path().to_path_buf();
                fpaths.push(fpath);
            }
            
            Ok(fpaths)
        }

        fn create_sample_chain_encoded_from_wavfiles(wav_fps: Vec<PathBuf>) -> Result<(SampleLoopConfig, SampleTrimConfig, Slices), Box<dyn Error>> {

            let mut wavs: Vec<WavFile> = Vec::new();
            for fp in wav_fps {
                let wav = WavFile::from_file(fp).unwrap();
                wavs.push(wav);
            }

            let slices_config = Slices::from_wavfiles(&wavs, &0).unwrap();

            let bars = get_otsample_nbars_from_wavfiles(&wavs, &125.0).unwrap();

            let trim_config = SampleTrimConfig {
                start: 0,
                end: wavs.iter().map(|x| x.len as u32).sum(),
                length: bars,
            };

            let loop_config = SampleLoopConfig {
                start: 0,
                length: bars,
                mode: SampleAttributeLoopMode::Off,
            };

            Ok((loop_config, trim_config, slices_config))

        }

        fn read_valid_sample_chain(path: &str) -> Result<SampleAttributes, Box<dyn Error>> {
            let read_chain = SampleAttributes
                ::from_file(path)
                .unwrap()
                ;
            Ok(read_chain)
        }

        #[test]
        fn test_default_10_samples() {

            let wav_fps = get_test_wav_paths("data/tests/1/wavs/").unwrap();
            let (loop_config, trim_config, slices) = create_sample_chain_encoded_from_wavfiles(wav_fps).unwrap();

            let composed_chain_res = SampleAttributes
                ::new(
                    &125.0, 
                    &SampleAttributeTimestrechMode::Off, 
                    &SampleAttributeTrigQuantizationMode::PatternLength, 
                    &-24.0, 
                    &trim_config, 
                    &loop_config, 
                    &slices, 
                )
            ;

            let composed_chain = &composed_chain_res.clone().unwrap();

            if composed_chain_res.is_err() {
                println!("ERROR IN TEST: {:#?}:", &composed_chain_res.err());
                assert!(false);
            }

            let valid_ot_fp = "data/tests/1/chain.ot";
            let valid_sample_chain = read_valid_sample_chain(&valid_ot_fp).unwrap();

            assert_eq!(
                composed_chain.encode().unwrap(), 
                valid_sample_chain.encode().unwrap(),
            );

        }

        #[test]
        fn test_default_3_samples() {

            let wav_fps = get_test_wav_paths("data/tests/2/wavs/").unwrap();
            let (loop_config, trim_config, slices) = create_sample_chain_encoded_from_wavfiles(wav_fps).unwrap();

            let composed_chain = SampleAttributes
                ::new(
                    &125.0, 
                    &SampleAttributeTimestrechMode::Off, 
                    &SampleAttributeTrigQuantizationMode::PatternLength, 
                    &-24.0, 
                    &trim_config, 
                    &loop_config, 
                    &slices, 
                )
                .unwrap()
            ;

            let valid_ot_fp = "data/tests/2/chain.ot";
            let valid_sample_chain = read_valid_sample_chain(&valid_ot_fp).unwrap();

            assert_eq!(
                composed_chain.encode().unwrap(), 
                valid_sample_chain.encode().unwrap(),
            );

        }

        #[ignore]
        #[test]
        fn test_default_64_samples() {

            let wav_fps = get_test_wav_paths("data/tests/3/wavs/").unwrap();
            let (loop_config, trim_config, slices) = create_sample_chain_encoded_from_wavfiles(wav_fps).unwrap();

            let composed_chain = SampleAttributes
                ::new(
                    &175.0,
                    &SampleAttributeTimestrechMode::Off, 
                    &SampleAttributeTrigQuantizationMode::PatternLength, 
                    &24.0,
                    &trim_config,
                    &loop_config,
                    &slices, 
                )
                .unwrap()
            ;

            let valid_ot_fp = "data/tests/3/chain.ot";
            let valid_sample_chain = read_valid_sample_chain(&valid_ot_fp).unwrap();

            assert_eq!(
                composed_chain, 
                valid_sample_chain,
            );

            assert_eq!(
                composed_chain.encode().unwrap(), 
                valid_sample_chain.encode().unwrap(),
            );

        }

        // how to handle > 64 samples
        #[ignore]
        #[test]
        fn test_default_67_samples() {

            let wav_fps = get_test_wav_paths("data/tests/3/wavs/").unwrap();
            let (loop_config, trim_config, slices) = create_sample_chain_encoded_from_wavfiles(wav_fps).unwrap();

            let composed_chain = SampleAttributes
                ::new(
                    &175.0,
                    &SampleAttributeTimestrechMode::Off, 
                    &SampleAttributeTrigQuantizationMode::PatternLength, 
                    &24.0,
                    &trim_config,
                    &loop_config,
                    &slices, 
                )
                .unwrap()
            ;

            let valid_ot_fp = "data/tests/3/chain.ot";
            let valid_sample_chain = read_valid_sample_chain(&valid_ot_fp).unwrap();

            assert_eq!(
                composed_chain, 
                valid_sample_chain,
            );

            assert_eq!(
                composed_chain.encode().unwrap(), 
                valid_sample_chain.encode().unwrap(),
            );

        }

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
                loop_start: 0,
            };

            let slices: [Slice; 64] = [default_slice; 64];

            let slice_conf = Slices {
                slices: slices,
                count: 0,
            };

            (trim_config, loop_config, slice_conf)

        }


        #[ignore]
        #[test]
        fn test_non_default_tempo_3_samples() {

            let (trim_conf, loop_conf, slices) = create_mock_configs_blank();

            let composed_chain = SampleAttributes
                ::new(
                    &147.0,
                    &SampleAttributeTimestrechMode::Off, 
                    &SampleAttributeTrigQuantizationMode::PatternLength, 
                    &0.0,
                    &trim_conf,
                    &loop_conf,
                    &slices,
                );

            assert!(composed_chain.is_err());

        }

        #[ignore]
        #[test]
        fn test_non_default_quantize_3_samples() {

            let wav_fps = get_test_wav_paths("data/tests/3/wavs/").unwrap();
            let (loop_config, trim_config, slices) = create_sample_chain_encoded_from_wavfiles(wav_fps).unwrap();

            let composed_chain = SampleAttributes
                ::new(
                    &125.0,
                    &SampleAttributeTimestrechMode::Off, 
                    &SampleAttributeTrigQuantizationMode::PatternLength, 
                    &0.0,
                    &trim_config,
                    &loop_config,
                    &slices,
                )
                .unwrap()
            ;

            let valid_ot_fp = "data/tests/3/chain.ot";
            let valid_sample_chain = read_valid_sample_chain(&valid_ot_fp).unwrap();

            assert_eq!(
                composed_chain,
                valid_sample_chain,
            );

            assert_eq!(
                composed_chain.encode().unwrap(), 
                valid_sample_chain.encode().unwrap(),
            );

        }


        #[ignore]
        #[test]
        fn test_non_default_gain_3_samples() {

            let wav_fps = get_test_wav_paths("data/tests/3/wavs/").unwrap();
            let (loop_config, trim_config, slices) = create_sample_chain_encoded_from_wavfiles(wav_fps).unwrap();

            let composed_chain = SampleAttributes
                ::new(
                    &125.0,
                    &SampleAttributeTimestrechMode::Off, 
                    &SampleAttributeTrigQuantizationMode::PatternLength, 
                    &24.0,
                    &trim_config,
                    &loop_config,
                    &slices,
                )
                .unwrap()
            ;

            let valid_ot_fp = "data/tests/3/chain.ot";
            let valid_sample_chain = read_valid_sample_chain(&valid_ot_fp).unwrap();

            assert_eq!(
                composed_chain, 
                valid_sample_chain,
            );

            assert_eq!(
                composed_chain.encode().unwrap(), 
                valid_sample_chain.encode().unwrap(),
            );

            assert_eq!(composed_chain, valid_sample_chain);

        }

        #[test]
        fn test_oob_tempo() {

            let (trim_conf, loop_conf, slices) = create_mock_configs_blank();

            let composed_chain = SampleAttributes
                ::new(
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
        fn test_invalid_gain() {

            let (trim_conf, loop_conf, slices) = create_mock_configs_blank();

            let composed_chain = SampleAttributes
                ::new(
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
