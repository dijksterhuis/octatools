/*
Reading and Writing Octatrack .ot sample chain metadata files.

TODOs:
* Default: https://doc.rust-lang.org/std/default/trait.Default.html
*/

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use log::{error, info, warn, debug};
use bincode::ErrorKind;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

use crate::{results::*, wavfile::WavFile};
use crate::constants::{
    HEADER_BYTES,
    UNKNOWN_BYTES,
    SAMPLE_RATE,
};
use crate::config_yaml_samplechain::YamlChainConfigSamplechain;
use crate::wavfile::chain_wavfiles_64_batch;

/// Enum options for otsample file settings.
pub mod options {
    /*
    Available options for certain Octatrack sample chain settings.
    */

    use crate::results::*;

    use serde::{Deserialize, Serialize};

    /// Octatrack manual: 
    /// > sets whether timestretch should be applied to the sample or not. 
    /// > Different timestretch algorithms exist
    #[derive(PartialEq, Debug, Clone, Default, Serialize, Deserialize)]
    pub enum SampleTimestrechModes {
        // > does not apply timestretch to the sample
        #[default]
        Off,
        // > an algorithm suitable for most material.
        Normal,
        // > timestretch algorithm especially useful for rhythmic material.
        Beat,
    }


    impl SampleTimestrechModes {
        pub fn from_value(v: u32) -> Result<SampleTimestrechModes, ()> {
            match v {
                0 => Ok(SampleTimestrechModes::Off),
                2 => Ok(SampleTimestrechModes::Normal),
                3 => Ok(SampleTimestrechModes::Beat),
                _ => Err(())
            }
        }
        pub fn value(self) -> U32Result {
            match self {
                SampleTimestrechModes::Off => Ok(0),
                SampleTimestrechModes::Normal => Ok(2),
                SampleTimestrechModes::Beat => Ok(3),
            }
        }
    }

    /// Octatrack manual: 
    /// > displays the amount of bars the looped section of the sample consists of.
    /// > Altering this setting will alter the ORIGINAL TEMPO and TRIM LEN (BARS) settings.
    /// > An arrow will appear next to the LOOP LEN (BARS) setting, indicating this setting
    /// > has priority.
    #[derive(PartialEq, Debug, Clone, Default, Serialize, Deserialize)]
    pub enum SampleLoopModes {
        // > does not apply timestretch to the sample
        #[default]
        Off,
        // > an algorithm suitable for most material.
        Normal,
        // > timestretch algorithm especially useful for rhythmic material.
        PingPong,
    }

    impl SampleLoopModes {
        pub fn from_value(v: u32) -> Result<SampleLoopModes, ()> {
            match v {
                0 => Ok(SampleLoopModes::Off),
                1 => Ok(SampleLoopModes::Normal),
                2 => Ok(SampleLoopModes::PingPong),
                _ => Err(())
            }
        }
        pub fn value(self) -> U32Result {
            match self {
                SampleLoopModes::Off => Ok(0),
                SampleLoopModes::Normal => Ok(1),
                SampleLoopModes::PingPong => Ok(2),
            }
        }
    }

    /// Octatrack manual: 
    /// > makes it possible to quantize manual trigging of recorder buffers,
    /// > Pickup machines and Flex and Static samples and slices. Manual
    /// > trigging is done by for example pressing [TRACK] + [PLAY] or the
    /// > last eight [TRIG] keys.
    /// > Samples initiated by the sequencer will not be quantized.
    #[derive(PartialEq, Debug, Clone, Default, Serialize, Deserialize)]
    pub enum SampleTrigQuantizationModes {
        // > Make the sample play back immediately once it is trigged.
        // > This is the default option
        #[default]
        Direct,
        // > After the sample has been trigged, start sample playback
        // > once the pattern has played its full length.
        PatternLength,
        // > Start sample playback after the set amount of sequencer steps
        OneStep,
        TwoSteps,
        ThreeSteps,
        FourSteps,
        SixSteps,
        EightSteps,
        TwelveSteps,
        SixteenSteps,
        TwentyFourSteps,
        ThirtyTwoSteps,
        FourtyEightSteps,
        SixtyFourSteps,
        NinetySixSteps,
        OneTwentyEightSteps,
        OneNinetyTwoSteps,
        TwoFiveSixSteps,
    }

    impl SampleTrigQuantizationModes {
        pub fn from_value(v: u32) -> Result<SampleTrigQuantizationModes, ()> {
            match v {
                0 => Ok(SampleTrigQuantizationModes::Direct),
                255 => Ok(SampleTrigQuantizationModes::PatternLength),
                1 => Ok(SampleTrigQuantizationModes::OneStep),
                2 => Ok(SampleTrigQuantizationModes::TwoSteps),
                3 => Ok(SampleTrigQuantizationModes::ThreeSteps),
                4 => Ok(SampleTrigQuantizationModes::FourSteps),
                5 => Ok(SampleTrigQuantizationModes::SixSteps),
                6 => Ok(SampleTrigQuantizationModes::EightSteps),
                7 => Ok(SampleTrigQuantizationModes::TwelveSteps),
                8 => Ok(SampleTrigQuantizationModes::SixteenSteps),
                9 => Ok(SampleTrigQuantizationModes::TwentyFourSteps),
                10 => Ok(SampleTrigQuantizationModes::ThirtyTwoSteps),
                11 => Ok(SampleTrigQuantizationModes::FourtyEightSteps),
                12 => Ok(SampleTrigQuantizationModes::SixtyFourSteps),
                13 => Ok(SampleTrigQuantizationModes::NinetySixSteps),
                14 => Ok(SampleTrigQuantizationModes::OneTwentyEightSteps),
                15 => Ok(SampleTrigQuantizationModes::OneNinetyTwoSteps),
                16 => Ok(SampleTrigQuantizationModes::TwoFiveSixSteps),
                _ => Err(())

            }
        }
        pub fn value(self) -> U32Result {
            match self {
                SampleTrigQuantizationModes::Direct => Ok(0),
                SampleTrigQuantizationModes::PatternLength => Ok(255),
                SampleTrigQuantizationModes::OneStep => Ok(1),
                SampleTrigQuantizationModes::TwoSteps => Ok(2),
                SampleTrigQuantizationModes::ThreeSteps => Ok(3),
                SampleTrigQuantizationModes::FourSteps => Ok(4),
                SampleTrigQuantizationModes::SixSteps => Ok(5),
                SampleTrigQuantizationModes::EightSteps => Ok(6),
                SampleTrigQuantizationModes::TwelveSteps => Ok(7),
                SampleTrigQuantizationModes::SixteenSteps => Ok(8),
                SampleTrigQuantizationModes::TwentyFourSteps => Ok(9),
                SampleTrigQuantizationModes::ThirtyTwoSteps => Ok(10),
                SampleTrigQuantizationModes::FourtyEightSteps => Ok(11),
                SampleTrigQuantizationModes::SixtyFourSteps => Ok(12),
                SampleTrigQuantizationModes::NinetySixSteps => Ok(13),
                SampleTrigQuantizationModes::OneTwentyEightSteps => Ok(14),
                SampleTrigQuantizationModes::OneNinetyTwoSteps => Ok(15),
                SampleTrigQuantizationModes::TwoFiveSixSteps => Ok(16),
            }
        }
    }


    /// "Specification" tests ... ie. guarantee that enum values are correct
    #[cfg(test)]
    mod test_specification {

        mod ot_trig_quantize_mode {

            mod value {
                use crate::otsample::options::SampleTrigQuantizationModes;

                #[test]
                fn test_direct() {
                    assert_eq!(SampleTrigQuantizationModes::Direct.value().unwrap(), 0);
                }
                #[test]
                fn test_patternlen() {
                    assert_eq!(SampleTrigQuantizationModes::PatternLength.value().unwrap(), 255);
                }
                #[test]
                fn test_1() {
                    assert_eq!(SampleTrigQuantizationModes::OneStep.value().unwrap(), 1);
                }
                #[test]
                fn test_2() {
                    assert_eq!(SampleTrigQuantizationModes::TwoSteps.value().unwrap(), 2);
                }
                #[test]
                fn test_3() {
                    assert_eq!(SampleTrigQuantizationModes::ThreeSteps.value().unwrap(), 3);
                }
                #[test]
                fn test_4() {
                    assert_eq!(SampleTrigQuantizationModes::FourSteps.value().unwrap(), 4);
                }
                #[test]
                fn test_6() {
                    assert_eq!(SampleTrigQuantizationModes::SixSteps.value().unwrap(), 5);
                }
                #[test]
                fn test_8() {
                    assert_eq!(SampleTrigQuantizationModes::EightSteps.value().unwrap(), 6);
                }
                #[test]
                fn test_12() {
                    assert_eq!(SampleTrigQuantizationModes::TwelveSteps.value().unwrap(), 7);
                }
                #[test]
                fn test_16() {
                    assert_eq!(SampleTrigQuantizationModes::SixteenSteps.value().unwrap(), 8);
                }
                #[test]
                fn test_24() {
                    assert_eq!(SampleTrigQuantizationModes::TwentyFourSteps.value().unwrap(), 9);
                }
                #[test]
                fn test_32() {
                    assert_eq!(SampleTrigQuantizationModes::ThirtyTwoSteps.value().unwrap(), 10);
                }
                #[test]
                fn test_48() {
                    assert_eq!(SampleTrigQuantizationModes::FourtyEightSteps.value().unwrap(), 11);
                }
                #[test]
                fn test_64() {
                    assert_eq!(SampleTrigQuantizationModes::SixtyFourSteps.value().unwrap(), 12);
                }
                #[test]
                fn test_96() {
                    assert_eq!(SampleTrigQuantizationModes::NinetySixSteps.value().unwrap(), 13);
                }
                #[test]
                fn test_128() {
                    assert_eq!(SampleTrigQuantizationModes::OneTwentyEightSteps.value().unwrap(), 14);
                }
                #[test]
                fn test_192() {
                    assert_eq!(SampleTrigQuantizationModes::OneNinetyTwoSteps.value().unwrap(), 15);
                }
                #[test]
                fn test_256() {
                    assert_eq!(SampleTrigQuantizationModes::TwoFiveSixSteps.value().unwrap(), 16);
                }
            }
        
            mod from_value {
                use crate::otsample::options::SampleTrigQuantizationModes;

                #[test]
                fn test_error() {
                    assert_eq!(
                        SampleTrigQuantizationModes::from_value(200),
                        Err(()),
                    );
                }
                #[test]
                fn test_direct() {
                    assert_eq!(
                        SampleTrigQuantizationModes::Direct,
                        SampleTrigQuantizationModes::from_value(0).unwrap()
                    );
                }
                #[test]
                fn test_patternlen() {
                    assert_eq!(
                        SampleTrigQuantizationModes::PatternLength,
                        SampleTrigQuantizationModes::from_value(255).unwrap()
                    );
                }
                #[test]
                fn test_1() {
                    assert_eq!(
                        SampleTrigQuantizationModes::OneStep,
                        SampleTrigQuantizationModes::from_value(1).unwrap()
                    );
                }
                #[test]
                fn test_2() {
                    assert_eq!(
                        SampleTrigQuantizationModes::TwoSteps,
                        SampleTrigQuantizationModes::from_value(2).unwrap()
                    );
                }
                #[test]
                fn test_3() {
                    assert_eq!(
                        SampleTrigQuantizationModes::ThreeSteps,
                        SampleTrigQuantizationModes::from_value(3).unwrap()
                    );
                }
                #[test]
                fn test_4() {
                    assert_eq!(
                        SampleTrigQuantizationModes::FourSteps,
                        SampleTrigQuantizationModes::from_value(4).unwrap()
                    );
                }
                #[test]
                fn test_6() {
                    assert_eq!(
                        SampleTrigQuantizationModes::SixSteps,
                        SampleTrigQuantizationModes::from_value(5).unwrap()
                    );
                }
                #[test]
                fn test_8() {
                    assert_eq!(
                        SampleTrigQuantizationModes::EightSteps,
                        SampleTrigQuantizationModes::from_value(6).unwrap()
                    );
                }
                #[test]
                fn test_12() {
                    assert_eq!(
                        SampleTrigQuantizationModes::TwelveSteps,
                        SampleTrigQuantizationModes::from_value(7).unwrap()
                    );
                }
                #[test]
                fn test_16() {
                    assert_eq!(
                        SampleTrigQuantizationModes::SixteenSteps,
                        SampleTrigQuantizationModes::from_value(8).unwrap()
                    );
                }
                #[test]
                fn test_24() {
                    assert_eq!(
                        SampleTrigQuantizationModes::TwentyFourSteps,
                        SampleTrigQuantizationModes::from_value(9).unwrap()
                    );
                }
                #[test]
                fn test_32() {
                    assert_eq!(
                        SampleTrigQuantizationModes::ThirtyTwoSteps,
                        SampleTrigQuantizationModes::from_value(10).unwrap()
                    );
                }
                #[test]
                fn test_48() {
                    assert_eq!(
                        SampleTrigQuantizationModes::FourtyEightSteps,
                        SampleTrigQuantizationModes::from_value(11).unwrap()
                    );
                }
                #[test]
                fn test_64() {
                    assert_eq!(
                        SampleTrigQuantizationModes::SixtyFourSteps,
                        SampleTrigQuantizationModes::from_value(12).unwrap()
                    );
                }
                #[test]
                fn test_96() {
                    assert_eq!(
                        SampleTrigQuantizationModes::NinetySixSteps,
                        SampleTrigQuantizationModes::from_value(13).unwrap()
                    );
                }
                #[test]
                fn test_128() {
                    assert_eq!(
                        SampleTrigQuantizationModes::OneTwentyEightSteps,
                        SampleTrigQuantizationModes::from_value(14).unwrap()
                    );
                }
                #[test]
                fn test_192() {
                    assert_eq!(
                        SampleTrigQuantizationModes::OneNinetyTwoSteps,
                        SampleTrigQuantizationModes::from_value(15).unwrap()
                    );
                }
                #[test]
                fn test_256() {
                    assert_eq!(
                        SampleTrigQuantizationModes::TwoFiveSixSteps,
                        SampleTrigQuantizationModes::from_value(16).unwrap()
                    );
                }
            }


        }

        mod ot_timestrech_mode {
        
            mod value {
                use crate::otsample::options::SampleTimestrechModes;
    
                #[test]
                fn test_off_value() {
                    assert_eq!(SampleTimestrechModes::Off.value().unwrap(), 0);
                }
                #[test]
                fn test_normal_value() {
                    assert_eq!(SampleTimestrechModes::Normal.value().unwrap(), 2);
                }
                #[test]
                fn test_beat_value() {
                    assert_eq!(SampleTimestrechModes::Beat.value().unwrap(), 3);
                }
            }

            mod from_value {
                use crate::otsample::options::SampleTimestrechModes;
    
                #[test]
                fn test_error() {
                    // not in a sequental range with other values
                    // dunno why they implemented it to skip value of 1, possible bug or easter egg?
                    assert_eq!(
                        SampleTimestrechModes::from_value(1),
                        Err(()),
                    );
                    // do a slightly exhausitve check, but don't test the whole u32 range
                    // as it's not worth the performance drain
                    for i in 4..u8::MAX {
                        assert_eq!(
                            SampleTimestrechModes::from_value(i as u32),
                            Err(()),
                        );    
                    }  
                }
                #[test]
                fn test_off_from_value() {
                    assert_eq!(
                        SampleTimestrechModes::Off,
                        SampleTimestrechModes::from_value(0).unwrap()
                    );
                }
                #[test]
                fn test_normal_from_value() {
                    assert_eq!(
                        SampleTimestrechModes::Normal,
                        SampleTimestrechModes::from_value(2).unwrap()
                    );
                }
                #[test]
                fn test_beat_from_value() {
                    assert_eq!(
                        SampleTimestrechModes::Beat,
                        SampleTimestrechModes::from_value(3).unwrap()
                    );
                }
            }
        }

        mod ot_loop_mode {

            mod value {
                use crate::otsample::options::SampleLoopModes;
            
                #[test]
                fn test_off_value() {
                    assert_eq!(SampleLoopModes::Off.value().unwrap(), 0);
                }
                #[test]
                fn test_normal_value() {
                    assert_eq!(SampleLoopModes::Normal.value().unwrap(), 1);
                }
                #[test]
                fn test_beat_value() {
                    assert_eq!(SampleLoopModes::PingPong.value().unwrap(), 2);
                }
            }

            mod from_value {
                use crate::otsample::options::SampleLoopModes;
            
                #[test]
                fn test_error() {
                    // do a slightly exhausitve check, but don't test the whole u32 range
                    // as it's not worth the performance drain
                    for i in 3..u8::MAX {
                        assert_eq!(
                            SampleLoopModes::from_value(i as u32),
                            Err(()),
                        );
                    }
                }
                #[test]
                fn test_off_from_value() {
                    assert_eq!(
                        SampleLoopModes::Off,
                        SampleLoopModes::from_value(0).unwrap()
                    );
                }
                #[test]
                fn test_normal_from_value() {
                    assert_eq!(
                        SampleLoopModes::Normal,
                        SampleLoopModes::from_value(1).unwrap()
                    );
                }
                #[test]
                fn test_beat_from_value() {
                    assert_eq!(
                        SampleLoopModes::PingPong,
                        SampleLoopModes::from_value(2).unwrap()
                    );
                }
            }
        }
    }




}


/// An OT Sample's Trim settings
/// * start of full sample
/// * end of full sample
/// * length of trim to play (before stop or loop)
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct SampleTrimConfig {
    pub start: u32,
    pub end: u32,
    pub length: u32,
}

impl SampleTrimConfig {
    pub fn from_decoded(decoded: &SampleChain) -> Result<Self, Box<dyn Error>> {
        let new = SampleTrimConfig {
            start: decoded.trim_start,
            end: decoded.trim_end,
            length: decoded.trim_len,
        };

        Ok(new)
    }

}

/// An OT Sample's Loop settings
/// * start of loop position
/// * length of the loop
/// * how to perform the looping (ref: SampleLoopModeConstants and SampleLoopModeSetting)
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct SampleLoopConfig {
    pub start: u32,
    pub length: u32,
    pub mode: u8,
}

impl SampleLoopConfig {
    pub fn from_decoded(decoded: &SampleChain) -> Result<Self, Box<dyn Error>> {
        Ok(
            SampleLoopConfig {
                start: decoded.loop_start,
                length: decoded.loop_len,
                mode: decoded.loop_mode as u8,
            }
        )
    }
}

/// Positions of a slice within a sliced WAV file
/// (a sliced WAV file is multiple samples joined in series)
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub struct Slice {
    pub trim_start: u32,
    pub trim_end: u32,
    pub loop_start: u32,
}

impl Slice {
    pub fn as_bswapped(&self) -> Self {
        let bswapped_slice = Slice {
            trim_start: self.trim_start.swap_bytes(),
            trim_end: self.trim_end.swap_bytes(),
            loop_start: self.loop_start.swap_bytes(),
        };

        bswapped_slice
    }

    pub fn from_wavfile(wavfile: &WavFile, offset: u32) -> Result<Slice, Box<dyn Error>> {
        Ok(
                Slice {
                trim_start: 0 + offset,
                trim_end: offset + wavfile.len,
                loop_start: 0xFFFFFFFF,
            }
        )
    }

}


pub struct Slices {
    pub slices: [Slice; 64],
    pub count: u32,
    pub total_samples: u32,
}

impl Slices {

    fn get_vec_from_wavfiles(wavfiles: &Vec<WavFile>, offset: &u32) -> Result<Vec<Slice>, Box<dyn Error>> {

        let mut off = offset.clone();
        let mut slices: Vec<Slice> = Vec::new();
    
        for w in wavfiles.iter() {
            slices.push(Slice::from_wavfile(w, off).unwrap());
            off += w.len as u32;
        }
    
        Ok(slices)
    }

    pub fn from_wavfiles(wavfiles: &Vec<WavFile>, offset: &u32) -> Result<Slices, Box<dyn Error>> {

        let new_slices: _ = Slices::get_vec_from_wavfiles(&wavfiles, &offset).unwrap();
    
        let default_slice = Slice {
            trim_end: 0,
            trim_start: 0,
            loop_start: 0,
        };
    
        let mut slices_arr: [Slice; 64] = [default_slice; 64];
        for (i, slice_vec) in new_slices.iter().enumerate() {
            slices_arr[i] = slice_vec.clone();
        }

        Ok(
            Slices {
                slices: slices_arr,
                total_samples: wavfiles.iter().map(|x| x.len as u32).sum(),
                count: wavfiles.len() as u32
            }
    
        )
    }
    

}

// TODO: This should be for one WavFile, then map this function over the Vec
pub fn get_otsample_n_bars_from_wavfiles(wavs: &Vec<WavFile>, tempo_bpm: &f32) -> U32Result {
    let total_samples: u32 = wavs.iter().map(|x| x.len as u32).sum();
    let beats = total_samples as f32 / (SAMPLE_RATE as f32 * 60.0 * 4.0);
    let mut bars = ((tempo_bpm * 4.0 * beats) + 0.5) * 0.25;
    bars -= bars % 0.25;
    Ok((bars * 100.0) as u32)
}


/*
Notes for each field:

* Header is always: `FORM....DPS1SMPA`Octatack
* Blank values then a single 2 value: `......` (dunno why)
* tempo is always the true BPM multiplied by 24 
* by default, trim len and loop len are: (((tempo * totalSampleCount) / (sampleRate * 60.0 * 4.0)) + 0.5) * 25;
* loop len by defalt is equal to loop len. sample chains for drum hits etc don't use this mechanism. 
Although can be used to join Nx samples together in series, without having to do any editing.
So potentially useful for weird poly-rhythmic tape loop kinda stuff. 
* timestrech: 0 = Off; 2 = Normal; 3 = Beat
* loopmode: 0 = Off; 1 = Normal; 2 = PingPong
* gain: default: 0 / 0x30; minimum: -24 / 0x00; maximum: 24; 0x30 
Always add 48 to whatever the value is. I assume this is to ensure 0 < x < 128. 
* quantization: 0x00 = Pattern length; 0xFF = Direct; 1-10 = 1,2,3,4,6,8,12,16,24,32,48,64,96,128,192,256
* trim start: usually can assume 0 (first sample)
* trim_end: usually can same as trim_len (when trime start is 0)
* loop_point: usually can assume same as trim_start 
* Slices: always a64 length array / empty slices are zero valued.
* slice count limits the number of slices seen in the OT menu (as we always have 64 slots available)
* checksum: See the `get_checksum` method documentation for details on why this is weird
*/
/// OT file data struct. General metadata for the sample's configuration on the OT 
/// and the slice array with pointer positions for the sliced WAV.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct SampleChain {
    pub header: [u8; 16],
    pub blank: [u8; 7],
    pub tempo: u32,
    pub trim_len: u32,
    pub loop_len: u32,
    pub stretch: u32,
    pub loop_mode: u32,
    pub gain: u16,
    pub quantization: u8,
    pub trim_start: u32,
    pub trim_end: u32,
    pub loop_start: u32,
    #[serde(with = "BigArray")]
    pub slices: [Slice; 64],
    pub slices_len: u32,
    pub checksum: u16,
}


impl SampleChain {
    pub fn new(
        tempo: &u32,
        stretch: &u32,
        quantization: &u8,
        gain: &u16,
        trim_config: &SampleTrimConfig,
        loop_config: &SampleLoopConfig,
        slices: &Slices,
    ) -> Result<SampleChain, ()> {

        // todo: gain out of bounds for ot settings options
        if *gain > 48 {
            return Err(())
        }

        // todo: gain out of bounds for ot settings options
        if *tempo > 900 {
            return Err(())
        }
        
        // todo: validate that we've got acceptable options
        let bad_loop = options::SampleLoopModes::from_value(loop_config.mode as u32).is_err();
        let bad_stretch = options::SampleTimestrechModes::from_value(*stretch).is_err();
        let bad_quantise = options::SampleTrigQuantizationModes::from_value(*quantization as u32).is_err();

        if bad_loop | bad_stretch | bad_quantise {
            return Err(())
        }

        Ok(
            SampleChain {
                header: HEADER_BYTES,
                blank: UNKNOWN_BYTES,
                gain: *gain,
                stretch: *stretch,
                tempo: *tempo,
                quantization: *quantization,
                trim_start: trim_config.start,
                trim_end: trim_config.end,
                trim_len: trim_config.length,
                loop_start: loop_config.start,
                loop_len: loop_config.length,
                loop_mode: loop_config.mode as u32,
                slices: slices.slices,
                slices_len: slices.count,
                checksum: 0,
            }
        )
    }

    /// bswap all struct fields
    /// WARNING: MUST BE CALLED BEFORE SERIALISATION WHEN SYSTEM IS LITTLE ENDIAN!
    pub fn to_bswapped(&mut self) -> Self {

        let mut bswapped_slices: [Slice; 64] = self.slices.clone();

        for (i, slice) in self.slices.iter().enumerate() {
            bswapped_slices[i] = slice.as_bswapped();
        }

        let bswapped = SampleChain {
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
    pub fn to_file(&self, path: &PathBuf) -> VoidResult {

        use std::fs::File;
        use std::io::Write;

        let bytes: Vec<u8> = self.encode().unwrap();

        let mut file = File::create(path).unwrap();
        let res: VoidResult = file
            .write_all(&bytes)
            .map_err(|e| e.into())
            ;

        res
    }

    fn decode(bytes: &Vec<u8>) -> Result<Self, Box<ErrorKind>> {
        let mut decoded: SampleChain = bincode::deserialize(&bytes[..]).unwrap();
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

    /// Read some `.ot` file into a new struct
    pub fn from_file(path: &str) -> Result<Self, Box<ErrorKind>> {

        let mut infile = File::open(path)?;
        let mut bytes: Vec<u8> = vec![];
        let _: usize = infile.read_to_end(&mut bytes)?;

        let decoded = SampleChain::decode(&bytes).unwrap();

        Ok(decoded)
    }

    // TODO: this is reading a bunch of things I don't want it to read just to create a sample chain
    // these things should be getting created elsewhere -- the wav files etc.
    pub fn from_yaml_conf(chain_config: &YamlChainConfigSamplechain) -> Result<Self, ()> {

        info!("Processing chain data: {:#?}", chain_config);

        debug!("Loading Nx WAV files: {:#?}", chain_config.sample_file_paths.len());

        let mut wavfiles: Vec<WavFile> = Vec::new();
        for wav_file_path in &chain_config.sample_file_paths {
            let wavfile = WavFile::from_file(&wav_file_path).unwrap();
            wavfiles.push(wavfile);
        };

        debug!("Loaded Nx WAV files: {:#?}", wavfiles.len());

        // TODO: Overflow to a new wav file and OT file!
        if wavfiles.len() > 64 {
            warn!("More than 64 samples -- need to overflow here! TODO!");
        };

        debug!("Creating slices from WAV files: {:#?}", wavfiles.len());

        let slices = Slices::from_wavfiles(&wavfiles, &0).unwrap();

        debug!("Creating sliced WAV file: {:#?}", &wavfiles.clone().into_iter().map(|x: WavFile| x.len).collect::<Vec<u32>>());

        let wav_sliced = chain_wavfiles_64_batch(&wavfiles).unwrap();

        debug!("Sliced WAV file sample len: {:#?}", &wav_sliced.len);

        debug!("Working out bar length / trim config / loop config from slices: {:#?}", slices.count);

        let bars = get_otsample_n_bars_from_wavfiles(&wavfiles, &125.0).unwrap();

        let trim_config = SampleTrimConfig {
            start: 0,
            end: wavfiles.iter().map(|x: &WavFile| x.len as u32).sum(),
            length: bars,
        };

        let loop_config = SampleLoopConfig {
            start: 0,
            length: bars,
            mode: 0,
        };

        debug!("Converting config values for OT file: {:#?}", slices.count);

        let chain_ot_settings = chain_config.octatrack_settings.clone();

        let quantization_value: u8 = chain_ot_settings.quantization_mode.value().unwrap() as u8;
        let timestretch_value: u32 = chain_ot_settings.timestretch_mode.value().unwrap();
        let tempo_u32 = chain_ot_settings.bpm as u32;
        let gain_u16 = chain_ot_settings.gain as u16;

        // TODO: Why is this unused?
        let loopmode_value: u32 = chain_ot_settings.loop_mode.value().unwrap();

        // tempo: &u32,
        // stretch: &u32,
        // quantization: &u8,
        // gain: &u16,
        // trim_config: &SampleTrimConfig,
        // loop_config: &SampleLoopConfig,
        // slices: &Slices,

        debug!("Creating sample chain struct: {:#?}", slices.count);

        let chain_data = SampleChain
            ::new(
                &tempo_u32,
                &timestretch_value,
                &quantization_value,
                &gain_u16,
                &trim_config,
                &loop_config,
                &slices
            )
            .unwrap()
        ;

        Ok(chain_data)
    
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
        use super::*;

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
    use super::*;

    mod test_integration_sample_chain_create_vs_read {

        use std::error::Error;
        use std::path::PathBuf;
        use walkdir::{DirEntry, WalkDir};

        use crate::wavfile::WavFile;
        use crate::otsample::get_otsample_n_bars_from_wavfiles;
        use super::{
            SampleChain,
            SampleLoopConfig,
            SampleTrimConfig,
            Slices,
            Slice,
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

        fn create_sample_chain_encoded_from_wavfiles(wav_fps: &Vec<PathBuf>) -> Result<(SampleLoopConfig, SampleTrimConfig, Slices), Box<dyn Error>> {

            let mut wavs: Vec<WavFile> = Vec::new();
            for fp in wav_fps.into_iter() {
                let wav = WavFile::from_file(&fp).unwrap();
                wavs.push(wav);
            }

            let slices_config = Slices::from_wavfiles(&wavs, &0).unwrap();

            let bars = get_otsample_n_bars_from_wavfiles(&wavs, &125.0).unwrap();

            let trim_config = SampleTrimConfig {
                start: 0,
                end: wavs.iter().map(|x| x.len as u32).sum(),
                length: bars,
            };

            let loop_config = SampleLoopConfig {
                start: 0,
                length: bars,
                mode: 0,
            };

            Ok((loop_config, trim_config, slices_config))

        }

        fn read_valid_sample_chain_encoded(path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
            let read_chain = SampleChain
                ::from_file(path)
                .unwrap()
                .encode()
                .unwrap()
                ;
            Ok(read_chain)
        }

        #[test]
        fn test_default_10_samples() {

            let wav_fps = get_test_wav_paths("data/tests/1/wavs/").unwrap();
            let (loop_config, trim_config, slices) = create_sample_chain_encoded_from_wavfiles(&wav_fps).unwrap();

            let composed_chain = SampleChain
                ::new(
                    &125, 
                    &0, 
                    &255, 
                    &0, 
                    &trim_config, 
                    &loop_config, 
                    &slices, 
                )
                .unwrap()
                .encode()
                .unwrap()
                ;

            let valid_ot_fp = "data/tests/1/chain.ot";
            let valid_sample_chain = read_valid_sample_chain_encoded(&valid_ot_fp).unwrap();

            assert_eq!(composed_chain, valid_sample_chain);

        }

        #[test]
        fn test_default_3_samples() {

            let wav_fps = get_test_wav_paths("data/tests/2/wavs/").unwrap();
            let (loop_config, trim_config, slices) = create_sample_chain_encoded_from_wavfiles(&wav_fps).unwrap();

            let composed_chain = SampleChain
                ::new(
                    &125, 
                    &0, 
                    &255, 
                    &0, 
                    &trim_config, 
                    &loop_config, 
                    &slices, 
                )
                .unwrap()
                .encode()
                .unwrap()
                ;

            let valid_ot_fp = "data/tests/2/chain.ot";
            let valid_sample_chain = read_valid_sample_chain_encoded(&valid_ot_fp).unwrap();

            assert_eq!(composed_chain, valid_sample_chain);

        }

        #[ignore]
        #[test]
        fn test_default_64_samples() {

            let wav_fps = get_test_wav_paths("data/tests/3/wavs/").unwrap();
            let (loop_config, trim_config, slices) = create_sample_chain_encoded_from_wavfiles(&wav_fps).unwrap();

            let composed_chain = SampleChain
                ::new(
                    &175,
                    &2,
                    &255,
                    &24,
                    &trim_config,
                    &loop_config,
                    &slices, 
                )
                .unwrap()
                .encode()
                .unwrap()
                ;

            let valid_ot_fp = "data/tests/3/chain.ot";
            let valid_sample_chain = read_valid_sample_chain_encoded(&valid_ot_fp).unwrap();

            assert_eq!(composed_chain, valid_sample_chain);

        }

        // how to handle > 64 samples
        #[ignore]
        #[test]
        fn test_default_67_samples() {

            let wav_fps = get_test_wav_paths("data/tests/3/wavs/").unwrap();
            let (loop_config, trim_config, slices) = create_sample_chain_encoded_from_wavfiles(&wav_fps).unwrap();

            let composed_chain = SampleChain
                ::new(
                    &175,
                    &2,
                    &255,
                    &24,
                    &trim_config,
                    &loop_config,
                    &slices, 
                )
                .unwrap()
                .encode()
                .unwrap()
                ;

            let valid_ot_fp = "data/tests/3/chain.ot";
            let valid_sample_chain = read_valid_sample_chain_encoded(&valid_ot_fp).unwrap();

            assert_eq!(composed_chain, valid_sample_chain);

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
                mode: 0,
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
                total_samples: 0,
            };

            (trim_config, loop_config, slice_conf)

        }


        #[ignore]
        #[test]
        fn test_non_default_tempo_3_samples() {

            let (trim_conf, loop_conf, slices) = create_mock_configs_blank();

            let composed_chain = SampleChain
                ::new(
                    &147,
                    &0,
                    &255,
                    &0,
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
            let (loop_config, trim_config, slices) = create_sample_chain_encoded_from_wavfiles(&wav_fps).unwrap();

            let composed_chain = SampleChain
                ::new(
                    &125,
                    &2,
                    &0,
                    &0,
                    &trim_config,
                    &loop_config,
                    &slices,
                )
                .unwrap()
                .encode()
                .unwrap()
                ;

            let valid_ot_fp = "data/tests/3/chain.ot";
            let valid_sample_chain = read_valid_sample_chain_encoded(&valid_ot_fp).unwrap();

            assert_eq!(composed_chain, valid_sample_chain);

        }


        #[ignore]
        #[test]
        fn test_non_default_gain_3_samples() {

            let wav_fps = get_test_wav_paths("data/tests/3/wavs/").unwrap();
            let (loop_config, trim_config, slices) = create_sample_chain_encoded_from_wavfiles(&wav_fps).unwrap();

            let composed_chain = SampleChain
                ::new(
                    &125,
                    &0,
                    &255,
                    &24,
                    &trim_config,
                    &loop_config,
                    &slices,
                )
                .unwrap()
                .encode()
                .unwrap()
                ;

            let valid_ot_fp = "data/tests/3/chain.ot";
            let valid_sample_chain = read_valid_sample_chain_encoded(&valid_ot_fp).unwrap();

            assert_eq!(composed_chain, valid_sample_chain);

        }

        #[test]
        fn test_oob_tempo() {

            let (trim_conf, loop_conf, slices) = create_mock_configs_blank();

            let composed_chain = SampleChain
                ::new(
                    &10000,
                    &0,
                    &255,
                    &0,
                    &trim_conf,
                    &loop_conf,
                    &slices,
                );

            assert!(composed_chain.is_err());
        }

        #[test]
        fn test_oob_stretch() {

            let (trim_conf, loop_conf, slices) = create_mock_configs_blank();

            let composed_chain = SampleChain
                ::new(
                    &125,
                    &300,
                    &255,
                    &0,
                    &trim_conf,
                    &loop_conf,
                    &slices,
                );

            assert!(composed_chain.is_err());
        }

        #[test]
        fn test_invalid_quantize() {

            let (trim_conf, loop_conf, slices) = create_mock_configs_blank();

            let composed_chain = SampleChain
                ::new(
                    &125,
                    &0,
                    &23,
                    &0,
                    &trim_conf,
                    &loop_conf,
                    &slices,
                );

            assert!(composed_chain.is_err());
        }


        #[test]
        fn test_invalid_gain() {

            let (trim_conf, loop_conf, slices) = create_mock_configs_blank();

            let composed_chain = SampleChain
                ::new(
                    &125,
                    &0,
                    &255,
                    &300,
                    &trim_conf,
                    &loop_conf,
                    &slices,
                );

            assert!(composed_chain.is_err());
        }


    }

}
