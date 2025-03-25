//! Various global constants.

/// Default sample rate.
pub const DEFAULT_SAMPLE_RATE: u16 = 44100;

/// Acceptable audio file formats as per the Octatrack manual.
pub const OCTATRACK_AUDIO_FILE_FORMATS: [&str; 2] = ["wav", "aiff"];

/// An 'AudioSpec' is a representation of how an audio file is stored as a file
/// so we can test whether an audio file will be compatible with the Elektron Octatrack.
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
