use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};
use log::{error, info, warn, debug};

use crate::wavfile::scan_dir_path_for_wavfiles;


// samples can be stored either in the set's audio pool or in the project directory
#[derive(PartialEq, Debug, Clone)]
pub struct OctatrackSampleFiles {
    pub name: String,
    pub audio_path: PathBuf,
    pub otfile_path: Option<PathBuf>,
}

impl OctatrackSampleFiles {

    pub fn from_pathbufs(audio_fp: &PathBuf, ot_fp: &Option<PathBuf>) -> Result<OctatrackSampleFiles, ()> {

        Ok(
            OctatrackSampleFiles {
                name: audio_fp.file_stem().unwrap().to_str().unwrap().to_string(),
                audio_path: audio_fp.clone(),
                otfile_path: ot_fp.clone()
            }
        )
    }
}


pub trait ScanForSamples {
    fn scan_dir_path_for_samples(dir_path: &PathBuf) -> Result<Vec<OctatrackSampleFiles>, ()> {

        let wav_file_paths: Vec<PathBuf> = scan_dir_path_for_wavfiles(&dir_path).unwrap();

        let mut ot_sample_files: Vec<OctatrackSampleFiles> = Vec::new();
        for wav_file_path in wav_file_paths {

            // TODO: optimise this?
            let mut ot_file_path = wav_file_path.clone();
            ot_file_path.set_extension("ot");

            let mut ot_file_pathbuf = Some(ot_file_path.clone());
            if ! ot_file_path.exists() {ot_file_pathbuf = None};

            let sample = OctatrackSampleFiles {
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


#[derive(PartialEq, Debug, Clone)]
pub struct OctatrackSetProjects {
    pub name: String,
    pub path: PathBuf,
    pub samples: Vec<OctatrackSampleFiles>
}

impl ScanForSamples for OctatrackSetProjects {}

impl OctatrackSetProjects {
    pub fn from_dir(path: &PathBuf) -> Result<OctatrackSetProjects, ()> {

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


#[derive(PartialEq, Debug, Clone)]
pub struct OctatrackSetAudioPool {
    pub name: String,
    pub path: PathBuf,
    // may not be any samples
    pub samples: Vec<OctatrackSampleFiles>
}

impl ScanForSamples for OctatrackSetAudioPool {}

impl OctatrackSetAudioPool {
    pub fn from_pathbuf(path: &PathBuf) -> Result<OctatrackSetAudioPool, ()> {
        Ok(
            OctatrackSetAudioPool {
                name: path.file_name().unwrap().to_str().unwrap().to_string(),
                path: path.clone(),
                // samples: scan_dir_for_ot_files(path).unwrap_or(Vec::new()).clone()
                samples: OctatrackSetAudioPool::scan_dir_path_for_samples(&path).unwrap()
            }
        )
    }
}


#[derive(PartialEq, Debug, Clone)]
pub struct OctatrackSet {
    pub name: String,
    pub path: PathBuf,
    pub audio_pool: OctatrackSetAudioPool,
    pub projects: Vec<OctatrackSetProjects>,
}

impl OctatrackSet {

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
            let project = OctatrackSetProjects::from_dir(&project_path);
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




// WHY AM I USING THIS?
/*
fn scan_dir_for_ot_files(dirpath: &PathBuf) -> Result<Vec<OctatrackSampleFiles>, ()> {

    let audio_file_paths = WalkDir
        ::new(dirpath)
        .sort_by_file_name()
        .max_depth(1)
        .min_depth(1)
        .into_iter()
        .filter_entry(
            |e: &DirEntry| {
                
                let f = e
                    .file_name()
                    .to_str()
                    .unwrap_or(".strd")
                    ;

                e.file_type().is_file()
                // TODO: probably better to check for compatible audio file types
                && !f.ends_with(".strd")
                && !f.ends_with(".work")
                && !f.ends_with(".ot")
                && !f.ends_with(".txt")  // OctaEdit sample reports
            }
        )
    ;

    let mut samples: Vec<OctatrackSampleFiles> = Vec::new();
    for ent in audio_file_paths.into_iter() {

        // *sighs*
        let unwrapped = ent.unwrap();
        let cloned = unwrapped.clone();

        let file_stem = unwrapped
            .into_path()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
        ;

        let audio_fp = cloned.into_path();

        let mut ot_file_path = audio_fp
            .parent()
            .unwrap()
            .join(
                PathBuf::from(&file_stem)
            )
        ;
        ot_file_path.set_extension("ot");

        let ot_fp = Some(ot_file_path).filter(|f| f.exists());

        // TODO: THIS IS WRONG
        let sample = OctatrackSampleFiles
            ::from_pathbufs(
                &audio_fp,
                &ot_fp
            )
            .unwrap();

        samples.push(sample);
    }

    Ok(samples)
}
*/