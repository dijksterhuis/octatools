mod chain_deconstruct {

    use crate::actions::samples::deconstruct_samplechain_from_paths;
    use std::{fs, path::Path, path::PathBuf};

    #[test]
    fn test_basic() {
        let audio_fpath = PathBuf::from("../data/tests/chains/deconstruct/test.wav");
        let attributes_fpath = PathBuf::from("../data/tests/chains/deconstruct/test.ot");
        let outdir = std::env::temp_dir().join("");

        let res = deconstruct_samplechain_from_paths(&audio_fpath, &attributes_fpath, &outdir);

        let outfiles = res.unwrap();

        let files_exist: bool = outfiles.iter().all(|fp| fp.exists());

        // clean up
        for file in outfiles {
            let _ = fs::remove_file(file);
        }

        assert!(files_exist)
    }
}

mod chain_create {
    use crate::actions::samples::{batch_create_samplechains, SampleChainOpts};
    use std::array::from_fn;
    use std::env::temp_dir;
    use std::path::PathBuf;

    use crate::audio::wav::WavFile;
    use crate::RBoxErr;

    use ot_tools_lib::read_type_from_bin_file;
    use ot_tools_lib::samples::{
        options::{SampleAttributeTimestrechMode, SampleAttributeTrigQuantizationMode},
        SampleAttributes,
    };

    fn get_base_outpath(test_name: &str) -> PathBuf {
        temp_dir()
            .join("ot-tools-cli")
            .join("sample-chains")
            .join(test_name)
    }

    struct ChainTestResultPaths {
        created_ot_fp: PathBuf,
        created_wav_fp: PathBuf,
        valid_ot_fp: PathBuf,
        valid_wav_fp: PathBuf,
    }

    fn boilerplate_test_sample_chain<const N: usize>(
        sample_chain_opts: Option<SampleChainOpts>,
        extra_name: Option<String>,
    ) -> RBoxErr<Vec<ChainTestResultPaths>> {
        let test_name = if let Some(extra) = extra_name {
            format!["default-{N}-samples-{extra}"]
        } else {
            format!["default-{N}-samples"]
        };
        let test_dir = get_base_outpath(test_name.as_str());
        let _ = std::fs::create_dir_all(&test_dir);

        let dummy_wav_fp = PathBuf::from("../data/tests/samples/chains/create/wav.wav");
        let wav_fps: [PathBuf; N] = from_fn(|_| dummy_wav_fp.clone());

        batch_create_samplechains(
            &wav_fps,
            &test_dir,
            &"chain".to_string(),
            sample_chain_opts,
            None,
            None,
        )?;

        let mut results: Vec<ChainTestResultPaths> = vec![];

        // handling multiple chain file outputs
        for i in 0..(N.div_euclid(64 + 1) + 1) {
            let mut created_ot_fp = test_dir.join(format!["chain-{}", i + 1]);
            let mut created_wav_fp = created_ot_fp.clone();

            created_ot_fp.set_extension("ot");
            created_wav_fp.set_extension("wav");

            let valid_ot_fp = PathBuf::from(
                format![
                    "../data/tests/samples/chains/create/{test_name}/valid-{}.ot",
                    i + 1
                ]
                .as_str(),
            );
            let valid_wav_fp = PathBuf::from(
                format![
                    "../data/tests/samples/chains/create/{test_name}/valid-{}.wav",
                    i + 1
                ]
                .as_str(),
            );

            results.push(ChainTestResultPaths {
                created_ot_fp,
                created_wav_fp,
                valid_ot_fp,
                valid_wav_fp,
            });
        }

        Ok(results)
    }

    fn ok_boilerplate_test_sample_chain<const N: usize>(
        sample_chain_opts: Option<SampleChainOpts>,
        extra_name: Option<String>,
    ) {
        let results = boilerplate_test_sample_chain::<N>(sample_chain_opts, extra_name).unwrap();

        for (idx, r) in results.iter().enumerate() {
            println!("test: Checking chain output idx: {}", idx + 1);
            assert!(r.created_ot_fp.exists());
            assert!(r.created_wav_fp.exists());

            let created_ot = read_type_from_bin_file::<SampleAttributes>(&r.created_ot_fp).unwrap();
            let created_wav = WavFile::from_path(&r.created_wav_fp).unwrap();

            let valid_ot = read_type_from_bin_file::<SampleAttributes>(&r.valid_ot_fp).unwrap();
            let valid_wav = WavFile::from_path(&r.valid_wav_fp).unwrap();

            assert_eq!(valid_ot, created_ot);
            // WavFile types cannot have the `Eq` trait due to samples field
            // being a vector so unfortunately we have to check that the wav
            // samples have been written correctly by checking the samples field
            assert_eq!(created_wav.samples, valid_wav.samples);
        }
    }

    fn err_boilerplate_test_sample_chain<const N: usize>(
        sample_chain_opts: Option<SampleChainOpts>,
        extra_name: Option<String>,
    ) {
        assert!(boilerplate_test_sample_chain::<N>(sample_chain_opts, extra_name).is_err());
    }

    #[test]
    fn ok_default_1_samples() {
        ok_boilerplate_test_sample_chain::<1>(None, None);
    }

    #[test]
    fn ok_tq_1_samples() {
        let x = SampleChainOpts {
            bpm: None,
            gain: None,
            timestretch_mode: None,
            trig_quantization_mode: Some(SampleAttributeTrigQuantizationMode::FourSteps),
            loop_mode: None,
        };
        ok_boilerplate_test_sample_chain::<1>(Some(x), Some("tq".to_string()));
    }

    #[test]
    fn ok_ts_1_samples() {
        let x = SampleChainOpts {
            bpm: None,
            gain: None,
            timestretch_mode: Some(SampleAttributeTimestrechMode::Beat),
            trig_quantization_mode: None,
            loop_mode: None,
        };
        ok_boilerplate_test_sample_chain::<1>(Some(x), Some("ts".to_string()));
    }

    #[test]
    fn ok_gain_1_samples() {
        let x = SampleChainOpts {
            bpm: None,
            gain: Some(12.0),
            timestretch_mode: None,
            trig_quantization_mode: None,
            loop_mode: None,
        };
        ok_boilerplate_test_sample_chain::<1>(Some(x), Some("gain".to_string()));
    }

    #[test]
    fn ok_bpm_1_samples() {
        let x = SampleChainOpts {
            bpm: Some(140.0),
            gain: None,
            timestretch_mode: None,
            trig_quantization_mode: None,
            loop_mode: None,
        };
        ok_boilerplate_test_sample_chain::<1>(Some(x), Some("bpm".to_string()));
    }

    #[test]
    fn err_oob_gain_upper_1_samples() {
        let x = SampleChainOpts {
            bpm: Some(24.1),
            gain: None,
            timestretch_mode: None,
            trig_quantization_mode: None,
            loop_mode: None,
        };
        err_boilerplate_test_sample_chain::<1>(
            Some(x),
            Some("err_oob_gain_upper_1_samples".to_string()),
        );
    }

    #[test]
    fn err_oob_gain_lower_1_samples() {
        let x = SampleChainOpts {
            bpm: Some(-24.1),
            gain: None,
            timestretch_mode: None,
            trig_quantization_mode: None,
            loop_mode: None,
        };
        err_boilerplate_test_sample_chain::<1>(
            Some(x),
            Some("err_oob_gain_upper_1_samples".to_string()),
        );
    }

    #[test]
    fn err_oob_bpm_upper_1_samples() {
        let x = SampleChainOpts {
            bpm: Some(3000.0),
            gain: None,
            timestretch_mode: None,
            trig_quantization_mode: None,
            loop_mode: None,
        };
        err_boilerplate_test_sample_chain::<1>(
            Some(x),
            Some("err_oob_bpm_upper_1_samples".to_string()),
        );
    }

    #[test]
    fn err_oob_bpm_lower_1_samples() {
        let x = SampleChainOpts {
            bpm: Some(5.0),
            gain: None,
            timestretch_mode: None,
            trig_quantization_mode: None,
            loop_mode: None,
        };
        err_boilerplate_test_sample_chain::<1>(
            Some(x),
            Some("err_oob_bpm_lower_1_samples".to_string()),
        );
    }

    #[test]
    fn ok_default_2_samples() {
        ok_boilerplate_test_sample_chain::<2>(None, None);
    }

    #[test]
    fn ok_tq_2_samples() {
        let x = SampleChainOpts {
            bpm: None,
            gain: None,
            timestretch_mode: None,
            trig_quantization_mode: Some(SampleAttributeTrigQuantizationMode::FourSteps),
            loop_mode: None,
        };
        ok_boilerplate_test_sample_chain::<2>(Some(x), Some("tq".to_string()));
    }

    #[test]
    fn ok_ts_2_samples() {
        let x = SampleChainOpts {
            bpm: None,
            gain: None,
            timestretch_mode: Some(SampleAttributeTimestrechMode::Beat),
            trig_quantization_mode: None,
            loop_mode: None,
        };
        ok_boilerplate_test_sample_chain::<2>(Some(x), Some("ts".to_string()));
    }

    #[test]
    fn ok_gain_2_samples() {
        let x = SampleChainOpts {
            bpm: None,
            gain: Some(12.0),
            timestretch_mode: None,
            trig_quantization_mode: None,
            loop_mode: None,
        };
        ok_boilerplate_test_sample_chain::<2>(Some(x), Some("gain".to_string()));
    }

    #[test]
    fn ok_bpm_2_samples() {
        let x = SampleChainOpts {
            bpm: Some(140.0),
            gain: None,
            timestretch_mode: None,
            trig_quantization_mode: None,
            loop_mode: None,
        };
        ok_boilerplate_test_sample_chain::<2>(Some(x), Some("bpm".to_string()));
    }

    #[test]
    fn ok_default_3_samples() {
        ok_boilerplate_test_sample_chain::<3>(None, None);
    }

    #[test]
    fn ok_default_10_samples() {
        ok_boilerplate_test_sample_chain::<10>(None, None);
    }

    #[test]
    fn ok_default_16_samples() {
        ok_boilerplate_test_sample_chain::<16>(None, None);
    }

    #[test]
    fn ok_default_32_samples() {
        ok_boilerplate_test_sample_chain::<32>(None, None);
    }

    #[test]
    fn ok_default_48_samples() {
        ok_boilerplate_test_sample_chain::<48>(None, None);
    }

    #[test]
    fn ok_default_62_samples() {
        ok_boilerplate_test_sample_chain::<62>(None, None);
    }

    #[test]
    fn ok_default_63_samples() {
        ok_boilerplate_test_sample_chain::<63>(None, None);
    }

    // specific weirdness related to checksum values
    #[test]
    fn ok_default_64_samples() {
        ok_boilerplate_test_sample_chain::<64>(None, None);
    }

    #[test]
    fn ok_default_65_samples() {
        ok_boilerplate_test_sample_chain::<65>(None, None);
    }

    #[test]
    fn ok_default_127_samples() {
        ok_boilerplate_test_sample_chain::<127>(None, None);
    }

    #[test]
    fn ok_default_256_samples() {
        ok_boilerplate_test_sample_chain::<256>(None, None);
    }

    mod slices_from_wavs {

        use crate::actions::samples::create_slices_from_wavfiles;
        use crate::audio::wav::WavFile;
        use std::path::PathBuf;

        #[test]
        fn no_offset_ok() {
            let fp = PathBuf::from("../data/tests/misc/test.wav");
            let wav = WavFile::from_path(&fp).unwrap();
            let wavs = [
                wav.clone(),
                wav.clone(),
                wav.clone(),
                wav.clone(),
                wav.clone(),
            ]
            .to_vec();

            assert!(create_slices_from_wavfiles(&wavs, 0).is_ok())
        }

        #[test]
        fn offset_100_ok() {
            let fp = PathBuf::from("../data/tests/misc/test.wav");
            let wav = WavFile::from_path(&fp).unwrap();
            let wavs = [
                wav.clone(),
                wav.clone(),
                wav.clone(),
                wav.clone(),
                wav.clone(),
            ]
            .to_vec();

            assert!(create_slices_from_wavfiles(&wavs, 100).is_ok())
        }

        #[test]
        fn offset_30000_ok() {
            let fp = PathBuf::from("../data/tests/misc/test.wav");
            let wav = WavFile::from_path(&fp).unwrap();
            let wavs = [
                wav.clone(),
                wav.clone(),
                wav.clone(),
                wav.clone(),
                wav.clone(),
            ]
            .to_vec();

            assert!(create_slices_from_wavfiles(&wavs, 30000).is_ok())
        }
    }
}

mod indexing {
    use crate::actions::samples::{create_index_samples_dir_full, create_index_samples_dir_simple};
    use std::path::PathBuf;

    #[test]
    fn simple_no_yaml_ok() {
        let dirpath = PathBuf::from("../data/tests/samples/indexing/");
        let r = create_index_samples_dir_simple(&dirpath, &None);
        assert!(r.is_ok())
    }

    #[test]
    fn full_no_yaml_ok() {
        let dirpath = PathBuf::from("../data/tests/samples/indexing/");
        let r = create_index_samples_dir_full(&dirpath, &None);
        assert!(r.is_ok())
    }

    #[test]
    fn simple_with_yaml_ok() {
        let yamlpath = std::env::temp_dir().join("test-samples-search-simple.yaml");
        let dirpath = PathBuf::from("../data/tests/samples/indexing/");
        let r = create_index_samples_dir_simple(&dirpath, &Some(yamlpath.clone()));

        let _ = std::fs::remove_file(yamlpath);
        assert!(r.is_ok())
    }

    #[test]
    fn full_with_yaml_ok() {
        let yamlpath = std::env::temp_dir().join("test-samples-search-full.yaml");
        let dirpath = PathBuf::from("../data/tests/samples/indexing/");
        let r = create_index_samples_dir_full(&dirpath, &Some(yamlpath.clone()));

        let _ = std::fs::remove_file(yamlpath);
        assert!(r.is_ok())
    }

    // fails as paths in the target yaml are linux only
    #[cfg(not(target_os = "windows"))]
    #[test]
    fn simple_with_yaml_matches_validation() {
        use crate::actions::samples::SamplesDirIndexSimple;
        use ot_tools_lib::yaml_file_to_type;

        let testpath = PathBuf::from("../data/tests/samples/indexing/simple-valid.yaml");
        let outpath = std::env::temp_dir().join("test-samples-search-simple-validate.yaml");
        let dirpath = PathBuf::from("../data/tests/samples/indexing/");
        let _ = create_index_samples_dir_simple(&dirpath, &Some(outpath.clone()));

        let valid = yaml_file_to_type::<SamplesDirIndexSimple>(&testpath).unwrap();
        let written = yaml_file_to_type::<SamplesDirIndexSimple>(&outpath).unwrap();

        let _ = std::fs::remove_file(outpath);
        assert_eq!(written, valid)
    }

    // fails as paths in the target yaml are linux only
    #[cfg(not(target_os = "windows"))]
    #[test]
    // #[ignore]
    fn full_with_yaml_matches_validation() {
        use crate::actions::samples::SamplesDirIndexFull;
        use ot_tools_lib::yaml_file_to_type;

        let testpath = PathBuf::from("../data/tests/samples/indexing/full-valid.yaml");
        let outpath = std::env::temp_dir().join("test-samples-search-full-validate.yaml");
        let dirpath = PathBuf::from("../data/tests/samples/indexing/");
        let _ = create_index_samples_dir_full(&dirpath, &Some(outpath.clone()));

        let valid = yaml_file_to_type::<SamplesDirIndexFull>(&testpath).unwrap();
        let written = yaml_file_to_type::<SamplesDirIndexFull>(&outpath).unwrap();

        let _ = std::fs::remove_file(outpath);
        assert_eq!(written, valid)
    }
}
