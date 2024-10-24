//! Recursively scan through an Octatrack Set directory.

use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};
use log::{error, info, warn, debug};
use serde::{Serialize, Deserialize};
use crate::audio::wavfile::scan_dir_path_for_wavfiles;
use crate::octatrack::samples::OctatrackSampleFilePair;

/// Searching for audio 'sample' (`.wav` files only for now) within an Octatrack Set.

pub trait SearchForOctatrackSampleFilePair {

    /// Recursively search through a directory tree for audio 'samples' (`.wav` files).

    fn scan_dir_path_for_samples(dir_path: &PathBuf) -> Result<Vec<OctatrackSampleFilePair>, ()> {

        let wav_file_paths: Vec<PathBuf> = scan_dir_path_for_wavfiles(&dir_path).unwrap();
        let mut ot_sample_files: Vec<OctatrackSampleFilePair> = Vec::new();

        for wav_file_path in wav_file_paths {

            // TODO: optimise this?
            let mut ot_file_path = wav_file_path.clone();
            ot_file_path.set_extension("ot");

            let mut ot_file_pathbuf = Some(ot_file_path.clone());
            if ! ot_file_path.exists() {ot_file_pathbuf = None};

            let sample = OctatrackSampleFilePair {
                name: wav_file_path.file_name().unwrap().to_str().unwrap().to_string().clone(),
                audio_path: wav_file_path.clone(),
                otfile_path: ot_file_pathbuf.clone(),
            };

            ot_sample_files.push(sample);
        };

        debug!("Audio samples and OT files: {:#?}", ot_sample_files);
        Ok(ot_sample_files)

    }

}

/// Octatrack Projects that are contained within an Octatrack Set.

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct OctatrackSetProjects {

    /// Name of this Project (directory basename)
    pub name: String,

    /// Explicit path to this Audio Pool
    pub path: PathBuf,

    /// 'Samples' which are members of this Project.
    pub samples: Vec<OctatrackSampleFilePair>
}

impl SearchForOctatrackSampleFilePair for OctatrackSetProjects {}

impl OctatrackSetProjects {

    /// Create a new `OctatrackSetProjects` from a `PathBuf`.
    /// **NOTE**: the `PathBuf` must point to the correct directory.

    pub fn from_pathbuf(path: &PathBuf) -> Result<OctatrackSetProjects, ()> {

        Ok(
            OctatrackSetProjects {
                name: path.file_name().unwrap().to_str().unwrap().to_string(),
                path: path.clone(),
                // samples: scan_dir_for_ot_files(path).unwrap_or(Vec::new()).clone()
                samples: OctatrackSetProjects::scan_dir_path_for_samples(&path).unwrap()
            }
        )
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
    pub samples: Vec<OctatrackSampleFilePair>
}

impl SearchForOctatrackSampleFilePair for OctatrackSetAudioPool {}

impl OctatrackSetAudioPool {

    /// Create a new `OctatrackSetAudioPool` from a `PathBuf`.
    /// **NOTE**: the `PathBuf` must point to the correct directory.

    pub fn from_pathbuf(path: &PathBuf) -> Result<OctatrackSetAudioPool, ()> {
        Ok(
            OctatrackSetAudioPool {
                name: path.file_name().unwrap().to_str().unwrap().to_string(),
                path: path.clone(),
                samples: OctatrackSetAudioPool::scan_dir_path_for_samples(&path).unwrap()
            }
        )
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
    pub projects: Vec<OctatrackSetProjects>,
}

impl OctatrackSet {

    // TODO: Rename this?

    /// Create a collection of `OctatrackSet`s by recursively 
    /// searching through a directory tree, starting at a given `PathBuf` 

    pub fn from_cfcard_pathbuf(path: &PathBuf) -> Result<Vec<OctatrackSet>, ()> {

        let set_paths_iter: _ = WalkDir::new(path)
            .sort_by_file_name()
            .max_depth(1)
            .min_depth(1)
            .into_iter()
            .filter_entry(
                |e: &DirEntry| {
                    e.file_type().is_dir()
                    && !e.file_name().to_str().unwrap_or(".").starts_with(".")
                }
            )
        ;

        let mut ot_sets: Vec<OctatrackSet> = Vec::new();
        for entry in set_paths_iter {
            let unwrapped = entry.unwrap();
            let ot_set_path = unwrapped.path().to_path_buf();
            let ot_set = OctatrackSet::from_pathbuf(&ot_set_path);
            let unwrapped_set = ot_set.unwrap();

            if unwrapped_set != None {
                ot_sets.push(unwrapped_set.unwrap());
            }

        }

        Ok(ot_sets)

    }

    /// For a given `PathBuf`, find relevant Octatrack directories and files.

    pub fn from_pathbuf(path: &PathBuf) -> Result<Option<OctatrackSet>, ()>  {

        let audio_pool_path = path.join("AUDIO");

        // if AUDIO doesn't exist then this is not a set
        // it's some other random directory like 'System Volume Information'

        if ! audio_pool_path.exists() {
            return Ok(None);
        }

        let audio_pool = OctatrackSetAudioPool
            ::from_pathbuf(&audio_pool_path)
            .unwrap()
        ;

        let project_paths_iter: _ = WalkDir::new(path)
            .sort_by_file_name()
            .max_depth(1)
            .min_depth(1)
            .into_iter()
            .filter_entry(
                |e: &DirEntry| {
                    e.file_type().is_dir()
                    && e.file_name() != "AUDIO"
                }
            )
        ;

        let mut projects: Vec<OctatrackSetProjects> = Vec::new();
        for entry in project_paths_iter {
            let unwrapped = entry.unwrap();
            let project_path = unwrapped.path().to_path_buf();
            let project = OctatrackSetProjects::from_pathbuf(&project_path);
            projects.push(project.unwrap());
        }

        Ok(
            Some(
                OctatrackSet {
                    audio_pool,
                    projects,
                    name: path.file_name().unwrap().to_str().unwrap().to_string(),
                    path: path.clone(),
                }
            )
        )

    }

}
