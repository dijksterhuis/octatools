use log::{info, trace};
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

use crate::audio::{aiff::AiffFile, wav::WavFile};
use serde_octatrack::{common::RBoxErr, constants::OCTATRACK_COMPATIBLE_AUDIO_SPECS};

/// A filter for walkdir rescursive search: include directories
fn direntry_is_dir(entry: &DirEntry) -> bool {
    entry.file_type().is_dir()
}

fn direntry_is_not_posix_hidden(entry: &DirEntry) -> bool {
    !entry.file_name().to_string_lossy().starts_with(".")
}

fn direntry_has_file_extension(entry: &DirEntry, v: &str) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with(v))
        .unwrap_or(false)
}

fn direntry_is_compat_wav(entry: &DirEntry) -> bool {
    trace!(
        "Testing whether DirEntry is a compatible WAV file: path={:#?}",
        entry.path()
    );
    file_path_is_compat_wav(&entry.path().to_path_buf()).unwrap_or(false)
}

fn direntry_is_compat_aiff(entry: &DirEntry) -> bool {
    trace!(
        "Testing whether DirEntry is a compatible AIFF file: path={:#?}",
        entry.path()
    );
    file_path_is_compat_aiff(&entry.path().to_path_buf()).unwrap_or(false)
}

/// A filter for walkdir rescursive search: only files that have the 'wav' file extension
fn file_path_is_compat_wav(path: &PathBuf) -> RBoxErr<bool> {
    trace!("Opening WAV file: path={path:#?}");
    let mut reader = WavFile::open(&path)?;

    trace!("Opening WAV spec: path={path:#?}");
    let spec = WavFile::read_spec(&mut reader)?;

    trace!("Creating serde_octatrack AudioSpec: path={path:#?}");
    let audio_spec = serde_octatrack::constants::AudioSpec {
        channels: spec.channels as u8,
        sample_rate: spec.sample_rate as u32,
        bit_depth: spec.bits_per_sample as u8,
    };

    trace!("Checking compatibility: spec={audio_spec:#?}");
    Ok(OCTATRACK_COMPATIBLE_AUDIO_SPECS.contains(&audio_spec))
}

/// A filter for walkdir rescursive search: only files that have the 'aiff' file extension
fn file_path_is_compat_aiff(path: &PathBuf) -> RBoxErr<bool> {
    trace!("Opening AIFF file: path={path:#?}");
    let mut reader = AiffFile::open(&path)?;

    trace!("Opening AIFF spec: path={path:#?}");
    let spec = AiffFile::read_spec(&mut reader)?;

    trace!("Creating serde_octatrack AudioSpec: path={path:#?}");
    let audio_spec = serde_octatrack::constants::AudioSpec {
        channels: spec.channels as u8,
        sample_rate: spec.sample_rate as u32,
        bit_depth: spec.comm_sample_size as u8,
    };

    trace!("Checking compatibility: spec={audio_spec:#?}");
    Ok(OCTATRACK_COMPATIBLE_AUDIO_SPECS.contains(&audio_spec))
}

/// Recursively search for WAV audio files for a given directory tree.
pub fn scan_dir_path_for_audio_files(dir_path: &PathBuf) -> RBoxErr<Vec<PathBuf>> {
    info!(
        "Recursively searching for Octatrack compatible audio files: dirpath={:1?}",
        dir_path,
    );

    let fpaths: Vec<PathBuf> = WalkDir::new(dir_path)
        .sort_by_file_name()
        .min_depth(1)
        .into_iter()
        .filter_entry(|e| {
            // ignore hidden files on *nix
            direntry_is_not_posix_hidden(e) && {
                // need to ensure we recurse through dirs in the tree
                // need to filter them out below (only want audio files)
                direntry_is_dir(e)
                    || { direntry_has_file_extension(&e, "wav") && direntry_is_compat_wav(&e) }
                    || { direntry_has_file_extension(&e, "aiff") && direntry_is_compat_aiff(&e) }
            }
        })
        .map(|x| x.unwrap())
        .filter(|x| !x.file_type().is_dir())
        .map(|x| x.path().to_path_buf())
        .collect();

    info!(
        "Found Octatrack compatible audio files: dirpath={:1?} found={:2?}",
        dir_path,
        fpaths.len(),
    );

    Ok(fpaths)
}
