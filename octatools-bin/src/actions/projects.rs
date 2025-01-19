use crate::audio::utils::scan_dir_path_for_audio_files;
use crate::RBoxErr;

use serde_octatrack::{
    projects::{slots::ProjectSampleSlot, Project},
    read_type_from_bin_file, write_type_to_bin_file,
};

use itertools::Itertools;
use std::{
    fs,
    path::{Path, PathBuf},
};

/// List all the sample slots within an Octatrack Project, given a path to a Project data file
pub fn list_project_sample_slots(path: &Path) -> RBoxErr<()> {
    let project = read_type_from_bin_file::<Project>(path).expect("Could not load project file");

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

    let mut project =
        read_type_from_bin_file::<Project>(project_file_path).expect("Could not load project file");

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

            let _ = fs::copy(&audio_fpath, &new_audio_path)?;

            let mut ot_filepath = audio_fpath.clone();
            ot_filepath.set_extension("ot");

            if ot_filepath.exists() {
                let mut new_otfile_path = audio_pool_path.join(&audio_fname);
                new_otfile_path.set_extension("ot");

                let _ = fs::copy(&ot_filepath, &new_otfile_path)?;
            }

            slot.path = new_audio_path;
        }
    }

    project.slots = slots;
    write_type_to_bin_file::<Project>(&project, &abs_project_fp)
        .expect("Could not save project file");

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

    let mut project =
        read_type_from_bin_file::<Project>(project_file_path).expect("Could not load project file");

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

            let _ = fs::copy(&audio_fpath, &new_audio_path)?;

            let mut ot_filepath = audio_fpath.clone();
            ot_filepath.set_extension("ot");

            if ot_filepath.exists() {
                let mut new_otfile_path = project_dir_path.join(&audio_fname);
                new_otfile_path.set_extension("ot");

                let _ = fs::copy(&ot_filepath, &new_otfile_path)?;
            }

            slot.path = new_audio_path;
        }
    }

    project.slots = slots;
    write_type_to_bin_file::<Project>(&project, &abs_project_fp)
        .expect("Could not save project file");

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

    let project =
        read_type_from_bin_file::<Project>(&abs_project_fp).expect("Could not load project file");

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

#[cfg(test)]
#[allow(unused_imports)]
mod test {
    use super::*;

    #[test]
    fn test_list_sample_slots_ok() {
        let fp = PathBuf::from("../data/tests/blank-project/project.work");
        let r = list_project_sample_slots(&fp);
        assert!(r.is_ok())
    }

    #[allow(dead_code)]  // only dead code on windows!
    fn make_sslot_mock_set_dir(base_path: &PathBuf) {
        let _ = fs::create_dir(fs::canonicalize(base_path).unwrap());
        let _ = fs::create_dir(fs::canonicalize(base_path.join("AUDIO")).unwrap());
        let _ = fs::create_dir(fs::canonicalize(base_path.join("PROJECT")).unwrap());
        let _ = fs::copy(
            fs::canonicalize(PathBuf::from("./../data/tests/misc/test.wav")).unwrap(),
            fs::canonicalize(base_path.join("AUDIO/first-0.wav")).unwrap(),
        );
        let _ = fs::copy(
            fs::canonicalize(PathBuf::from("./../data/tests/misc/test.wav")).unwrap(),
            fs::canonicalize(base_path.join("AUDIO/second-0.wav")).unwrap(),
        );
        let _ = fs::copy(
            fs::canonicalize(PathBuf::from("./../data/tests/misc/pair.wav")).unwrap(),
            fs::canonicalize(base_path.join("AUDIO/third-0.wav")).unwrap(),
        );
        let _ = fs::copy(
            fs::canonicalize(PathBuf::from("./../data/tests/misc/pair.ot")).unwrap(),
            fs::canonicalize(base_path.join("AUDIO/third-0.ot")).unwrap(),
        );
        let _ = fs::copy(
            fs::canonicalize(PathBuf::from("./../data/tests/misc/test.wav")).unwrap(),
            fs::canonicalize(base_path.join("PROJECT/fourth-0.wav")).unwrap(),
        );
        let _ = fs::copy(
            fs::canonicalize(PathBuf::from("./../data/tests/misc/pair.wav")).unwrap(),
            fs::canonicalize(base_path.join("PROJECT/fifth-0.wav")).unwrap(),
        );
        let _ = fs::copy(
            fs::canonicalize(PathBuf::from("./../data/tests/misc/pair.ot")).unwrap(),
            fs::canonicalize(base_path.join("PROJECT/fifth-0.ot")).unwrap(),
        );
    }

    // fails due to paths on windows
    #[cfg(not(target_os = "windows"))]
    #[test]
    fn test_consolidate_sslots_audio_pool_ok() {
        let base_path =
            PathBuf::from("../data/tests/projects/sample-slots/consolidation/to_audio_pool/");

        let test_dir_path = std::env::temp_dir().join("ot_consolidate_audio_pool");

        if test_dir_path.exists() {
            let _ = fs::remove_dir_all(&test_dir_path);
        };

        make_sslot_mock_set_dir(&test_dir_path);

        let _ = yaml_file_to_bin_file::<Project>(
            &base_path.join("init/project.yaml"),
            &test_dir_path.join("PROJECT/project.work"),
        )
        .unwrap();

        let r = consolidate_sample_slots_to_audio_pool(&test_dir_path.join("PROJECT/project.work"));

        assert!(r.is_ok());

        assert!(test_dir_path.join("PROJECT/project.work").exists());
        assert!(test_dir_path.join("AUDIO/first-0.wav").exists());
        assert!(test_dir_path.join("AUDIO/second-0.wav").exists());
        assert!(test_dir_path.join("AUDIO/third-0.wav").exists());
        assert!(test_dir_path.join("AUDIO/fourth-0.wav").exists());
        assert!(test_dir_path.join("AUDIO/fifth-0.wav").exists());
        assert!(test_dir_path.join("AUDIO/fifth-0.ot").exists());
        assert!(test_dir_path.join("PROJECT/fourth-0.wav").exists());
        assert!(test_dir_path.join("PROJECT/fifth-0.wav").exists());
        assert!(test_dir_path.join("PROJECT/fifth-0.ot").exists());
    }

    // fails due to paths on windows
    #[cfg(not(target_os = "windows"))]
    #[test]
    fn test_consolidate_sslots_project_pool_ok() {
        let base_path =
            PathBuf::from("./../data/tests/projects/sample-slots/consolidation/to_project_pool/");

        let test_dir_path = std::env::temp_dir().join("ot_consolidate_project_pool");

        if test_dir_path.exists() {
            let _ = fs::remove_dir_all(&test_dir_path);
        };

        make_sslot_mock_set_dir(&test_dir_path);

        println!(
            "{:#?}",
            &test_dir_path.join("PROJECT/project.work").exists()
        );
        println!("{:#?}", &test_dir_path.join("PROJECT/project.work"));

        let _ = yaml_file_to_bin_file::<Project>(
            &base_path.join("init/project.yaml"),
            &test_dir_path.join("PROJECT/project.work"),
        )
        .unwrap();

        let r =
            consolidate_sample_slots_to_project_pool(&test_dir_path.join("PROJECT/project.work"));

        assert!(r.is_ok());

        assert!(test_dir_path.join("PROJECT/project.work").exists());
        assert!(test_dir_path.join("AUDIO/first-0.wav").exists());
        assert!(test_dir_path.join("AUDIO/second-0.wav").exists());
        assert!(test_dir_path.join("AUDIO/third-0.wav").exists());
        assert!(test_dir_path.join("PROJECT/first-0.wav").exists());
        assert!(test_dir_path.join("PROJECT/second-0.wav").exists());
        assert!(test_dir_path.join("PROJECT/third-0.wav").exists());
        assert!(test_dir_path.join("PROJECT/fourth-0.wav").exists());
        assert!(test_dir_path.join("PROJECT/fifth-0.wav").exists());
        assert!(test_dir_path.join("PROJECT/fifth-0.ot").exists());
    }

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn test_purge_project_pool_ok() {
        let base_path = PathBuf::from("./../data/tests/projects/sample-slots/purge/project_pool/");

        let test_dir_path = std::env::temp_dir().join("ot_purge_project_pool");

        if test_dir_path.exists() {
            let _ = fs::remove_dir_all(&test_dir_path);
        };

        make_sslot_mock_set_dir(&test_dir_path);

        let _ = fs::copy(
            PathBuf::from("./../data/tests/misc/pair.wav"),
            test_dir_path.join("PROJECT/deleteme1.wav"),
        );
        let _ = fs::copy(
            PathBuf::from("./../data/tests/misc/pair.ot"),
            test_dir_path.join("PROJECT/deleteme1.ot"),
        );
        let _ = fs::copy(
            PathBuf::from("./../data/tests/misc/test.wav"),
            test_dir_path.join("PROJECT/deleteme2.wav"),
        );

        let _ = yaml_file_to_bin_file::<Project>(
            &base_path.join("init/project.yaml"),
            &test_dir_path.join("PROJECT/project.work"),
        )
        .unwrap();

        let r = purge_project_pool(&base_path.join("test/PROJECT/project.work"));

        assert!(r.is_ok());

        assert!(test_dir_path.join("PROJECT/project.work").exists());
        assert!(test_dir_path.join("AUDIO/first-0.wav").exists());
        assert!(test_dir_path.join("AUDIO/second-0.wav").exists());
        assert!(test_dir_path.join("AUDIO/third-0.wav").exists());
        assert!(test_dir_path.join("PROJECT/fourth-0.wav").exists());
        assert!(test_dir_path.join("PROJECT/fifth-0.wav").exists());
        assert!(test_dir_path.join("PROJECT/fifth-0.ot").exists());
        assert!(!test_dir_path.join("PROJECT/deleteme1.wav").exists());
        assert!(!test_dir_path.join("PROJECT/deleteme1.ot").exists());
        assert!(!test_dir_path.join("PROJECT/deleteme2.wav").exists());
    }
}
