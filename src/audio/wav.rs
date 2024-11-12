//! Reading and Writing .wav files.
//!
//! **TODO**: Need to move this out into the binary crate.

use hound::{self, WavReader, WavSpec};
use log::{debug, trace};
use std::{error::Error, fs::File, io::BufReader, path::PathBuf};

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
    /// Write samples to a WAV file.
    pub fn to_file(&self, path: &PathBuf) -> Result<(), Box<dyn Error>> {
        trace!("Writing WAV data to file: path={path:#?}");
        let mut writer = hound::WavWriter::create(path, self.spec).unwrap();

        trace!("Writing WAV samples: path={path:#?}");
        for sample in &self.samples {
            let _res = writer.write_sample(sample.clone()).unwrap();
        }

        debug!("Wrote new WAV file: path={path:#?}");
        Ok(())
    }

    /// Read samples, specficiation etc. from a WAV file.
    pub fn from_file(path: PathBuf) -> Result<WavFile, Box<dyn Error>> {
        trace!("Reading WAV file from path: {path:#?}");
        let mut reader = WavFile::open(&path).unwrap();

        trace!("Reading WAV Spec: path={path:#?}");
        let spec = WavFile::read_spec(&mut reader).unwrap();

        trace!("Reading WAV Samples: path={path:#?}");
        let samples = WavFile::read_samples(&mut reader).unwrap();

        debug!("Read new WAV file: path={path:#?}");
        Ok(WavFile {
            file_path: path,
            samples: samples.clone(),
            len: samples.len() as u32 / spec.channels as u32,
            spec,
        })
    }

    /// Open a wav file into a read buffer
    pub fn open(path: &PathBuf) -> Result<WavReader<BufReader<File>>, hound::Error> {
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
        let samples_iter = reader.samples::<f32>();

        trace!("Collecting samples from iterator.");
        let samples: Vec<f32> = samples_iter.map(|x| x.unwrap()).collect();

        debug!("Read WAV file sample data.");
        Ok(samples)
    }
}
