//! Reading and Writing .wav files.

use std::{
    error::Error,
    path::PathBuf,
    io::BufReader,
    fs::File,
};
use hound::{
    self,
    WavReader,
    WavSpec,
};
use walkdir::{
    DirEntry,
    WalkDir,
};
use log::{
    debug,
    info,
    warn,
    error,
};

use crate::constants::OCTATRACK_COMPATIBLE_HOUND_WAVSPECS;


/// Chain together a wav sample vector into individual wav file(s).
///
/// Each individual output can have a maximum of 64 samples,
/// so results are batched up with a max size of 64.
// TODO: this needs to return a hashmap so the yaml chain generator can  
//       read the underlying sample information for a batch.
// TODO: Split this up into two functions --> wavfile_vec_to_batch64 and wavfile_batch64_to_wavfile
// TODO: Looks like there's a new struct, or a new datatype there...
pub fn chain_wavfiles_64_batch(wavfiles: &Vec<WavFile>) -> Result<Vec<(WavFile, Vec<WavFile>)>, ()> {

    let originals: Vec<WavFile> = wavfiles.clone();
    let mut slice_vecs: Vec<Vec<WavFile>> = vec![];

    let vec_mod_length = wavfiles.len().div_euclid(64);
    
    for i in 0..(vec_mod_length+1) {
        let (start, mut end) = (i * 64, (i * 64) + 64);

        if end > originals.len() {
            end = originals.len();
        };
        let mut s: Vec<WavFile> = Vec::with_capacity(end - start);

        for o in &originals[start..end] {s.push(o.clone());};
        slice_vecs.push(s);
    };
    
    let mut chains : Vec<(WavFile, Vec<WavFile>)> = vec![];
    for slice_vec in slice_vecs {

        let mut single_chain_wav: WavFile = slice_vec[0].clone();

        for wavfile in slice_vec[1..].into_iter() {
            for s in &wavfile.samples {
                single_chain_wav.samples.push(*s);
            };
            single_chain_wav.len += wavfile.len;
        };
        chains.push((single_chain_wav, slice_vec));
    };

    Ok(chains)
}


/// Representation of a wav audio file

#[derive(PartialEq, Debug, Clone)]
pub struct WavFile {
    /// hound wav file specification struct
    pub spec: WavSpec,
    /// length of the wav file data (number of samples)
    pub len: u32,
    /// instantaneous samples of the audio
    pub samples: Vec<i32>,  // cannot use Copy trait
}

impl WavFile {

    /// Write samples to a WAV file.

    pub fn to_file(&self, wav_file_path: &PathBuf) -> Result<(), Box<dyn Error>> {
        let mut writer = hound::WavWriter
            ::create(wav_file_path, self.spec)
            .unwrap()
            ;

        for sample in &self.samples {
            let _res = writer.write_sample(sample.clone()).unwrap();
        };

        Ok(())
    }

    /// Read samples, specficiation etc. frm a WAV file.

    pub fn from_file(wav_file_path: &PathBuf) -> Result<WavFile, Box<dyn Error>> {
        let mut reader = WavFile::open(&wav_file_path).unwrap();
        let spec = WavFile::read_wav_spec(&mut reader).unwrap();
        let samples = WavFile::read_wav_samples(&mut reader).unwrap();

        Ok(
            WavFile {
                samples: samples.clone(),
                len: samples.len() as u32 / spec.channels as u32,
                spec,
            }
        )
    }

    /// Open a wav file into a read buffer

    fn open(path: &PathBuf) -> Result<WavReader<BufReader<File>>, hound::Error> {
        hound::WavReader::open(path)
    }

    /// Read the hound WavSpec for an opened wav file buffer

    fn read_wav_spec(reader: &mut WavReader<BufReader<File>>) -> Result<WavSpec, Box<dyn Error>> {
        let spec = hound::WavReader::spec(reader);
        Ok(spec)
    }

    /// Write samples to an open and writeable file buffer.

    fn read_wav_samples(reader: &mut WavReader<BufReader<File>>) -> Result<Vec<i32>, Box<dyn Error>> {

        let samples_iter = reader.samples::<i32>();

        let mut samples: Vec<i32> = Vec::new();
        for sample in samples_iter {
            samples.push(sample.unwrap());
        }
        Ok(samples)
    }
}


/// A filter for walkdir rescursive search: include directories

fn walkdir_filter_is_dir(entry: &DirEntry) -> bool {
    entry.file_type().is_dir()
}

fn walkdir_filter_is_not_hidden(entry: &DirEntry) -> bool {
    ! entry.file_name().to_string_lossy().starts_with(".")
}


/// A filter for walkdir rescursive search: only files that have the 'wav' file extension

fn walkdir_filter_is_compat_wav(entry: &DirEntry) -> bool {

    let is_wav = entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with("wav"))
        .unwrap_or(false)
    ;

    if ! is_wav {return false;};

    let wav_path = entry.path().to_path_buf();
    
    debug!("Checking WAV file: {:#?}", wav_path);

    let wav_reader_res = WavFile::open(&wav_path);
    if wav_reader_res.is_err() {
        error!("Could not read WAV file: wavfile={:#?} error={:#?}", &wav_path, wav_reader_res.err().unwrap().to_string());
        return false
    }

    let wav_spec_res = WavFile::read_wav_spec(&mut wav_reader_res.unwrap());
    if wav_spec_res.is_err() {
        error!("Could not read WAV file WavSpec: wavfile={:#?} error={:#?}", &wav_path, wav_spec_res.err().unwrap().to_string());
        return false
    }

    OCTATRACK_COMPATIBLE_HOUND_WAVSPECS.contains(&wav_spec_res.unwrap().clone())

}

/// Recursively search for WAV audio files for a given directory tree.

pub fn scan_dir_path_for_wavfiles(dir_path: &PathBuf) -> Result<Vec<PathBuf>, ()> {

    info!(
        "Recursively searching for Octatrack compatible WAV files: dirpath={:1?}", 
        dir_path,
    );

    let paths_iter: _ = WalkDir::new(dir_path)
        .sort_by_file_name()
        .min_depth(1)
        .into_iter()
        .filter_entry(
            |e| 
            // ignore temporary files on linux
            walkdir_filter_is_not_hidden(e) 
            && {
                // needed to ensure we recurse throughout the dir tree
                // means we need to exclude them in the vec push later
                walkdir_filter_is_dir(e)
                || walkdir_filter_is_compat_wav(e) 
            }
        )
    ;

    let mut fpaths: Vec<PathBuf> = Vec::new();
    for entry in paths_iter {
        let unwrapped = entry.unwrap();
        let fpath= unwrapped.path().to_path_buf();

        // exclude directories
        if ! unwrapped.file_type().is_dir() {
            fpaths.push(fpath);
        };
    };

    info!(
        "Found Octatrack compatible WAV files: dirpath={:1?} found={:2?}",
        dir_path,
        fpaths.len(),
    );

    Ok(fpaths)


}
