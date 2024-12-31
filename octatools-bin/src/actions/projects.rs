use serde_octatrack::{
    projects::{slots::ProjectSampleSlot, Project},
    FromPath, ToPath, ToYamlFile,
};

use crate::audio::utils::scan_dir_path_for_audio_files;
use crate::{actions::load_from_yaml, RBoxErr};
use itertools::Itertools;
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Show deserialised representation of a Project for a given project file at `path`
pub fn show_project(path: &PathBuf) -> RBoxErr<()> {
    let b = Project::from_path(path).expect("Could not load project file");
    println!("{b:#?}");
    Ok(())
}

/// List all the sample slots within an Octatrack Project, given a path to a Project data file
pub fn list_project_sample_slots(path: &PathBuf) -> RBoxErr<()> {
    let project = Project::from_path(path).expect("Could not load project file");

    let slots = project
        .slots
        .iter()
        .sorted_by(|x, y| Ord::cmp(&x.slot_id, &y.slot_id));

    for slot in slots {
        println!("{:#?}", slot)
    }

    Ok(())
}

/// Copy sample files for project sample slots to the project set's audio pool directory,
/// updating the project sample slot location.
pub fn consolidate_sample_slots_to_audio_pool(project_file_path: &Path) -> RBoxErr<()> {
    let abs_project_fp = fs::canonicalize(project_file_path)?;

    let project_dir_path = abs_project_fp
        .parent()
        .unwrap_or_else(|| panic!("Cannot find project directory from project file path."));

    let audio_pool_path = project_dir_path
        .to_path_buf()
        .parent()
        .unwrap_or_else(|| panic!("Cannot find set directory from project file path."))
        .join("AUDIO");

    let mut project = Project::from_path(&abs_project_fp).expect("Could not load project file");

    let mut slots: Vec<ProjectSampleSlot> = project
        .slots
        .into_iter()
        .sorted_by(|x, y| Ord::cmp(&x.slot_id, &y.slot_id))
        .collect();

    for slot in slots.iter_mut() {
        // recording buffer slots can have an empty path field
        if slot.path != PathBuf::from("") {
            let audio_fname = slot
                .path
                .file_name()
                .unwrap_or_else(|| panic!("Could not resolve filename for audio file."))
                .to_str()
                .unwrap_or_else(|| panic!("Could not convert filename into string."))
                .to_string();

            let audio_fpath = project_dir_path
                .join(&slot.path)
                .to_path_buf()
                .canonicalize()
                .expect("Could not get abslute path for sample slot.");

            let new_audio_path = audio_pool_path.join(&audio_fname);

            let _ = std::fs::copy(&audio_fpath, &new_audio_path)?;

            let mut ot_filepath = audio_fpath.clone();
            ot_filepath.set_extension("ot");

            if ot_filepath.exists() {
                let mut new_otfile_path = audio_pool_path.join(&audio_fname);
                new_otfile_path.set_extension("ot");

                let _ = std::fs::copy(&ot_filepath, &new_otfile_path)?;
            }

            slot.path = new_audio_path;
        }
    }

    project.slots = slots;
    project.to_path(&abs_project_fp)?;

    Ok(())
}

/// Copy sample files for project sample slots to the project set's audio pool directory,
/// updating the project sample slot location.
pub fn consolidate_sample_slots_to_project_pool(project_file_path: &Path) -> RBoxErr<()> {
    let abs_project_fp = fs::canonicalize(project_file_path)?;

    println!("{:#?}", abs_project_fp);

    let project_dir_path = abs_project_fp
        .parent()
        .unwrap_or_else(|| panic!("Cannot find project directory from project file path."));

    println!("{:#?}", project_dir_path);

    let mut project = Project::from_path(&abs_project_fp).expect("Could not load project file");

    let mut slots: Vec<ProjectSampleSlot> = project
        .slots
        .into_iter()
        .sorted_by(|x, y| Ord::cmp(&x.slot_id, &y.slot_id))
        .collect();

    for slot in slots.iter_mut() {
        // recording buffer slots can have an empty path field
        if slot.path != PathBuf::from("") {
            let audio_fname = slot
                .path
                .file_name()
                .unwrap_or_else(|| panic!("Could not resolve filename for audio file."))
                .to_str()
                .unwrap_or_else(|| panic!("Could not convert filename into string."))
                .to_string();

            let audio_fpath = project_dir_path.join(&slot.path);
            println!("{:#?}", audio_fpath);
            // must be relative to project file!
            let new_audio_path = project_dir_path.join(&audio_fname);
            println!("{:#?}", audio_fpath);

            let _ = std::fs::copy(&audio_fpath, &new_audio_path)?;

            let mut ot_filepath = audio_fpath.clone();
            ot_filepath.set_extension("ot");

            if ot_filepath.exists() {
                let mut new_otfile_path = project_dir_path.join(&audio_fname);
                new_otfile_path.set_extension("ot");

                let _ = std::fs::copy(&ot_filepath, &new_otfile_path)?;
            }

            slot.path = new_audio_path;
        }
    }

    project.slots = slots;
    project.to_path(&abs_project_fp)?;

    Ok(())
}

/// Remove audio sample files from the project directory which
/// are not loaded in the project's samples slots.
/// No such feature exists for a set audio pool, as the set audio pool is
/// supposed to have a bunch of possible samples available which may not be in use.
pub fn purge_project_pool(project_file_path: &Path) -> RBoxErr<()> {
    let abs_project_fp = fs::canonicalize(project_file_path)?;

    let project_dir_path = abs_project_fp
        .parent()
        .unwrap_or_else(|| panic!("Cannot find project directory from project file path."));

    let project = Project::from_path(&abs_project_fp).expect("Could not load project file");

    let slots: Vec<ProjectSampleSlot> = project
        .slots
        .into_iter()
        .sorted_by(|x, y| Ord::cmp(&x.slot_id, &y.slot_id))
        .collect();

    let slot_paths: Vec<PathBuf> = slots
        .into_iter()
        .map(|x| {
            project_dir_path
                .join(x.path)
                .to_path_buf()
                .canonicalize()
                .expect("Could not get abslute path for sample slot.")
        })
        .collect();
    let samples: Vec<PathBuf> = scan_dir_path_for_audio_files(&project_dir_path.to_path_buf())?;

    for sample in samples {
        if !slot_paths.contains(&sample) {
            fs::remove_file(&sample)?;

            let mut ot_filepath = sample.clone();
            ot_filepath.set_extension("ot");

            if ot_filepath.exists() {
                fs::remove_file(&ot_filepath)?;
            }
        }
    }

    Ok(())
}

/// Load Project file data from a YAML file
pub fn load_project(yaml_path: &Path, outfile: &Path) -> RBoxErr<()> {
    load_from_yaml::<Project>(yaml_path, outfile)
}

/// Dump Project file data to a YAML file
pub fn dump_project(path: &Path, yaml_path: &Path) -> RBoxErr<()> {
    let b = Project::from_path(path).expect("Could not load project file");
    let _ = b.to_yaml(yaml_path);
    Ok(())
}

mod test {

    use super::*;

    #[test]
    fn test_show_ok() {
        let fp = PathBuf::from("data/tests/blank-project/project.work");
        let r = show_project(&fp);
        assert!(r.is_ok())
    }

    #[test]
    fn test_list_sample_slots_ok() {
        let fp = PathBuf::from("data/tests/blank-project/project.work");
        let r = list_project_sample_slots(&fp);
        assert!(r.is_ok())
    }

    #[test]
    fn test_load_project_ok() {
        let outfile = PathBuf::from("/tmp/octatools-actions-project-load-test-ok.work");
        let yaml = PathBuf::from("data/tests/projects/project.yaml");
        let r = load_project(&yaml, &outfile);

        let _ = std::fs::remove_file(&outfile);
        assert!(r.is_ok())
    }

    #[test]
    fn test_load_project_matches_blank() {
        let testfile = PathBuf::from("data/tests/projects/blank.work");
        let outfile = PathBuf::from("/tmp/octatools-actions-project-load-test-full.work");
        let yaml = PathBuf::from("data/tests/projects/project.yaml");

        let _ = load_project(&yaml, &outfile);

        let written = Project::from_path(&outfile).unwrap();
        let valid = Project::from_path(&testfile).unwrap();

        let _ = std::fs::remove_file(&outfile);
        assert_eq!(written, valid)
    }

    #[test]
    fn test_dump_project_ok() {
        let infile = PathBuf::from("data/tests/projects/blank.work");
        let outyaml = PathBuf::from("/tmp/project-test-dump-ok.yaml");
        let r = dump_project(&infile, &outyaml);

        let _ = std::fs::remove_file(&outyaml);
        assert!(r.is_ok())
    }

    fn make_sslot_mock_set_dir(base_path: &PathBuf) {
        let _ = fs::create_dir(base_path.join("test"));
        let _ = fs::create_dir(base_path.join("test/AUDIO"));
        let _ = fs::create_dir(base_path.join("test/PROJECT"));
        let _ = fs::copy(
            PathBuf::from("./data/tests/misc/test.wav"),
            base_path.join("test/AUDIO/first-0.wav"),
        );
        let _ = fs::copy(
            PathBuf::from("./data/tests/misc/test.wav"),
            base_path.join("test/AUDIO/second-0.wav"),
        );
        let _ = fs::copy(
            PathBuf::from("./data/tests/misc/pair.wav"),
            base_path.join("test/AUDIO/third-0.wav"),
        );
        let _ = fs::copy(
            PathBuf::from("./data/tests/misc/pair.ot"),
            base_path.join("test/AUDIO/third-0.ot"),
        );
        let _ = fs::copy(
            PathBuf::from("./data/tests/misc/test.wav"),
            base_path.join("test/PROJECT/fourth-0.wav"),
        );
        let _ = fs::copy(
            PathBuf::from("./data/tests/misc/pair.wav"),
            base_path.join("test/PROJECT/fifth-0.wav"),
        );
        let _ = fs::copy(
            PathBuf::from("./data/tests/misc/pair.ot"),
            base_path.join("test/PROJECT/fifth-0.ot"),
        );
    }

    #[test]
    fn test_consolidate_sslots_audio_pool_ok() {
        let base_path =
            PathBuf::from("./data/tests/projects/sample-slots/consolidation/to_audio_pool/");

        if base_path.join("test/").exists() {
            let _ = std::fs::remove_dir_all(&base_path.join("test/"));
        };

        make_sslot_mock_set_dir(&base_path);

        let _ = load_project(
            &base_path.join("init/project.yaml"),
            &base_path.join("test/PROJECT/project.work"),
        )
        .unwrap();

        let r =
            consolidate_sample_slots_to_audio_pool(&base_path.join("test/PROJECT/project.work"));

        assert!(r.is_ok());

        assert!(base_path.join("test/PROJECT/project.work").exists());
        assert!(base_path.join("test/AUDIO/first-0.wav").exists());
        assert!(base_path.join("test/AUDIO/second-0.wav").exists());
        assert!(base_path.join("test/AUDIO/third-0.wav").exists());
        assert!(base_path.join("test/AUDIO/fourth-0.wav").exists());
        assert!(base_path.join("test/AUDIO/fifth-0.wav").exists());
        assert!(base_path.join("test/AUDIO/fifth-0.ot").exists());
        assert!(base_path.join("test/PROJECT/fourth-0.wav").exists());
        assert!(base_path.join("test/PROJECT/fifth-0.wav").exists());
        assert!(base_path.join("test/PROJECT/fifth-0.ot").exists());
    }

    #[test]
    fn test_consolidate_sslots_project_pool_ok() {
        let base_path =
            PathBuf::from("./data/tests/projects/sample-slots/consolidation/to_project_pool/");

        if base_path.join("test/").exists() {
            let _ = std::fs::remove_dir_all(&base_path.join("test/"));
        };

        make_sslot_mock_set_dir(&base_path);

        let _ = load_project(
            &base_path.join("init/project.yaml"),
            &base_path.join("test/PROJECT/project.work"),
        )
        .unwrap();

        let r =
            consolidate_sample_slots_to_project_pool(&base_path.join("test/PROJECT/project.work"));

        assert!(r.is_ok());

        assert!(base_path.join("test/PROJECT/project.work").exists());
        assert!(base_path.join("test/AUDIO/first-0.wav").exists());
        assert!(base_path.join("test/AUDIO/second-0.wav").exists());
        assert!(base_path.join("test/AUDIO/third-0.wav").exists());
        assert!(base_path.join("test/PROJECT/first-0.wav").exists());
        assert!(base_path.join("test/PROJECT/second-0.wav").exists());
        assert!(base_path.join("test/PROJECT/third-0.wav").exists());
        assert!(base_path.join("test/PROJECT/fourth-0.wav").exists());
        assert!(base_path.join("test/PROJECT/fifth-0.wav").exists());
        assert!(base_path.join("test/PROJECT/fifth-0.ot").exists());
    }

    #[test]
    fn test_purge_project_pool_ok() {
        let base_path = PathBuf::from("./data/tests/projects/sample-slots/purge/project_pool/");

        if base_path.join("test/").exists() {
            let _ = std::fs::remove_dir_all(&base_path.join("test/"));
        };

        make_sslot_mock_set_dir(&base_path);

        let _ = fs::copy(
            PathBuf::from("./data/tests/misc/pair.wav"),
            base_path.join("test/PROJECT/deleteme1.wav"),
        );
        let _ = fs::copy(
            PathBuf::from("./data/tests/misc/pair.ot"),
            base_path.join("test/PROJECT/deleteme1.ot"),
        );
        let _ = fs::copy(
            PathBuf::from("./data/tests/misc/test.wav"),
            base_path.join("test/PROJECT/deleteme2.wav"),
        );

        let _ = load_project(
            &base_path.join("init/project.yaml"),
            &base_path.join("test/PROJECT/project.work"),
        )
        .unwrap();

        let r = purge_project_pool(&base_path.join("test/PROJECT/project.work"));

        assert!(r.is_ok());

        assert!(base_path.join("test/PROJECT/project.work").exists());
        assert!(base_path.join("test/AUDIO/first-0.wav").exists());
        assert!(base_path.join("test/AUDIO/second-0.wav").exists());
        assert!(base_path.join("test/AUDIO/third-0.wav").exists());
        assert!(base_path.join("test/PROJECT/fourth-0.wav").exists());
        assert!(base_path.join("test/PROJECT/fifth-0.wav").exists());
        assert!(base_path.join("test/PROJECT/fifth-0.ot").exists());
        assert!(!base_path.join("test/PROJECT/deleteme1.wav").exists());
        assert!(!base_path.join("test/PROJECT/deleteme1.ot").exists());
        assert!(!base_path.join("test/PROJECT/deleteme2.wav").exists());
    }
}
