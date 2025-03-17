use crate::actions::banks::copy_bank_by_paths;
use copy_dir;
use octatools_lib::banks::parts::AudioTrackMachineSlot;
use octatools_lib::banks::Bank;
use octatools_lib::projects::{options::ProjectSampleSlotType, slots::ProjectSampleSlot, Project};
use octatools_lib::{read_type_from_bin_file, write_type_to_bin_file};
use std::env::temp_dir;
use std::path::PathBuf;
use utils::resolve_fname_and_fext_from_path;

use super::*;
use crate::OctatoolErrors;

mod unit {
    use super::*;

    mod resolve_fname_and_fext_from_path {
        use super::*;

        #[test]
        fn success_unhidden_rel_path_with_parent_dirs() {
            let path = PathBuf::from(".")
                .join("some")
                .join("path")
                .join("file.ext");
            let r = resolve_fname_and_fext_from_path(&path);
            assert!(r.is_ok());
            assert_eq!(r.unwrap(), "file.ext".to_string());
        }

        #[test]
        fn success_unhidden_rel_path_with_parent_dirs_no_period_prefix() {
            let path = PathBuf::from("some/path/to/a/file.ext");
            let r = resolve_fname_and_fext_from_path(&path);
            assert!(r.is_ok());
            assert_eq!(r.unwrap(), "file.ext".to_string());
        }

        #[test]
        fn success_unhidden_fname_fext_only() {
            let path = PathBuf::from("file.ext");
            let r = resolve_fname_and_fext_from_path(&path);
            assert!(r.is_ok());
            assert_eq!(r.unwrap(), "file.ext".to_string());
        }

        #[test]
        fn success_unhidden_abs_path_with_parent_dirs() {
            let path = PathBuf::from("/some/path/to/a/file.ext");
            let r = resolve_fname_and_fext_from_path(&path);
            assert!(r.is_ok());
            assert_eq!(r.unwrap(), "file.ext".to_string());
        }

        #[test]
        fn success_hidden_rel_path_with_parent_dirs() {
            let path = PathBuf::from("./some/path/to/a/.file.ext");
            let r = resolve_fname_and_fext_from_path(&path);
            assert!(r.is_ok());
            assert_eq!(r.unwrap(), ".file.ext".to_string());
        }

        #[test]
        fn success_hidden_rel_path_with_parent_dirs_no_period_prefix() {
            let path = PathBuf::from("some/path/to/a/.file.ext");
            let r = resolve_fname_and_fext_from_path(&path);
            assert!(r.is_ok());
            assert_eq!(r.unwrap(), ".file.ext".to_string());
        }

        #[test]
        fn success_hidden_fname_fext_only() {
            let path = PathBuf::from(".file.ext");
            let r = resolve_fname_and_fext_from_path(&path);
            assert!(r.is_ok());
            assert_eq!(r.unwrap(), ".file.ext".to_string());
        }

        #[test]
        fn fail_no_extension() {
            let path = PathBuf::from("some/path/to/file");
            let r = resolve_fname_and_fext_from_path(&path);
            assert!(r.is_err());
            assert!(matches!(
                r.unwrap_err().downcast_ref::<OctatoolErrors>(),
                Some(&OctatoolErrors::InvalidFilenameOrExtension)
            ));
        }

        #[test]
        fn fail_no_fname() {
            let path = PathBuf::from(".ext");
            let r = resolve_fname_and_fext_from_path(&path);
            assert!(r.is_err());
            assert!(matches!(
                r.unwrap_err().downcast_ref::<OctatoolErrors>(),
                Some(&OctatoolErrors::InvalidFilenameOrExtension)
            ));
        }
    }
}

mod integration {

    use super::*;

    #[derive(Debug)]
    struct TestPaths {
        audio_pool: PathBuf,
        inbank: PathBuf,
        outbank: PathBuf,
        inproject: PathBuf,
        outproject: PathBuf,
    }

    fn get_base_mock_path(test_name: &String) -> PathBuf {
        temp_dir()
            .join("octatools-cli")
            .join("copyBankTesting")
            .join(test_name)
    }

    fn tear_down_dirs(test_name: &String) {
        let base = get_base_mock_path(test_name);
        let _ = std::fs::remove_dir_all(base);
    }

    fn mock_dirs(test_name: &String) -> TestPaths {
        tear_down_dirs(test_name);
        let base = get_base_mock_path(test_name);

        println!("BASE: {:?}", base);

        let _ = std::fs::create_dir_all(&base);

        let paths = TestPaths {
            audio_pool: base.join("AUDIO"),
            inproject: base.join("BANK-COPY-SRC"),
            outproject: base.join("BANK-COPY-DEST"),
            inbank: base.join("BANK-COPY-SRC").join("bank01.work"),
            outbank: base.join("BANK-COPY-DEST").join("bank01.work"),
        };

        // create test data directories
        copy_dir::copy_dir(
            PathBuf::from("..")
                .join("data")
                .join("tests")
                .join("copy")
                .join("bank")
                .join("AUDIO"),
            &paths.audio_pool,
        )
        .unwrap();

        let _ = std::fs::create_dir_all(&paths.inproject);

        let _ = std::fs::create_dir_all(&paths.outproject);

        paths
    }

    fn write_mock_data_files(
        paths: &TestPaths,
        srcproj: &Project,
        srcbank: &Bank,
        destproj: &Project,
    ) {
        let _ = write_type_to_bin_file::<Project>(srcproj, &paths.inproject.join("project.work"));
        let _ = write_type_to_bin_file::<Project>(destproj, &paths.outproject.join("project.work"));
        let _ = write_type_to_bin_file::<Bank>(srcbank, &paths.inbank);
        // required by copy_bank to validate the dest bank file exists.
        let _ = write_type_to_bin_file::<Bank>(&Bank::default(), &paths.outbank);
    }

    fn edit_valid_test_data_part<F>(valid_destbank: &mut Bank, f: F)
    where
        F: Fn(usize, usize, &mut AudioTrackMachineSlot),
    {
        for (part_id, part) in valid_destbank.parts_unsaved.iter_mut().enumerate() {
            for (track_id, audio_track) in part.audio_track_machine_slots.iter_mut().enumerate() {
                f(part_id, track_id, audio_track);
            }
        }
    }

    mod copy_bank {
        use super::*;

        fn run_test(
            paths: &TestPaths,
            srcproj: &Project,
            srcbank: &Bank,
            destproj: &Project,
            valid_destproj: &Project,
            valid_destbank: &Bank,
        ) {
            write_mock_data_files(paths, srcproj, srcbank, destproj);

            let r = copy_bank_by_paths(&paths.inproject, &paths.outproject, 1, 1, false);
            // copy op was successful
            assert!(r.is_ok());

            // can read the copied bank file (no data corruption)
            let copiedbank_r = read_type_from_bin_file::<Bank>(&paths.outbank);
            assert!(copiedbank_r.is_ok());
            let copiedbank = copiedbank_r.unwrap();

            // can read the modified project file (no data corruption)
            let modifiedproj_r =
                read_type_from_bin_file::<Project>(&paths.outproject.join("project.work"));
            assert!(modifiedproj_r.is_ok());
            let modifiedproj = modifiedproj_r.unwrap();

            println!("MODIFIED PROJ SLOTS: {:#?}", modifiedproj.slots);
            println!("VALID PROJ SLOTS: {:#?}", valid_destproj.slots);

            // sample slots should all match
            for (valid_slot, modifiedslot) in
                valid_destproj.slots.iter().zip(modifiedproj.slots.iter())
            {
                assert_eq!(valid_slot, modifiedslot);
            }

            // all the plocks slots in the written data match what we created as validation data
            for (pattern_idx, (valid_pattern, copied_pattern)) in valid_destbank
                .patterns
                .iter()
                .zip(copiedbank.patterns.iter())
                .enumerate()
            {
                for (track_idx, (valid_track, copied_track)) in valid_pattern
                    .audio_track_trigs
                    .iter()
                    .zip(copied_pattern.audio_track_trigs.iter())
                    .enumerate()
                {
                    for (trig_idx, (valid_plocks, copied_plocks)) in valid_track
                        .plocks
                        .0
                        .iter()
                        .zip(copied_track.plocks.0.iter())
                        .enumerate()
                    {
                        if valid_plocks.static_slot_id != 255 {
                            println!("IDX: PATTERN: {} TRACK: {} TRIG: {} -- Static valid: {} v copied: {}", pattern_idx, track_idx, trig_idx, valid_plocks.static_slot_id, copied_plocks.static_slot_id);
                        }
                        assert_eq!(valid_plocks.static_slot_id, copied_plocks.static_slot_id);
                        assert_eq!(valid_plocks.flex_slot_id, copied_plocks.flex_slot_id);
                    }
                }
            }

            // all the track machines slots in the written data match what we created as validation data
            for (part_idx, (valid_part, copied_part)) in valid_destbank
                .parts_unsaved
                .iter()
                .zip(copiedbank.parts_unsaved.iter())
                .enumerate()
            {
                for (track_idx, (valid_track, copied_track)) in valid_part
                    .audio_track_machine_slots
                    .iter()
                    .zip(copied_part.audio_track_machine_slots.iter())
                    .enumerate()
                {
                    println!("IDX: PART: {} TRACK: {}", part_idx, track_idx);
                    assert_eq!(valid_track.static_slot_id, copied_track.static_slot_id);
                    assert_eq!(valid_track.flex_slot_id, copied_track.flex_slot_id);
                }
            }
        }

        #[test]
        // tests that everything works with a default bank -- there will be 'inactive' audio track
        // machine sample slots to handle
        fn ok_copy_bank_default_bank() {
            let test_name = "default".to_string();

            let paths = mock_dirs(&test_name);

            let srcproj = Project::default();
            let destproj = Project::default();
            let valid_destproj = Project::default();
            let srcbank = Bank::default();
            let mut valid_destbank = Bank::default();

            // track machine slot allocation will be pointed to the last free sample slot
            // if they do not already point at a sample slot -- track 1 in a default bank
            // will point at sample slot 1 --> so need to mutate in this case
            edit_valid_test_data_part(&mut valid_destbank, |_, _, audio_track| {
                audio_track.flex_slot_id = 127;
                audio_track.static_slot_id = 127;
            });

            run_test(
                &paths,
                &srcproj,
                &srcbank,
                &destproj,
                &valid_destproj,
                &valid_destbank,
            );

            tear_down_dirs(&test_name);
        }

        #[test]
        fn err_copy_to_non_default_without_force() {
            let test_name = "copy-to-non-empty-fail-no-force".to_string();

            let paths = mock_dirs(&test_name);

            let srcproj = Project::default();
            let destproj = Project::default();
            let srcbank = Bank::default();
            let mut valid_destbank = Bank::default();

            valid_destbank.patterns[0].header[0] = 24;

            write_mock_data_files(&paths, &srcproj, &srcbank, &destproj);
            // hack -- default bank is written in write_mock_data_files
            let _ = write_type_to_bin_file::<Bank>(&valid_destbank, &paths.outbank);

            let r = copy_bank_by_paths(&paths.inproject, &paths.outproject, 1, 1, false);

            assert!(r.is_err());
            assert_eq!(
                r.unwrap_err().to_string(),
                CliBankErrors::NoForceFlagWithModifiedDestination.to_string()
            );
        }

        #[test]
        fn ok_copy_to_non_default_with_force() {
            let test_name = "copy-to-non-empty-ok-forced".to_string();

            let paths = mock_dirs(&test_name);

            let srcproj = Project::default();
            let destproj = Project::default();
            let srcbank = Bank::default();
            let mut valid_destbank = Bank::default();

            valid_destbank.patterns[0].header[0] = 24;

            write_mock_data_files(&paths, &srcproj, &srcbank, &destproj);
            // hack -- default bank is written in write_mock_data_files
            let _ = write_type_to_bin_file::<Bank>(&valid_destbank, &paths.outbank);

            let r = copy_bank_by_paths(&paths.inproject, &paths.outproject, 1, 1, true);

            assert!(r.is_ok());
        }

        mod static_slots {
            use std::cmp::Ordering;

            #[test]
            fn test_one_slot_active_pattern() {
                use super::*;

                #[cfg(target_os = "windows")]
                let test_name = "copy2empty\\one_stat_act_pat".to_string();
                #[cfg(not(target_os = "windows"))]
                let test_name = "int/static/one_slot_active_pattern".to_string();

                let paths = mock_dirs(&test_name);

                let mut srcproj = Project::default();
                let destproj = Project::default();
                let mut valid_destproj = Project::default();
                let mut srcbank = Bank::default();

                // reminder: one indexed
                srcproj.slots.push(
                    ProjectSampleSlot::new(
                        ProjectSampleSlotType::Static,
                        1,
                        PathBuf::from("../AUDIO/first-0.wav"),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .unwrap(),
                );

                valid_destproj.slots.push(
                    ProjectSampleSlot::new(
                        ProjectSampleSlotType::Static,
                        1,
                        PathBuf::from("first-0.wav"),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .unwrap(),
                );

                // reminder: zero indexed
                srcbank.patterns[0].audio_track_trigs[0].plocks.0[0].static_slot_id = 0;
                let mut valid_destbank = srcbank.clone();

                // track machine slot allocation will be pointed to the last free sample slot
                // if they do not already point at a sample slot -- track 1 in a default bank
                // will point at sample slot 1 --> so need to mutate in this case
                edit_valid_test_data_part(&mut valid_destbank, |_, track_id, audio_track| {
                    audio_track.flex_slot_id = 127;
                    if track_id > 0 {
                        audio_track.static_slot_id = 127;
                    }
                });

                run_test(
                    &paths,
                    &srcproj,
                    &srcbank,
                    &destproj,
                    &valid_destproj,
                    &valid_destbank,
                );

                tear_down_dirs(&test_name);
            }

            #[test]
            fn n_slots_active_pattern() {
                use super::*;

                #[cfg(target_os = "windows")]
                let test_name = "copy2empty\\one_stat_act_pat".to_string();
                #[cfg(not(target_os = "windows"))]
                let test_name = "int/static/n_slots_active_pattern".to_string();

                let paths = mock_dirs(&test_name);

                let mut srcproj = Project::default();
                let destproj = Project::default();
                let mut valid_destproj = Project::default();
                let mut srcbank = Bank::default();

                // reminder: one indexed
                srcproj.slots.push(
                    // rename create
                    ProjectSampleSlot::new(
                        ProjectSampleSlotType::Static,
                        21,
                        PathBuf::from("../AUDIO/first-0.wav"),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .unwrap(),
                );

                // reminder: one indexed
                // slot reuse
                srcproj.slots.push(
                    ProjectSampleSlot::new(
                        ProjectSampleSlotType::Static,
                        22,
                        PathBuf::from("../AUDIO/first-0.wav"),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .unwrap(),
                );

                // reminder: one indexed
                // slot reuse
                srcproj.slots.push(
                    ProjectSampleSlot::new(
                        ProjectSampleSlotType::Static,
                        23,
                        PathBuf::from("../AUDIO/first-0.wav"),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .unwrap(),
                );

                // reminder: one indexed
                // rename create
                srcproj.slots.push(
                    ProjectSampleSlot::new(
                        ProjectSampleSlotType::Static,
                        24,
                        PathBuf::from("../AUDIO/second-0.wav"),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .unwrap(),
                );

                valid_destproj.slots.push(
                    ProjectSampleSlot::new(
                        ProjectSampleSlotType::Static,
                        1,
                        PathBuf::from("first-0.wav"),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .unwrap(),
                );

                valid_destproj.slots.push(
                    ProjectSampleSlot::new(
                        ProjectSampleSlotType::Static,
                        2,
                        PathBuf::from("second-0.wav"),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .unwrap(),
                );

                println!("src slots original: {:#?}", srcproj.slots);

                // reminder: zero indexed
                srcbank.patterns[0].audio_track_trigs[0].plocks.0[0].static_slot_id = 20;
                srcbank.patterns[0].audio_track_trigs[0].plocks.0[1].static_slot_id = 21;
                srcbank.patterns[0].audio_track_trigs[0].plocks.0[2].static_slot_id = 22;
                srcbank.patterns[0].audio_track_trigs[0].plocks.0[3].static_slot_id = 23;

                // handle slot remapping
                let mut valid_destbank = srcbank.clone();
                valid_destbank.patterns[0].audio_track_trigs[0].plocks.0[0].static_slot_id = 0;
                valid_destbank.patterns[0].audio_track_trigs[0].plocks.0[1].static_slot_id = 0;
                valid_destbank.patterns[0].audio_track_trigs[0].plocks.0[2].static_slot_id = 0;
                valid_destbank.patterns[0].audio_track_trigs[0].plocks.0[3].static_slot_id = 1;

                // track machine slot allocation will be pointed to the last free sample slot
                // if they do not already point at a sample slot -- need to mutate all track
                // machine slots
                edit_valid_test_data_part(&mut valid_destbank, |_, _, audio_track| {
                    audio_track.flex_slot_id = 127;
                    audio_track.static_slot_id = 127;
                });

                run_test(
                    &paths,
                    &srcproj,
                    &srcbank,
                    &destproj,
                    &valid_destproj,
                    &valid_destbank,
                );

                tear_down_dirs(&test_name);
            }

            #[test]
            fn n_slots_active_pattern_mutate_machines() {
                use super::*;

                #[cfg(target_os = "windows")]
                let test_name = "copy2empty\\one_stat_act_pat".to_string();
                #[cfg(not(target_os = "windows"))]
                let test_name = "int/static/n_slots_active_pattern_mutate_machines".to_string();

                let paths = mock_dirs(&test_name);

                let mut srcproj = Project::default();
                let destproj = Project::default();
                let mut valid_destproj = Project::default();
                let mut srcbank = Bank::default();

                // reminder: one indexed
                srcproj.slots.push(
                    // rename create
                    ProjectSampleSlot::new(
                        ProjectSampleSlotType::Static,
                        1,
                        PathBuf::from("../AUDIO/first-0.wav"),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .unwrap(),
                );

                // reminder: one indexed
                // slot reuse
                srcproj.slots.push(
                    ProjectSampleSlot::new(
                        ProjectSampleSlotType::Static,
                        2,
                        PathBuf::from("../AUDIO/first-0.wav"),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .unwrap(),
                );

                // reminder: one indexed
                // slot reuse
                srcproj.slots.push(
                    ProjectSampleSlot::new(
                        ProjectSampleSlotType::Static,
                        3,
                        PathBuf::from("../AUDIO/first-0.wav"),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .unwrap(),
                );

                // reminder: one indexed
                // rename create
                srcproj.slots.push(
                    ProjectSampleSlot::new(
                        ProjectSampleSlotType::Static,
                        4,
                        PathBuf::from("../AUDIO/second-0.wav"),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .unwrap(),
                );

                valid_destproj.slots.push(
                    ProjectSampleSlot::new(
                        ProjectSampleSlotType::Static,
                        1,
                        PathBuf::from("first-0.wav"),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .unwrap(),
                );

                valid_destproj.slots.push(
                    ProjectSampleSlot::new(
                        ProjectSampleSlotType::Static,
                        2,
                        PathBuf::from("second-0.wav"),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .unwrap(),
                );

                // reminder: zero indexed
                srcbank.patterns[0].audio_track_trigs[0].plocks.0[0].static_slot_id = 0;
                srcbank.patterns[0].audio_track_trigs[0].plocks.0[1].static_slot_id = 1;
                srcbank.patterns[0].audio_track_trigs[0].plocks.0[2].static_slot_id = 2;
                srcbank.patterns[0].audio_track_trigs[0].plocks.0[3].static_slot_id = 3;

                // handle slot reuse
                let mut valid_destbank = srcbank.clone();
                // slot reuses -- slots 0, 1, 2 are now on slot 0
                valid_destbank.patterns[0].audio_track_trigs[0].plocks.0[1].static_slot_id = 0;
                valid_destbank.patterns[0].audio_track_trigs[0].plocks.0[2].static_slot_id = 0;
                // slot reuses -- slots 3 is now on slot 1
                valid_destbank.patterns[0].audio_track_trigs[0].plocks.0[3].static_slot_id = 1;

                // track machine slot allocation will be pointed to the last free sample slot
                // if they do not already point at a sample slot -- tracks 1 to 4 in a default bank
                // will point at sample slots 1 to 4 --> so need to mutate in this case
                edit_valid_test_data_part(&mut valid_destbank, |_, track_id, audio_track| {
                    audio_track.flex_slot_id = 127;
                    audio_track.static_slot_id = match track_id.cmp(&3) {
                        Ordering::Less => 0,
                        Ordering::Greater => 127,
                        Ordering::Equal => 1,
                    };
                });

                run_test(
                    &paths,
                    &srcproj,
                    &srcbank,
                    &destproj,
                    &valid_destproj,
                    &valid_destbank,
                );

                tear_down_dirs(&test_name);
            }

            #[test]
            fn one_slot_active_part() {
                use super::*;

                #[cfg(target_os = "windows")]
                let test_name = "copy2empty\\one_stat_act_prt".to_string();
                #[cfg(not(target_os = "windows"))]
                let test_name = "int/static/one_slot_active_part".to_string();

                let paths = mock_dirs(&test_name);

                let mut srcproj = Project::default();
                let destproj = Project::default();
                let mut valid_destproj = Project::default();
                let mut srcbank = Bank::default();

                // reminder: one indexed
                srcproj.slots.push(
                    ProjectSampleSlot::new(
                        ProjectSampleSlotType::Static,
                        17,
                        PathBuf::from("../AUDIO/first-0.wav"),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .unwrap(),
                );

                valid_destproj.slots.push(
                    ProjectSampleSlot::new(
                        ProjectSampleSlotType::Static,
                        1,
                        PathBuf::from("first-0.wav"),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .unwrap(),
                );

                // reminder: zero indexed
                srcbank.parts_unsaved[0].audio_track_machine_slots[0].static_slot_id = 16;
                let mut valid_destbank = srcbank.clone();

                // track machine slot allocation will be pointed to the last free sample slot
                // if they do not already point at a sample slot -- track 1 in a default bank
                // will point at sample slot 1 --> so need to mutate in this case
                edit_valid_test_data_part(&mut valid_destbank, |part_id, track_id, audio_track| {
                    audio_track.flex_slot_id = 127;
                    if part_id == 0 && track_id == 0 {
                        audio_track.static_slot_id = 0;
                    } else {
                        audio_track.static_slot_id = 127;
                    }
                });

                run_test(
                    &paths,
                    &srcproj,
                    &srcbank,
                    &destproj,
                    &valid_destproj,
                    &valid_destbank,
                );

                tear_down_dirs(&test_name);
            }

            #[test]
            fn no_free_dest_slots_no_slot_reuses_fails() {
                use super::*;

                #[cfg(target_os = "windows")]
                let test_name = "copy2empty\\one_stat_act_pat".to_string();
                #[cfg(not(target_os = "windows"))]
                let test_name = "int/static/no_free_dest_slots_no_slot_reuses".to_string();

                let paths = mock_dirs(&test_name);

                let mut srcproj = Project::default();
                let mut destproj = Project::default();
                let srcbank = Bank::default();

                for i in 1..=128_u8 {
                    destproj.slots.push(
                        ProjectSampleSlot::new(
                            ProjectSampleSlotType::Static,
                            i,
                            // no matching samples, cannot reuse slots
                            PathBuf::from(format!["../AUDIO/{i}-0.wav"]),
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                        )
                        .unwrap(),
                    )
                }

                srcproj.slots.push(
                    ProjectSampleSlot::new(
                        ProjectSampleSlotType::Static,
                        1,
                        PathBuf::from("../AUDIO/first-0.wav"),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .unwrap(),
                );

                write_mock_data_files(&paths, &srcproj, &srcbank, &destproj);

                let r = copy_bank_by_paths(&paths.inproject, &paths.outproject, 1, 1, false);
                assert!(r.is_err());

                tear_down_dirs(&test_name);
            }

            #[test]
            fn no_free_dest_slots_can_slot_reuse_fails() {
                // this should fail because we cannot get an empty slot reservation in the
                // destination

                use super::*;

                #[cfg(target_os = "windows")]
                let test_name = "copy2empty\\one_stat_act_pat".to_string();
                #[cfg(not(target_os = "windows"))]
                let test_name = "int/static/no_free_dest_slots_can_slot_reuse__fails".to_string();

                let paths = mock_dirs(&test_name);

                let mut srcproj = Project::default();
                let mut destproj = Project::default();
                let srcbank = Bank::default();

                for i in 1..=128_u8 {
                    destproj.slots.push(
                        ProjectSampleSlot::new(
                            ProjectSampleSlotType::Static,
                            i,
                            // samples all match, can reuse a slot
                            PathBuf::from("../AUDIO/first-0.wav"),
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                        )
                        .unwrap(),
                    )
                }

                srcproj.slots.push(
                    ProjectSampleSlot::new(
                        ProjectSampleSlotType::Static,
                        1,
                        PathBuf::from("../AUDIO/first-0.wav"),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .unwrap(),
                );

                write_mock_data_files(&paths, &srcproj, &srcbank, &destproj);

                let r = copy_bank_by_paths(&paths.inproject, &paths.outproject, 1, 1, false);
                assert!(r.is_ok());

                tear_down_dirs(&test_name);
            }

            #[test]
            fn one_free_dest_slots_no_slot_reuses_fails() {
                // should fail because we actually need 2 free sample slots in this case.
                // one reserved for inactive sample slot assignments (default track machines),
                // the other for the sample slot we have to create

                use super::*;

                #[cfg(target_os = "windows")]
                let test_name = "copy2empty\\one_stat_act_pat".to_string();
                #[cfg(not(target_os = "windows"))]
                let test_name = "int/static/one_free_dest_slots_no_slot_reuses_fails".to_string();

                let paths = mock_dirs(&test_name);

                let mut srcproj = Project::default();
                let mut destproj = Project::default();
                let srcbank = Bank::default();

                for i in 1..=127_u8 {
                    destproj.slots.push(
                        ProjectSampleSlot::new(
                            ProjectSampleSlotType::Static,
                            i,
                            // no matching samples, cannot reuse slots
                            PathBuf::from(format!["../AUDIO/{i}-0.wav"]),
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                        )
                        .unwrap(),
                    )
                }

                srcproj.slots.push(
                    ProjectSampleSlot::new(
                        ProjectSampleSlotType::Static,
                        1,
                        PathBuf::from("../AUDIO/first-0.wav"),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .unwrap(),
                );

                write_mock_data_files(&paths, &srcproj, &srcbank, &destproj);

                let r = copy_bank_by_paths(&paths.inproject, &paths.outproject, 1, 1, false);
                assert!(r.is_err());

                tear_down_dirs(&test_name);
            }

            #[test]
            fn one_free_dest_slots_only_slot_reuses_success() {
                // succeeds because we have a free empty slot and we only remap onto
                // existing destination sample slots

                use super::*;

                #[cfg(target_os = "windows")]
                let test_name = "copy2empty\\one_stat_act_pat".to_string();
                #[cfg(not(target_os = "windows"))]
                let test_name = "int/static/one_free_dest_slots_only_slot_reuses_fail".to_string();

                let paths = mock_dirs(&test_name);

                let mut srcproj = Project::default();
                let mut destproj = Project::default();
                let srcbank = Bank::default();

                for i in 1..=127_u8 {
                    destproj.slots.push(
                        ProjectSampleSlot::new(
                            ProjectSampleSlotType::Static,
                            i,
                            PathBuf::from("../AUDIO/first-0.wav"),
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                        )
                        .unwrap(),
                    )
                }

                srcproj.slots.push(
                    ProjectSampleSlot::new(
                        ProjectSampleSlotType::Static,
                        1,
                        PathBuf::from("../AUDIO/first-0.wav"),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .unwrap(),
                );

                write_mock_data_files(&paths, &srcproj, &srcbank, &destproj);

                let r = copy_bank_by_paths(&paths.inproject, &paths.outproject, 1, 1, false);
                println!("r: {:?}", r);
                assert!(r.is_ok());

                tear_down_dirs(&test_name);
            }
        }

        mod flex_slots {
            use std::cmp::Ordering;

            #[test]
            fn test_one_slot_active_pattern() {
                use super::*;

                #[cfg(target_os = "windows")]
                let test_name = "copy2empty\\one_stat_act_pat".to_string();
                #[cfg(not(target_os = "windows"))]
                let test_name = "int/flex/one_slot_active_pattern".to_string();

                let paths = mock_dirs(&test_name);

                let mut srcproj = Project::default();
                let destproj = Project::default();
                let mut valid_destproj = Project::default();
                let mut srcbank = Bank::default();

                // reminder: one indexed
                srcproj.slots.push(
                    ProjectSampleSlot::new(
                        ProjectSampleSlotType::Flex,
                        1,
                        PathBuf::from("../AUDIO/first-0.wav"),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .unwrap(),
                );

                valid_destproj.slots.push(
                    ProjectSampleSlot::new(
                        ProjectSampleSlotType::Flex,
                        1,
                        PathBuf::from("first-0.wav"),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .unwrap(),
                );

                // reminder: zero indexed
                srcbank.patterns[0].audio_track_trigs[0].plocks.0[0].flex_slot_id = 0;
                let mut valid_destbank = srcbank.clone();

                // track machine slot allocation will be pointed to the last free sample slot
                // if they do not already point at a sample slot -- track 1 in a default bank
                // will point at sample slot 1 --> so need to mutate in this case
                edit_valid_test_data_part(&mut valid_destbank, |_, track_id, audio_track| {
                    audio_track.static_slot_id = 127;
                    if track_id > 0 {
                        audio_track.flex_slot_id = 127;
                    }
                });

                run_test(
                    &paths,
                    &srcproj,
                    &srcbank,
                    &destproj,
                    &valid_destproj,
                    &valid_destbank,
                );

                tear_down_dirs(&test_name);
            }

            #[test]
            fn n_slots_active_pattern() {
                use super::*;

                #[cfg(target_os = "windows")]
                let test_name = "copy2empty\\one_stat_act_pat".to_string();
                #[cfg(not(target_os = "windows"))]
                let test_name = "int/flex/n_slots_active_pattern".to_string();

                let paths = mock_dirs(&test_name);

                let mut srcproj = Project::default();
                let destproj = Project::default();
                let mut valid_destproj = Project::default();
                let mut srcbank = Bank::default();

                // reminder: one indexed
                srcproj.slots.push(
                    // rename create
                    ProjectSampleSlot::new(
                        ProjectSampleSlotType::Flex,
                        21,
                        PathBuf::from("../AUDIO/first-0.wav"),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .unwrap(),
                );

                // reminder: one indexed
                // slot reuse
                srcproj.slots.push(
                    ProjectSampleSlot::new(
                        ProjectSampleSlotType::Flex,
                        22,
                        PathBuf::from("../AUDIO/first-0.wav"),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .unwrap(),
                );

                // reminder: one indexed
                // slot reuse
                srcproj.slots.push(
                    ProjectSampleSlot::new(
                        ProjectSampleSlotType::Flex,
                        23,
                        PathBuf::from("../AUDIO/first-0.wav"),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .unwrap(),
                );

                // reminder: one indexed
                // rename create
                srcproj.slots.push(
                    ProjectSampleSlot::new(
                        ProjectSampleSlotType::Flex,
                        24,
                        PathBuf::from("../AUDIO/second-0.wav"),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .unwrap(),
                );

                valid_destproj.slots.push(
                    ProjectSampleSlot::new(
                        ProjectSampleSlotType::Flex,
                        1,
                        PathBuf::from("first-0.wav"),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .unwrap(),
                );

                valid_destproj.slots.push(
                    ProjectSampleSlot::new(
                        ProjectSampleSlotType::Flex,
                        2,
                        PathBuf::from("second-0.wav"),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .unwrap(),
                );

                println!("src slots original: {:#?}", srcproj.slots);

                // reminder: zero indexed
                srcbank.patterns[0].audio_track_trigs[0].plocks.0[0].flex_slot_id = 20;
                srcbank.patterns[0].audio_track_trigs[0].plocks.0[1].flex_slot_id = 21;
                srcbank.patterns[0].audio_track_trigs[0].plocks.0[2].flex_slot_id = 22;
                srcbank.patterns[0].audio_track_trigs[0].plocks.0[3].flex_slot_id = 23;

                // handle slot remapping
                let mut valid_destbank = srcbank.clone();
                valid_destbank.patterns[0].audio_track_trigs[0].plocks.0[0].flex_slot_id = 0;
                valid_destbank.patterns[0].audio_track_trigs[0].plocks.0[1].flex_slot_id = 0;
                valid_destbank.patterns[0].audio_track_trigs[0].plocks.0[2].flex_slot_id = 0;
                valid_destbank.patterns[0].audio_track_trigs[0].plocks.0[3].flex_slot_id = 1;

                // track machine slot allocation will be pointed to the last free sample slot
                // if they do not already point at a sample slot -- need to mutate all track
                // machine slots
                edit_valid_test_data_part(&mut valid_destbank, |_, _, audio_track| {
                    audio_track.static_slot_id = 127;
                    audio_track.flex_slot_id = 127;
                });

                run_test(
                    &paths,
                    &srcproj,
                    &srcbank,
                    &destproj,
                    &valid_destproj,
                    &valid_destbank,
                );

                tear_down_dirs(&test_name);
            }

            #[test]
            fn n_slots_active_pattern_mutate_machines() {
                use super::*;

                #[cfg(target_os = "windows")]
                let test_name = "copy2empty\\one_stat_act_pat".to_string();
                #[cfg(not(target_os = "windows"))]
                let test_name = "int/flex/n_slots_active_pattern_mutate_machines".to_string();

                let paths = mock_dirs(&test_name);

                let mut srcproj = Project::default();
                let destproj = Project::default();
                let mut valid_destproj = Project::default();
                let mut srcbank = Bank::default();

                // reminder: one indexed
                srcproj.slots.push(
                    // rename create
                    ProjectSampleSlot::new(
                        ProjectSampleSlotType::Flex,
                        1,
                        PathBuf::from("../AUDIO/first-0.wav"),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .unwrap(),
                );

                // reminder: one indexed
                // slot reuse
                srcproj.slots.push(
                    ProjectSampleSlot::new(
                        ProjectSampleSlotType::Flex,
                        2,
                        PathBuf::from("../AUDIO/first-0.wav"),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .unwrap(),
                );

                // reminder: one indexed
                // slot reuse
                srcproj.slots.push(
                    ProjectSampleSlot::new(
                        ProjectSampleSlotType::Flex,
                        3,
                        PathBuf::from("../AUDIO/first-0.wav"),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .unwrap(),
                );

                // reminder: one indexed
                // rename create
                srcproj.slots.push(
                    ProjectSampleSlot::new(
                        ProjectSampleSlotType::Flex,
                        4,
                        PathBuf::from("../AUDIO/second-0.wav"),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .unwrap(),
                );

                valid_destproj.slots.push(
                    ProjectSampleSlot::new(
                        ProjectSampleSlotType::Flex,
                        1,
                        PathBuf::from("first-0.wav"),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .unwrap(),
                );

                valid_destproj.slots.push(
                    ProjectSampleSlot::new(
                        ProjectSampleSlotType::Flex,
                        2,
                        PathBuf::from("second-0.wav"),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .unwrap(),
                );

                // reminder: zero indexed
                srcbank.patterns[0].audio_track_trigs[0].plocks.0[0].flex_slot_id = 0;
                srcbank.patterns[0].audio_track_trigs[0].plocks.0[1].flex_slot_id = 1;
                srcbank.patterns[0].audio_track_trigs[0].plocks.0[2].flex_slot_id = 2;
                srcbank.patterns[0].audio_track_trigs[0].plocks.0[3].flex_slot_id = 3;

                // handle slot reuse
                let mut valid_destbank = srcbank.clone();
                // slot reuses -- slots 0, 1, 2 are now on slot 0
                valid_destbank.patterns[0].audio_track_trigs[0].plocks.0[1].flex_slot_id = 0;
                valid_destbank.patterns[0].audio_track_trigs[0].plocks.0[2].flex_slot_id = 0;
                // slot reuses -- slots 3 is now on slot 1
                valid_destbank.patterns[0].audio_track_trigs[0].plocks.0[3].flex_slot_id = 1;

                // track machine slot allocation will be pointed to the last free sample slot
                // if they do not already point at a sample slot -- tracks 1 to 4 in a default bank
                // will point at sample slots 1 to 4 --> so need to mutate in this case
                edit_valid_test_data_part(&mut valid_destbank, |_, track_id, audio_track| {
                    audio_track.static_slot_id = 127;
                    audio_track.flex_slot_id = match track_id.cmp(&3) {
                        Ordering::Less => 0,
                        Ordering::Greater => 127,
                        Ordering::Equal => 1,
                    };
                });

                run_test(
                    &paths,
                    &srcproj,
                    &srcbank,
                    &destproj,
                    &valid_destproj,
                    &valid_destbank,
                );

                tear_down_dirs(&test_name);
            }

            #[test]
            fn one_slot_active_part() {
                use super::*;

                #[cfg(target_os = "windows")]
                let test_name = "copy2empty\\one_stat_act_prt".to_string();
                #[cfg(not(target_os = "windows"))]
                let test_name = "int/flex/one_slot_active_part".to_string();

                let paths = mock_dirs(&test_name);

                let mut srcproj = Project::default();
                let destproj = Project::default();
                let mut valid_destproj = Project::default();
                let mut srcbank = Bank::default();

                // reminder: one indexed
                srcproj.slots.push(
                    ProjectSampleSlot::new(
                        ProjectSampleSlotType::Flex,
                        17,
                        PathBuf::from("../AUDIO/first-0.wav"),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .unwrap(),
                );

                valid_destproj.slots.push(
                    ProjectSampleSlot::new(
                        ProjectSampleSlotType::Flex,
                        1,
                        PathBuf::from("first-0.wav"),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .unwrap(),
                );

                // reminder: zero indexed
                srcbank.parts_unsaved[0].audio_track_machine_slots[0].flex_slot_id = 16;
                let mut valid_destbank = srcbank.clone();

                // track machine slot allocation will be pointed to the last free sample slot
                // if they do not already point at a sample slot -- track 1 in a default bank
                // will point at sample slot 1 --> so need to mutate in this case
                edit_valid_test_data_part(&mut valid_destbank, |part_id, track_id, audio_track| {
                    audio_track.static_slot_id = 127;
                    if part_id == 0 && track_id == 0 {
                        audio_track.flex_slot_id = 0;
                    } else {
                        audio_track.flex_slot_id = 127;
                    }
                });

                run_test(
                    &paths,
                    &srcproj,
                    &srcbank,
                    &destproj,
                    &valid_destproj,
                    &valid_destbank,
                );

                tear_down_dirs(&test_name);
            }

            #[test]
            fn no_free_dest_slots_no_slot_reuses_fails() {
                use super::*;

                #[cfg(target_os = "windows")]
                let test_name = "copy2empty\\one_stat_act_pat".to_string();
                #[cfg(not(target_os = "windows"))]
                let test_name = "int/flex/no_free_dest_slots_no_slot_reuses".to_string();

                let paths = mock_dirs(&test_name);

                let mut srcproj = Project::default();
                let mut destproj = Project::default();
                let srcbank = Bank::default();

                for i in 1..=128_u8 {
                    destproj.slots.push(
                        ProjectSampleSlot::new(
                            ProjectSampleSlotType::Flex,
                            i,
                            // no matching samples, cannot reuse slots
                            PathBuf::from(format!["../AUDIO/{i}-0.wav"]),
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                        )
                        .unwrap(),
                    )
                }

                srcproj.slots.push(
                    ProjectSampleSlot::new(
                        ProjectSampleSlotType::Flex,
                        1,
                        PathBuf::from("../AUDIO/first-0.wav"),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .unwrap(),
                );

                write_mock_data_files(&paths, &srcproj, &srcbank, &destproj);

                let r = copy_bank_by_paths(&paths.inproject, &paths.outproject, 1, 1, false);
                assert!(r.is_err());

                tear_down_dirs(&test_name);
            }

            #[test]
            fn no_free_dest_slots_can_slot_reuse_fails() {
                // this should fail because we cannot get an empty slot reservation in the
                // destination

                use super::*;
                #[cfg(target_os = "windows")]
                let test_name = "copy2empty\\one_stat_act_pat".to_string();
                #[cfg(not(target_os = "windows"))]
                let test_name = "int/flex/no_free_dest_slots_can_slot_reuse__fails".to_string();

                let paths = mock_dirs(&test_name);

                let mut srcproj = Project::default();
                let mut destproj = Project::default();
                let srcbank = Bank::default();

                for i in 1..=128_u8 {
                    destproj.slots.push(
                        ProjectSampleSlot::new(
                            ProjectSampleSlotType::Flex,
                            i,
                            // samples all match, can reuse a slot
                            PathBuf::from("../AUDIO/first-0.wav"),
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                        )
                        .unwrap(),
                    )
                }

                srcproj.slots.push(
                    ProjectSampleSlot::new(
                        ProjectSampleSlotType::Flex,
                        1,
                        PathBuf::from("../AUDIO/first-0.wav"),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .unwrap(),
                );

                write_mock_data_files(&paths, &srcproj, &srcbank, &destproj);

                let r = copy_bank_by_paths(&paths.inproject, &paths.outproject, 1, 1, false);
                assert!(r.is_ok());

                tear_down_dirs(&test_name);
            }

            #[test]
            fn one_free_dest_slots_no_slot_reuses_fails() {
                // should fail because we actually need 2 free sample slots in this case.
                // one reserved for inactive sample slot assignments (default track machines),
                // the other for the sample slot we have to create

                use super::*;

                #[cfg(target_os = "windows")]
                let test_name = "copy2empty\\one_stat_act_pat".to_string();
                #[cfg(not(target_os = "windows"))]
                let test_name = "int/flex/one_free_dest_slots_no_slot_reuses_fails".to_string();

                let paths = mock_dirs(&test_name);

                let mut srcproj = Project::default();
                let mut destproj = Project::default();
                let srcbank = Bank::default();

                for i in 1..=127_u8 {
                    destproj.slots.push(
                        ProjectSampleSlot::new(
                            ProjectSampleSlotType::Flex,
                            i,
                            // no matching samples, cannot reuse slots
                            PathBuf::from(format!["../AUDIO/{i}-0.wav"]),
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                        )
                        .unwrap(),
                    )
                }

                srcproj.slots.push(
                    ProjectSampleSlot::new(
                        ProjectSampleSlotType::Flex,
                        1,
                        PathBuf::from("../AUDIO/first-0.wav"),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .unwrap(),
                );

                write_mock_data_files(&paths, &srcproj, &srcbank, &destproj);

                let r = copy_bank_by_paths(&paths.inproject, &paths.outproject, 1, 1, false);
                assert!(r.is_err());

                tear_down_dirs(&test_name);
            }

            #[test]
            fn one_free_dest_slots_only_slot_reuses_success() {
                // should fail because we actually need 2 free sample slots in this case.
                // one reserved for inactive sample slot assignments (default track machines),
                // the other for the sample slot we have to create

                // TODO: There's an optimization here where we work out whether we even need
                //       to remap, but i think that's going to be a very rare case.

                use super::*;

                #[cfg(target_os = "windows")]
                let test_name = "copy2empty\\one_stat_act_pat".to_string();
                #[cfg(not(target_os = "windows"))]
                let test_name = "int/flex/one_free_dest_slots_only_slot_reuses_success".to_string();

                let paths = mock_dirs(&test_name);

                let mut srcproj = Project::default();
                let mut destproj = Project::default();
                let srcbank = Bank::default();

                for i in 1..=127_u8 {
                    destproj.slots.push(
                        ProjectSampleSlot::new(
                            ProjectSampleSlotType::Flex,
                            i,
                            PathBuf::from("../AUDIO/first-0.wav"),
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                        )
                        .unwrap(),
                    )
                }

                srcproj.slots.push(
                    ProjectSampleSlot::new(
                        ProjectSampleSlotType::Flex,
                        1,
                        PathBuf::from("../AUDIO/first-0.wav"),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .unwrap(),
                );

                write_mock_data_files(&paths, &srcproj, &srcbank, &destproj);

                let r = copy_bank_by_paths(&paths.inproject, &paths.outproject, 1, 1, false);
                println!("r: {:?}", r);
                assert!(r.is_ok());

                tear_down_dirs(&test_name);
            }
        }
    }
}
