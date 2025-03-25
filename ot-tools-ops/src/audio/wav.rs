//! Reading and Writing .wav files.

use crate::RBoxErr;
use hound::{self, WavReader, WavSpec};
use log::trace;
use std::io::BufWriter;
use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

#[derive(Debug)]
enum AudioErrors {
    InvalidSampleFormat,
    InvalidBitDepth,
    InvalidSampleRate,
    InvalidChannelCount,
    FadePercentageOOB,
}
impl std::fmt::Display for AudioErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidSampleFormat => write!(f, "Only signed PCM WAV files are supported"),
            Self::InvalidBitDepth => write!(f, "Only 16/24-bit signed PCM WAV files are supported"),
            Self::InvalidSampleRate => write!(f, "Only 44.1kHz WAV files are supported"),
            Self::InvalidChannelCount => write!(f, "Only mono and stereo WAV files are supported"),
            Self::FadePercentageOOB => write!(
                f,
                "Fade percentage parameter out of bounds, must be between 0.0 and 1.0",
            ),
        }
    }
}
impl std::error::Error for AudioErrors {}

pub const ALLOWED_SAMPLE_RATE: u32 = 44100;
pub const ALLOWED_CHANNELS: [u16; 2] = [1, 2];
pub const ALLOWED_BIT_DEPTHS: [u16; 2] = [16, 24];

/// Representation of a wav audio file
#[derive(PartialEq, Debug, Clone)]
pub struct WavFile {
    /// `hound` specification struct for the Wav file.
    pub spec: WavSpec,

    /// Number of audio samples in the Wav file.
    pub len: u32,

    /// Audio samples
    pub samples: Vec<f32>, // cannot use Copy trait

    /// File path of the Wav file
    pub file_path: PathBuf,
}

// TODO: when I've got AIFF reading/writing sorted
// #[derive(PartialEq, Debug, Clone)]
// pub struct AudioFile {
//     pub bit_depth: usize,
//     pub channels: usize,
//     pub len: usize,
//     pub samples: Vec<f32>, // cannot use Copy trait
//     pub fname: String,
//     pub fpath: PathBuf,
// }

// the branches are not the same, i don't know why clippy is saying they are.
#[allow(clippy::if_same_then_else)]
fn f32_to_i32(v: f32) -> i32 {
    // need to use explicit values here
    // the boundary value i32::MAX/i32::MIN clip for some reason
    // when converting to f32
    if v > 0.0 {
        (v * (2_u32.pow(32 - 1) - 1) as f32) as i32
    } else {
        (v * 2_u32.pow(32 - 1) as f32) as i32
    }
}

fn f32_to_i24(v: f32) -> i32 {
    f32_to_i32(v) >> 8
}

fn f32_to_i16(v: f32) -> i16 {
    (f32_to_i32(v) >> 16) as i16
}

#[allow(dead_code)]
fn i32_to_f32(v: i32) -> f32 {
    if v > 0 {
        v as f32 / (2_u32.pow(32 - 1) - 1) as f32
    } else {
        v as f32 / 2_u32.pow(32 - 1) as f32
    }
}

fn i24_to_f32(v: i32) -> f32 {
    if v > 0 {
        v as f32 / (2_u32.pow(24 - 1) - 1) as f32
    } else {
        v as f32 / 2_u32.pow(24 - 1) as f32
    }
}

fn i16_to_f32(v: i16) -> f32 {
    if v > 0 {
        v as f32 / (2_u32.pow(16 - 1) - 1) as f32
    } else {
        v as f32 / 2_u32.pow(16 - 1) as f32
    }
}

fn read_wav_i16_samples(reader: &mut WavReader<BufReader<File>>) -> RBoxErr<Vec<f32>> {
    Ok(reader
        .samples::<i16>()
        .map(|x| i16_to_f32(x.unwrap()))
        .collect::<Vec<_>>())
}

fn read_wav_i24_samples(reader: &mut WavReader<BufReader<File>>) -> RBoxErr<Vec<f32>> {
    Ok(reader
        .samples::<i32>()
        .map(|x| i24_to_f32(x.unwrap()))
        .collect::<Vec<_>>())
}

fn write_wav_16_bit(spec: &WavSpec, samples: &[f32], buf: &mut BufWriter<File>) -> RBoxErr<()> {
    let spec_ex = hound::WavSpecEx {
        spec: *spec,
        bytes_per_sample: 2,
    };

    let mut writer = hound::WavWriter::new_with_spec_ex(buf, spec_ex)?;

    for sample in samples.iter() {
        writer.write_sample(f32_to_i16(*sample))?;
    }

    writer.finalize()?;

    Ok(())
}

fn write_wav_24_bit(spec: &WavSpec, samples: &[f32], buf: &mut BufWriter<File>) -> RBoxErr<()> {
    let spec_ex = hound::WavSpecEx {
        spec: *spec,
        bytes_per_sample: 3,
    };

    let mut writer = hound::WavWriter::new_with_spec_ex(buf, spec_ex)?;

    for sample in samples.iter() {
        writer.write_sample(f32_to_i24(*sample))?;
    }

    writer.finalize()?;

    Ok(())
}

impl WavFile {
    /// Open a wav file into a read buffer
    pub fn open(path: &Path) -> Result<WavReader<BufReader<File>>, hound::Error> {
        trace!("Opening WAV file: path={path:#?}");
        hound::WavReader::open(path)
    }

    /// Read the hound WavSpec for an opened wav file buffer
    pub fn read_spec(reader: &mut WavReader<BufReader<File>>) -> RBoxErr<WavSpec> {
        trace!("Reading WAV reader spec.");
        let spec = hound::WavReader::spec(reader);
        Ok(spec)
    }

    /// Crete a new struct by reading a file located at `path`.
    pub fn from_path(path: &Path) -> RBoxErr<Self> {
        trace!("Reading WAV file from path: {path:#?}");
        let mut reader = WavFile::open(path)?;

        trace!("Reading WAV Spec: path={path:#?}");
        let spec = WavFile::read_spec(&mut reader)?;

        if spec.sample_rate != ALLOWED_SAMPLE_RATE {
            return Err(AudioErrors::InvalidSampleRate.into());
        }

        if !ALLOWED_CHANNELS.contains(&spec.channels) {
            return Err(AudioErrors::InvalidChannelCount.into());
        }

        trace!("Reading WAV Samples: path={path:#?}");
        let samples = match spec.sample_format {
            hound::SampleFormat::Int => match spec.bits_per_sample {
                16 => read_wav_i16_samples(&mut reader),
                24 => read_wav_i24_samples(&mut reader),
                _ => Err(AudioErrors::InvalidBitDepth.into()),
            },
            hound::SampleFormat::Float => Err(AudioErrors::InvalidSampleFormat.into()),
        }?;

        trace!("Read new WAV file: path={path:#?}");
        Ok(WavFile {
            file_path: path.to_path_buf(),
            samples: samples.clone(),
            len: samples.len() as u32 / spec.channels as u32,
            spec,
        })
    }

    /// Crete a new file at the path from the current struct. Can only be used to write 16/24 bit
    /// signed PCM files. Any other specification type will panic (not compatible with Octatrack)
    pub fn to_path(&self, path: &Path) -> RBoxErr<()> {
        trace!("Writing WAV data to file: path={path:#?}");
        let file = File::create(path).unwrap();
        let mut buf_writer = BufWriter::new(file);

        match self.spec.bits_per_sample {
            16 => {
                trace!("Writing 16-bit WAV: path={path:#?}");
                write_wav_16_bit(&self.spec, &self.samples, &mut buf_writer)
            }
            24 => {
                trace!("Writing 24-bit WAV: path={path:#?}");
                write_wav_24_bit(&self.spec, &self.samples, &mut buf_writer)
            }
            _ => Err(AudioErrors::InvalidBitDepth.into()),
        }
    }

    /// Normalise audio samples between 0 and 1
    pub fn normalize(&mut self) -> RBoxErr<()> {
        trace!("Normalizing audio.");
        let max_abs = self
            .samples
            .iter()
            .map(|x| f32::abs(*x))
            .reduce(f32::max)
            .unwrap_or(1.0_f32);

        let normd = self
            .samples
            .iter()
            .map(|x| *x * (1.0 / max_abs))
            .collect::<Vec<f32>>();

        self.samples = normd;
        Ok(())
    }

    // really hacky and simple time stretching
    /// Time stretch audio vector by resampling, inserts duplicate samples to
    /// mimic slowing the audio down and removes samples to mimic speeding it up.
    /// WARNING: Speeding up via resampling is a lossy procedure.
    pub fn resample_time_stretch(&mut self, stretch: i8) -> RBoxErr<()> {
        trace!("Resampling WAV file with time stretch factor: {stretch}");
        let mut resampled: Vec<f32> = vec![];

        let n_channels = self.spec.channels as usize;

        // extending to mimic slowing own the audio
        if stretch < 0 {
            // audio samples are interleaved by channels
            for chn_sample in self.samples.chunks(n_channels) {
                for _ in 0..=stretch.abs() {
                    for x in chn_sample {
                        resampled.push(*x);
                    }
                }
            }
        }

        // remove entries to mimic speed up
        // WARNING: this is lossy
        if stretch > 0 {
            for (idx, chn_sample) in self.samples.chunks(n_channels).enumerate() {
                if idx.rem_euclid((stretch + 1) as usize) == 0 {
                    for x in chn_sample {
                        resampled.push(*x);
                    }
                }
            }
        }

        self.len = resampled.len() as u32;
        self.samples = resampled;
        Ok(())
    }

    /// Linear fade in
    ///
    /// ```text
    ///     x_{n} = x_n.k_n
    ///     forall n < K, n >= 0
    ///     where k_n = k_{n-1} + 1/K; k_0 = 0
    /// ```
    pub fn linear_fade_in(&mut self, percent: f32) -> RBoxErr<()> {
        // floats have no concept of a 'step' in ranges
        #[allow(clippy::manual_range_contains)]
        if percent > 1.0 || percent < 0.0 {
            return Err(AudioErrors::FadePercentageOOB.into());
        }
        trace!("Creating linear fade in for WAV file");

        let mut buf: Vec<f32> = vec![];
        let n_samples = self.samples.len() as f32;
        let n_channels = self.spec.channels as f32;

        // length of fade out in samples
        // note: interleaved so need to divide by n channels
        let n_fade_samples = ((n_samples / n_channels) * percent) as usize;

        // start amplitude factor
        let mut k = 0.0;

        // difference in fade amount per sample step
        let d = 1.0 / n_fade_samples as f32;

        for (idx, chan_samples) in self.samples.chunks(n_channels as usize).enumerate() {
            // if at the relevant % start of the sample, start applying fade in
            if idx <= n_fade_samples {
                for x in chan_samples {
                    buf.push(*x * k);
                }
                k += d;
            } else {
                for x in chan_samples {
                    buf.push(*x * k);
                }
            };
        }

        self.samples = buf;
        Ok(())
    }

    /// Linear fade out
    ///
    /// ```text
    ///     x_{n} = x_n.k_n
    ///     forall n > N - K, n >= 0
    ///     where k_n = k_{n-1} - 1/K; k_0 = 1
    /// ```
    pub fn linear_fade_out(&mut self, percent: f32) -> RBoxErr<()> {
        // floats have no concept of a 'step' in ranges
        #[allow(clippy::manual_range_contains)]
        if percent > 1.0 || percent < 0.0 {
            return Err(AudioErrors::FadePercentageOOB.into());
        }
        trace!("Creating linear fade out for WAV file");

        let mut buf: Vec<f32> = vec![];
        let n_samps = self.samples.len();
        let n_chans = self.spec.channels as usize;

        // deinterleaved sample length
        let n_deint = n_samps / self.spec.channels as usize;

        // length of fade out in deinterleaved samples
        let n_fade_deint = (n_deint as f32 * percent) as usize;

        // start amplitude factor
        let mut k = 1.0;

        // difference in fade amount per sample step
        let d = 1.0 / n_fade_deint as f32;

        for (idx, chan_samples) in self.samples.chunks(n_chans).enumerate() {
            // if at the relevant % end of the sample, start applying fade out
            if idx >= n_deint - n_fade_deint {
                for x in chan_samples {
                    buf.push(*x * k);
                }
                k -= d;
            } else {
                for x in chan_samples {
                    buf.push(*x);
                }
            };
        }

        self.samples = buf;
        Ok(())
    }

    /// Naive upmix of mono signal to an interleaved stereo signal
    /// (duplicate channels, attenuate by 0.5)
    // TODO: test
    pub fn mono_to_stereo_interleaved(&mut self) -> RBoxErr<()> {
        let mut buf: Vec<f32> = vec![];
        for sample in &self.samples {
            buf.push(sample * 0.5);
            buf.push(sample * 0.5);
        }
        self.len = buf.len() as u32;
        self.samples = buf;
        self.spec.channels = 2;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // these are here purely because i keep forgetting the upper/lower bounds
    const MAX_I32: i32 = 2147483647;
    const MIN_I32: i32 = -2147483648;
    const MAX_I24: i32 = 8388607;
    const MIN_I24: i32 = -8388608;
    const MAX_I16: i16 = 32767;
    const MIN_I16: i16 = -32768;

    mod f32_to_i32 {
        use super::*;
        #[test]
        fn max_val() {
            assert_eq!(MAX_I32, f32_to_i32(1.0_f32));
        }
        #[test]
        fn min_val() {
            assert_eq!(MIN_I32, f32_to_i32(-1.0_f32));
        }
        #[test]
        fn zero_val() {
            assert_eq!(0_i32, f32_to_i32(0.0_f32));
        }
        #[test]
        fn half_pos() {
            assert_eq!(1073741824, f32_to_i32(0.5_f32));
        }
        #[test]
        fn half_neg() {
            assert_eq!(-1073741824, f32_to_i32(-0.5_f32));
        }
    }

    mod f32_to_i24 {
        use super::*;
        #[test]
        fn max_val() {
            assert_eq!(MAX_I24, f32_to_i24(1.0_f32));
        }
        #[test]
        fn min_val() {
            assert_eq!(MIN_I24, f32_to_i24(-1.0_f32));
        }
        #[test]
        fn zero_val() {
            assert_eq!(0_i32, f32_to_i24(0.0_f32));
        }
        #[test]
        fn half_pos() {
            assert_eq!(4194304, f32_to_i24(0.5_f32));
        }
        #[test]
        fn half_neg() {
            assert_eq!(-4194304, f32_to_i24(-0.5_f32));
        }
    }
    mod f32_to_i16 {
        use super::*;
        #[test]
        fn max_val() {
            assert_eq!(MAX_I16, f32_to_i16(1.0_f32));
        }
        #[test]
        fn min_val() {
            assert_eq!(MIN_I16, f32_to_i16(-1.0_f32));
        }
        #[test]
        fn zero_val() {
            assert_eq!(0_i16, f32_to_i16(0.0_f32));
        }
        #[test]
        fn half_pos() {
            assert_eq!(16384, f32_to_i16(0.5_f32));
        }
        #[test]
        fn half_neg() {
            assert_eq!(-16384, f32_to_i16(-0.5_f32));
        }
    }

    mod i32_to_f32 {
        use super::*;
        #[test]
        fn max_val() {
            assert_eq!(1.0_f32, i32_to_f32(i32::MAX));
        }
        #[test]
        fn min_val() {
            assert_eq!(-1.0_f32, i32_to_f32(i32::MIN));
        }
        #[test]
        fn zero_val() {
            assert_eq!(0.0_f32, i32_to_f32(0_i32));
        }
        #[test]
        fn half_pos() {
            assert_eq!(0.5_f32, i32_to_f32(i32::MAX / 2));
        }
        #[test]
        fn half_neg() {
            assert_eq!(-0.5_f32, i32_to_f32(i32::MIN / 2));
        }
    }

    mod i24_to_f32 {
        use super::*;
        #[test]
        fn max_val() {
            assert_eq!(1.0_f32, i24_to_f32(MAX_I24));
        }
        #[test]
        fn min_val() {
            assert_eq!(-1.0_f32, i24_to_f32(MIN_I24));
        }
        #[test]
        fn zero_val() {
            assert_eq!(0.0_f32, i24_to_f32(0_i32));
        }
        #[test]
        fn half_pos() {
            assert_ne!(
                0.5_f32,
                i24_to_f32(MAX_I24 / 2),
                "half positive 24 bit should be slightly less than 0.5"
            );
        }
        #[test]
        fn half_neg() {
            assert_eq!(-0.5_f32, i24_to_f32(MIN_I24 / 2));
        }
    }

    mod i16_to_f32 {
        use super::*;
        #[test]
        fn max_val() {
            assert_eq!(1.0_f32, i16_to_f32(i16::MAX));
        }
        #[test]
        fn min_val() {
            assert_eq!(-1.0_f32, i16_to_f32(i16::MIN));
        }
        #[test]
        fn zero_val() {
            assert_eq!(0.0_f32, i16_to_f32(0_i16));
        }
        #[test]
        fn half_pos() {
            assert_ne!(
                0.5_f32,
                i16_to_f32(i16::MAX / 2),
                "half positive 16 bit should be slightly less than 0.5"
            );
        }
        #[test]
        fn half_neg() {
            assert_eq!(-0.5_f32, i16_to_f32(i16::MIN / 2),);
        }
    }
}
