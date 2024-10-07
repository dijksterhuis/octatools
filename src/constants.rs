use hound::{WavSpec, SampleFormat::Int};


pub const SAMPLE_RATE: u16 = 44100;
pub const HEADER_BYTES: [u8; 16] = [0x46,0x4F,0x52,0x4D,0x00,0x00,0x00,0x00,0x44,0x50,0x53,0x31,0x53,0x4D,0x50,0x41];
pub const UNKNOWN_BYTES: [u8; 7] = [0x00,0x00,0x00,0x00,0x00,0x02,0x00];
pub const AVAILABLE_SAMPLE_RATES: [u16; 2] = [44100, 48000];

// in `hexdump -C` format:
// ```
// FORM....DPS1SMPA
// ......
// ```
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

// acceptable octatrack files according to the manual
pub const OCTATRACK_AUDIO_FILE_FORMATS: [&str; 2] = ["wav",  "aiff"];
pub const OCTATRACK_AUDIO_FILE_SAMPLE_RATE: u16 = 44100;
pub const OCTATRACK_AUDIO_FILE_BIT_DEPTH: [u8; 2] = [16, 24];
pub const OCTATRACK_AUDIO_FILE_CHANNELS: [u8; 2] = [1, 2];


pub const OCTATRACK_COMPATIBLE_HOUND_WAVSPECS: [WavSpec; 4] = [
    WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: Int,
    },
    WavSpec {
        channels: 2,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: Int,
    },
    WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 24,
        sample_format: Int,
    },
    WavSpec {
        channels: 2,
        sample_rate: 44100,
        bits_per_sample: 24,
        sample_format: Int,
    },
    
];