//! Module to recursively scan through an Octatrack Set directory and build up listings of various entities.
//!
//! Please note: this is not included in the `serde_octatrack` library crate because there
//! is no serialization or deserialization of raw binary data files from an Octatrack.
//! This is just _walking through directories and listing things_.

use log::debug;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

use crate::common::RBoxErr;
use crate::utils::SampleFilePair;

use crate::audio::utils::scan_dir_path_for_audio_files;

/// Searching for audio 'sample' (`.wav` files only for now) within an Octatrack Set.
pub trait SearchForOctatrackSampleFilePair {
    /// Recursively search through a directory tree for audio 'samples' (`.wav` files).
    fn scan_dir_path_for_samples(dir_path: &PathBuf) -> RBoxErr<Vec<SampleFilePair>> {
        let wav_file_paths: Vec<PathBuf> = scan_dir_path_for_audio_files(dir_path)?;

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
    pub samples: Vec<SampleFilePair>,

    /// Project data files
    pub projects: Vec<PathBuf>,

    /// Arrangement data files
    pub arrangements: Vec<PathBuf>,

    /// Bank data files
    pub banks: Vec<PathBuf>,

    /// Marker data files
    pub markers: Vec<PathBuf>,

}

impl SearchForOctatrackSampleFilePair for OctatrackSetProject {}

impl OctatrackSetProject {
    /// Create a new `OctatrackSetProject` from the dirpath `PathBuf`.
    pub fn from_pathbuf(dirpath: &PathBuf) -> RBoxErr<Self> {
        // TODO: Handle looking for .work / .strd
        if !dirpath.is_dir() {
            return Err(Box::new(crate::common::OctatoolErrors::PathNotADirectory));
        }

        let projects: Vec<PathBuf> = WalkDir::new(dirpath)
            .sort_by_file_name()
            .min_depth(1)
            .into_iter()
            .filter_entry(|e| {
                e.file_name().to_string_lossy().starts_with("project")
            })
            .map(|x| x.unwrap())
            .map(|x| x.path().to_path_buf())
            .collect();

        let banks: Vec<PathBuf> = WalkDir::new(dirpath)
            .sort_by_file_name()
            .min_depth(1)
            .into_iter()
            .filter_entry(|e| {
                e.file_name().to_string_lossy().starts_with("bank")
            })
            .map(|x| x.unwrap())
            .map(|x| x.path().to_path_buf())
            .collect();

        let arrangements: Vec<PathBuf> = WalkDir::new(dirpath)
            .sort_by_file_name()
            .min_depth(1)
            .into_iter()
            .filter_entry(|e| {
                e.file_name().to_string_lossy().starts_with("arr")
            })
            .map(|x| x.unwrap())
            .map(|x| x.path().to_path_buf())
            .collect();

        let markers: Vec<PathBuf> = WalkDir::new(dirpath)
            .sort_by_file_name()
            .min_depth(1)
            .into_iter()
            .filter_entry(|e| {
                e.file_name().to_string_lossy().starts_with("markers")
            })
            .map(|x| x.unwrap())
            .map(|x| x.path().to_path_buf())
            .collect();


        Ok(Self {
            name: dirpath.file_name().unwrap().to_str().unwrap().to_string(),
            dirpath: dirpath.clone(),
            samples: Self::scan_dir_path_for_samples(dirpath).unwrap(),
            projects,
            banks,
            arrangements,
            markers,

        })
    }
}

/// An Audio Pool from some Octatrack Set.

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct OctatrackSetAudioPool {
    /// Name of this Audio Pool (directory basename). Should always be `'AUDIO'`.
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
    pub fn from_pathbuf(path: &PathBuf) -> RBoxErr<Self> {
        Ok(Self {
            name: path.file_name().unwrap().to_str().unwrap().to_string(),
            path: path.clone(),
            samples: Self::scan_dir_path_for_samples(path).unwrap(),
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
    pub fn from_cfcard_pathbuf(path: &Path) -> RBoxErr<Vec<Self>> {
        let ot_sets: Vec<Self> = WalkDir::new(path)
            .sort_by_file_name()
            .max_depth(1)
            .min_depth(1)
            .into_iter()
            .filter_entry(|e: &DirEntry| {
                e.file_type().is_dir() && !e.file_name().to_str().unwrap_or(".").starts_with('.')
            })
            .map(|e: Result<DirEntry, walkdir::Error>| {
                let unwrapped = e.unwrap();
                let ot_set_path = unwrapped.path().to_path_buf();
                let ot_set = Self::from_pathbuf(&ot_set_path);

                ot_set.unwrap()
            })
            .filter(|o: &Option<Self>| !o.is_none())
            .map(|o: Option<Self>| o.unwrap())
            .collect();

        Ok(ot_sets)
    }

    /// For a given `PathBuf` to a Set directory, gather information about Octatrack Projects and the Audio Pool.
    pub fn from_pathbuf(path: &PathBuf) -> RBoxErr<Option<OctatrackSet>> {
        if !path.exists() {
            panic!("Path does not exist: path={path:#?}");
        }
        if !path.is_dir() {
            panic!("Path is not a directory: path={path:#?}");
        }

        let audio_pool_path = path.join("AUDIO");
        if !audio_pool_path.exists() {
            panic!("Path is not a Set (no 'AUDIO' sub-directory found): path={path:#?}");
        }

        let audio_pool = OctatrackSetAudioPool::from_pathbuf(&audio_pool_path).expect(
            "Could not gather information about Set Audio Pool: audioPoolPath={audio_pool_path:#?}",
        );

        let projects: Vec<OctatrackSetProject> = WalkDir::new(path)
            .sort_by_file_name()
            .max_depth(1)
            .min_depth(1)
            .into_iter()
            .filter_entry(|e: &DirEntry| e.file_type().is_dir() && e.file_name() != "AUDIO")
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
