//! Various global constants.

/// Default sample rate of a wav file.
// TODO: Remove this!

pub const DEFAULT_SAMPLE_RATE: u16 = 44100;

/// (Not in use) Sample rates available to the application for WAV files

pub const AVAILABLE_SAMPLE_RATES: [u16; 2] = [44100, 48000];

/// Acceptable audio file formats as per the Octatrack manual.
pub const OCTATRACK_AUDIO_FILE_FORMATS: [&str; 2] = ["wav", "aiff"];

/// Acceptable audio file sample rates as per the Octatrack manual.
pub const OCTATRACK_AUDIO_FILE_SAMPLE_RATE: u16 = 44100;

/// Acceptable audio file bit depths as per the Octatrack manual.
pub const OCTATRACK_AUDIO_FILE_BIT_DEPTH: [u8; 2] = [16, 24];

/// Acceptable audio file number of channels as per the Octatrack manual.
pub const OCTATRACK_AUDIO_FILE_CHANNELS: [u8; 2] = [1, 2];

#[derive(Debug, PartialEq)]
pub struct AudioSpec {
    pub channels: u8,
    pub sample_rate: u32,
    pub bit_depth: u8,
}

/// Acceptable WAV file specifications as `hound::WavSpec` structs.
pub const OCTATRACK_COMPATIBLE_AUDIO_SPECS: [AudioSpec; 4] = [
    AudioSpec {
        channels: 1,
        sample_rate: 44100,
        bit_depth: 16,
    },
    AudioSpec {
        channels: 2,
        sample_rate: 44100,
        bit_depth: 16,
    },
    AudioSpec {
        channels: 1,
        sample_rate: 44100,
        bit_depth: 24,
    },
    AudioSpec {
        channels: 2,
        sample_rate: 44100,
        bit_depth: 24,
    },
];
