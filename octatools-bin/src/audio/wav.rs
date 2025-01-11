//! Reading and Writing .wav files.

use crate::RBoxErr;
use hound::{self, WavReader, WavSpec};
use log::{debug, trace};
use std::{
    error::Error,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

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

impl WavFile {
    /// Open a wav file into a read buffer
    pub fn open(path: &Path) -> Result<WavReader<BufReader<File>>, hound::Error> {
        trace!("Opening WAV file: path={path:#?}");
        hound::WavReader::open(path)
    }

    /// Read the hound WavSpec for an opened wav file buffer
    pub fn read_spec(reader: &mut WavReader<BufReader<File>>) -> Result<WavSpec, Box<dyn Error>> {
        trace!("Reading WAV reader spec.");
        let spec = hound::WavReader::spec(reader);
        Ok(spec)
    }

    /// Write samples to an open and writeable file buffer.
    fn read_samples(reader: &mut WavReader<BufReader<File>>) -> Result<Vec<f32>, Box<dyn Error>> {
        trace!("Reading WAV samples into iterator.");
        let samples_iter = reader.samples::<i32>();

        trace!("Collecting samples from iterator.");
        let samples: Vec<f32> = samples_iter
            .map(
                // conversion to f32
                |x| (x.unwrap() / i32::MAX) as f32,
            )
            .collect();

        debug!("Read WAV file sample data.");
        Ok(samples)
    }

    /// Crete a new struct by reading a file located at `path`.
    pub fn from_path(path: &Path) -> RBoxErr<Self> {
        trace!("Reading WAV file from path: {path:#?}");
        let mut reader = WavFile::open(path)?;

        trace!("Reading WAV Spec: path={path:#?}");
        let spec = WavFile::read_spec(&mut reader)?;

        println!("spec: {:#?}", spec);

        trace!("Reading WAV Samples: path={path:#?}");
        let samples = WavFile::read_samples(&mut reader)?;

        debug!("Read new WAV file: path={path:#?}");
        Ok(WavFile {
            file_path: path.to_path_buf(),
            samples: samples.clone(),
            len: samples.len() as u32 / spec.channels as u32,
            spec,
        })
    }

    /// Crete a new file at the path from the current struct
    pub fn to_path(&self, path: &Path) -> RBoxErr<()> {
        trace!("Writing WAV data to file: path={path:#?}");
        let mut writer = hound::WavWriter::create(path, self.spec)?;

        let samples_i32: Vec<i32> = self
            .samples
            .iter()
            .map(
                // conversion to f32
                |x| (x * (i32::MAX as f32)) as i32,
            )
            .collect();

        trace!("Writing WAV samples: path={path:#?}");
        for sample in samples_i32 {
            writer.write_sample(sample)?;
        }

        debug!("Wrote new WAV file: path={path:#?}");
        Ok(())
    }
}
