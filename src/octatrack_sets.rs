//! Recursively scan through an Octatrack Set directory and build up listings of various entities.
//! 
//! Please note: this is not included in the `serde_octatrack` library crate because there
//! is no serialization or deserialization of raw binary data files from an Octatrack.
//! This is just _walking through directories and listing things_.

use log::debug;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

use crate::utils::SampleFilePair;

use serde_octatrack::{
    common::{FromFileAtPathBuf, RVoidError},
    projects::Project,
};

use crate::audio::wavfile::scan_dir_path_for_wavfiles;

/// Searching for audio 'sample' (`.wav` files only for now) within an Octatrack Set.

pub trait SearchForOctatrackSampleFilePair {
    /// Recursively search through a directory tree for audio 'samples' (`.wav` files).

    fn scan_dir_path_for_samples(dir_path: &PathBuf) -> RVoidError<Vec<SampleFilePair>> {
        let wav_file_paths: Vec<PathBuf> = scan_dir_path_for_wavfiles(&dir_path).unwrap();

        let ot_sample_files: Vec<SampleFilePair> = wav_file_paths
            .into_iter()
            .map(|fp| SampleFilePair::from_audio_pathbuf(&fp).unwrap())
            .collect();

        debug!("Audio samples and OT files: {:#?}", ot_sample_files);
        Ok(ot_sample_files)
    }
}

/// Octatrack Projects that are contained within an Octatrack Set.

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct OctatrackSetProject {
    /// Name of this Project (directory basename)
    pub name: String,

    /// Explicit path to this Audio Pool
    pub dirpath: PathBuf,

    /// 'Samples' which are members of this Project.
    pub sample_filepaths: Vec<SampleFilePair>,

    pub data: Project,
}

impl SearchForOctatrackSampleFilePair for OctatrackSetProject {}

impl OctatrackSetProject {
    /// Create a new `OctatrackSetProject` from the dirpath `PathBuf`.

    pub fn from_pathbuf(dirpath: &PathBuf) -> RVoidError<Self> {
        // TODO: Handle looking for .work / .strd
        if !dirpath.is_dir() {
            return Err(());
        }

        Ok(Self {
            name: dirpath.file_name().unwrap().to_str().unwrap().to_string(),
            dirpath: dirpath.clone(),
            // samples: scan_dir_for_ot_files(path).unwrap_or(Vec::new()).clone()
            sample_filepaths: Self::scan_dir_path_for_samples(&dirpath).unwrap(),
            data: Project::from_pathbuf(dirpath.join("project.work")).unwrap(),
        })
    }
}

/// An Audio Pool from some Octatrack Set.

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct OctatrackSetAudioPool {
    // TODO: this should always be AUDIO ...
    /// Name of this Audio Pool (directory basename)
    pub name: String,

    /// Explicit path to this Audio Pool
    pub path: PathBuf,

    // TODO: There can be zero samples!
    /// 'Samples' which are members of this Audio Pool.
    pub samples: Vec<SampleFilePair>,
}

impl SearchForOctatrackSampleFilePair for OctatrackSetAudioPool {}

impl OctatrackSetAudioPool {
    /// Create a new `OctatrackSetAudioPool` from a `PathBuf`.
    /// **NOTE**: the `PathBuf` must point to the correct directory.

    pub fn from_pathbuf(path: &PathBuf) -> RVoidError<Self> {
        Ok(Self {
            name: path.file_name().unwrap().to_str().unwrap().to_string(),
            path: path.clone(),
            samples: Self::scan_dir_path_for_samples(&path).unwrap(),
        })
    }
}

/// An Octatrack 'Set'. Each 'Set' contains multiple Octatrack 'Project's and a single 'Audio Pool'.

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct OctatrackSet {
    /// Name of this set (the directory basename).
    pub name: String,

    /// Explicit path of this Set on a CF card.
    pub path: PathBuf,

    /// The 'Audio Pool' for this Set.
    pub audio_pool: OctatrackSetAudioPool,

    /// Projects which are members of this Set.
    pub projects: Vec<OctatrackSetProject>,
}

impl OctatrackSet {
    // TODO: Rename this?

    /// Create a collection of `OctatrackSet`s by recursively
    /// searching through a directory tree, starting at a given `PathBuf`

    pub fn from_cfcard_pathbuf(path: &PathBuf) -> RVoidError<Vec<OctatrackSet>> {
        let set_paths_iter: _ = WalkDir::new(path)
            .sort_by_file_name()
            .max_depth(1)
            .min_depth(1)
            .into_iter()
            .filter_entry(|e: &DirEntry| {
                e.file_type().is_dir() && !e.file_name().to_str().unwrap_or(".").starts_with(".")
            });

        let ot_sets: Vec<OctatrackSet> = set_paths_iter
            .map(|e: Result<DirEntry, walkdir::Error>| {
                let unwrapped = e.unwrap();
                let ot_set_path = unwrapped.path().to_path_buf();
                let ot_set = OctatrackSet::from_pathbuf(&ot_set_path);
                let unwrapped_set = ot_set.unwrap();
                unwrapped_set
            })
            .filter(|o: &Option<OctatrackSet>| !o.is_none())
            .map(|o: Option<OctatrackSet>| o.unwrap())
            .collect();
        Ok(ot_sets)
    }

    /// For a given `PathBuf`, find relevant Octatrack directories and files.

    pub fn from_pathbuf(path: &PathBuf) -> RVoidError<Option<OctatrackSet>> {
        let audio_pool_path = path.join("AUDIO");

        // if AUDIO doesn't exist then this is not a set
        // it's some other random directory like 'System Volume Information'

        if !audio_pool_path.exists() {
            return Ok(None);
        }

        let audio_pool = OctatrackSetAudioPool::from_pathbuf(&audio_pool_path).unwrap();

        let project_paths_iter: _ = WalkDir::new(path)
            .sort_by_file_name()
            .max_depth(1)
            .min_depth(1)
            .into_iter()
            .filter_entry(|e: &DirEntry| e.file_type().is_dir() && e.file_name() != "AUDIO");

        let projects: Vec<OctatrackSetProject> = project_paths_iter
            .into_iter()
            .map(|d| {
                let unwrapped = d.unwrap();
                let project_path = unwrapped.path().to_path_buf();
                let project = OctatrackSetProject::from_pathbuf(&project_path);
                project.unwrap()
            })
            .collect();

        Ok(Some(OctatrackSet {
            audio_pool,
            projects,
            name: path.file_name().unwrap().to_str().unwrap().to_string(),
            path: path.clone(),
        }))
    }
}
