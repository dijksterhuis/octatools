use aifc::{AifcReadInfo, AifcReader};
use log::{debug, trace};
use std::{error::Error, fs::File, io::BufReader, path::PathBuf};

// simple conversion algorithm to convert Sample to f32
// source: https://github.com/karip/aifc/blob/main/examples/aifc-tinyaudio/src/main.rs
#[allow(dead_code)]
fn sample_to_f32(s: aifc::Sample) -> f32 {
    match s {
        aifc::Sample::U8(s) => s as f32 / (1u32 << 7) as f32 - 1.0,
        aifc::Sample::I8(s) => s as f32 / (1u32 << 7) as f32,
        aifc::Sample::I16(s) => s as f32 / (1u32 << 15) as f32,
        aifc::Sample::I24(s) => s as f32 / (1u32 << 23) as f32,
        aifc::Sample::I32(s) => s as f32 / (1u32 << 31) as f32,
        aifc::Sample::F32(s) => s,
        aifc::Sample::F64(s) => s as f32,
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct AiffFile {
    /// specification struct for the Aiff file.
    pub spec: AifcReadInfo,

    /// Number of audio samples in the Aiff file.
    pub len: u32,

    /// Audio samples
    pub samples: Vec<f32>, // cannot use Copy trait

    /// File path of the Aiff file
    pub file_path: PathBuf,
}

impl AiffFile {
    /// Write samples to a WAV file.
    #[allow(dead_code)]
    pub fn to_file(&self, path: &PathBuf) -> Result<(), Box<dyn Error>> {
        trace!("Writing to new AIFF file: path={path:#?}");
        let f = std::fs::File::create(path)?;
        let mut stream = std::io::BufWriter::new(f);

        trace!("Creating new AIFF Write Spec: path={path:#?}");
        let write_format = aifc::AifcWriteInfo {
            file_format: self.spec.file_format,
            channels: self.spec.channels,
            sample_rate: self.spec.sample_rate,
            sample_format: aifc::SampleFormat::F32,
        };

        trace!("Creating new AIFF writer: path={path:#?}");
        let mut writer =
            aifc::AifcWriter::new(&mut stream, &write_format).expect("Can't create writer");

        trace!("Writing samples: path={path:#?}");
        writer
            .write_samples_f32(&self.samples)
            .expect("Can't write samples");

        writer.finalize().expect("Can't finalize");
        debug!("Wrote to new AIFF file: path={path:#?}");

        Ok(())
    }

    /// Read samples, specficiation etc. from an AIFF file.
    #[allow(dead_code)]
    pub fn from_file(path: PathBuf) -> Result<AiffFile, Box<dyn Error>> {
        trace!("Reading AIFF data from file: path={path:#?}");
        let mut reader = AiffFile::open(&path)?;

        trace!("Reading AIFF info spec: path={path:#?}");
        let info = AiffFile::read_spec(&mut reader)?;

        trace!("Reading AIFF samples: path={path:#?}");
        let samples = AiffFile::read_samples(&mut reader)?;

        trace!("Getting AIFF sample length: path={path:#?}");
        let length = info.sample_len.unwrap() as u32;

        debug!("Read AIFF file: path={path:#?}");
        Ok(AiffFile {
            spec: info,
            len: length,
            samples,
            file_path: path,
        })
    }

    /// Open an AIFF file into a read buffer
    pub fn open(path: &PathBuf) -> Result<AifcReader<BufReader<File>>, Box<dyn Error>> {
        let bufreader = BufReader::new(File::open(path)?);
        let reader = aifc::AifcReader::new(bufreader).expect("Can't create reader");
        Ok(reader)
    }

    /// Read the hound WavSpec for an opened wav file buffer
    pub fn read_spec(
        reader: &mut AifcReader<BufReader<File>>,
    ) -> Result<AifcReadInfo, Box<dyn Error>> {
        trace!("Reading AIFF spec from reader.");
        let info = reader.read_info().expect("Can't read header");
        Ok(info)
    }

    /// Write samples to an open and writeable file buffer.
    fn read_samples(reader: &mut AifcReader<BufReader<File>>) -> Result<Vec<f32>, Box<dyn Error>> {
        trace!("Reading AIFF samples from reader.");
        let samples: Vec<f32> = reader
            .samples()
            .expect("Can't iterate samples")
            .map(|x| sample_to_f32(x.unwrap()))
            .collect();

        Ok(samples)
    }
}
