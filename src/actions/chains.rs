//! Functions for CLI actions related to chaining samples into sliced sample chains.

mod yaml;

use log::{debug, info, trace};
use rand::Rng;
use std::path::PathBuf;

use serde_octatrack::{
    samples::{
        configs::{SampleLoopConfig, SampleTrimConfig},
        options::{
            SampleAttributeLoopMode, SampleAttributeTimestrechMode,
            SampleAttributeTrigQuantizationMode,
        },
        slices::{Slice, Slices},
        SampleAttributes,
    },
    FromPath, ToPath,
};

use crate::{
    audio::wav::WavFile,
    common::{FromYamlFile, RBoxErr},
    utils::{
        create_slices_from_wavfiles, get_otsample_nbars_from_wavfile,
        get_otsample_nbars_from_wavfiles,
    },
};

use yaml::{create::YamlChainCreate, deconstruct::YamlChainDeconstruct};

/// Chain together a wav sample vector into individual wav file(s).
///
/// Each individual output can have a maximum of 64 samples,
/// so results are batched up with a max size of 64.
// TODO: this needs to return a hashmap so the yaml chain generator can
//       read the underlying sample information for a batch.
// TODO: Split this up into two functions --> wavfile_vec_to_batch64 and wavfile_batch64_to_wavfile
// TODO: Looks like there's a new struct, or a new datatype there...
pub fn chain_wavfiles_64_batch(
    wavfiles: &Vec<WavFile>,
) -> Result<Vec<(WavFile, Vec<WavFile>)>, ()> {
    debug!("Batching {:#?} audio files.", wavfiles.len());

    let vec_mod_length = wavfiles.len().div_euclid(64);

    trace!("Creating sample chain audio file batches.");
    let slice_vecs: Vec<Vec<WavFile>> = (0..(vec_mod_length + 1))
        .map(|i| {
            let (start, mut end) = (i * 64, (i * 64) + 64);
            if end > wavfiles.len() {
                end = wavfiles.len();
            };
            wavfiles[start..end].to_vec()
        })
        .collect();

    trace!("Creating singular sample of samples in each batch.");
    let mut chains: Vec<(WavFile, Vec<WavFile>)> = vec![];

    for slice_vec in slice_vecs {
        let mut single_chain_wav: WavFile = slice_vec[0].clone();

        for wavfile in slice_vec[1..].iter() {
            for s in &wavfile.samples {
                single_chain_wav.samples.push(*s);
            }
            single_chain_wav.len += wavfile.len;
        }
        chains.push((single_chain_wav, slice_vec));
    }

    info!(
        "Batched {:#?} audio files into {:#?} chains.",
        wavfiles.len(),
        chains.len()
    );

    Ok(chains)
}

/// Create Octatrack samplechain file-pairs from a loaded yaml config.

pub fn create_samplechains_from_yaml(yaml_conf_fpath: &PathBuf) -> RBoxErr<()> {
    let chain_conf = YamlChainCreate::from_yaml(yaml_conf_fpath)
        .unwrap_or_else(|_| panic!("Could not load yaml file: path={yaml_conf_fpath:#?}"));

    info!("Creating sample chains from yaml config.");
    trace!("Yaml contents: {chain_conf:#?}");

    for chain_config in &chain_conf.chains {
        info!("Creating chain: name={:#?}", &chain_config.chain_name);
        info!(
            "Getting wav files: n={:#?}",
            &chain_config.sample_file_paths.len()
        );

        create_samplechain_from_pathbufs_only(
            &chain_config.sample_file_paths,
            &chain_conf.global_settings.out_dir_path,
            &chain_config.chain_name,
        )
        .unwrap_or_else(|_| {
            panic!(
                "Could not generate sample chain: name={:#?}",
                &chain_config.chain_name
            )
        });
    }

    Ok(())
}

/// Create 64 length sample chains and write out the files.

pub fn create_samplechain_from_pathbufs_only(
    wav_fps: &Vec<PathBuf>,
    outdir_path: &PathBuf,
    outchain_name: &String,
) -> RBoxErr<()> {
    let wavfiles: Vec<WavFile> = wav_fps
        .iter()
        .map(|fp: &PathBuf| {
            WavFile::from_path(fp)
                .unwrap_or_else(|_| panic!("Could not read wav file: path={fp:#?}"))
        })
        .collect();

    let wavfiles_batched: Vec<(WavFile, Vec<WavFile>)> =
        chain_wavfiles_64_batch(&wavfiles).expect("Error creating batches of audio files!");

    for (idx, (single_wav, vec_wavs)) in wavfiles_batched.iter().enumerate() {
        trace!("Making slices: {} / {}", idx + 1, wavfiles_batched.len());
        let slices = create_slices_from_wavfiles(vec_wavs, 0)
            .unwrap_or_else(|_| panic!("Could not create sample chain slices: idx={idx:#?}"));

        trace!(
            "Calculating bar length: {} / {}",
            idx + 1,
            wavfiles_batched.len()
        );
        // TODO -- can use single wavfile here?! would make the funtion more generally applicable.
        let bars = get_otsample_nbars_from_wavfiles(vec_wavs, &125.0).unwrap();

        trace!(
            "Setting up sample attributes data: {} / {}",
            idx + 1,
            wavfiles_batched.len()
        );
        let trim_config = SampleTrimConfig {
            start: 0,
            end: single_wav.len,
            length: bars,
        };

        let loop_config = SampleLoopConfig {
            start: 0,
            length: bars,
            mode: SampleAttributeLoopMode::default(),
        };

        let chain_data = SampleAttributes::new(
            &120.0,
            &SampleAttributeTimestrechMode::default(),
            &SampleAttributeTrigQuantizationMode::default(),
            &0.0,
            &trim_config,
            &loop_config,
            &slices,
        )
        .unwrap_or_else(|_| {
            panic!("Could not create sample attributes data for sample chain: idx={idx:#?}")
        });

        trace!(
            "Modifying file paths: {} / {}",
            idx + 1,
            wavfiles_batched.len()
        );
        let base_outchain_path = outdir_path.join(outchain_name);

        let mut wav_sliced_outpath = base_outchain_path;
        wav_sliced_outpath.set_extension("wav");
        single_wav.to_path(&wav_sliced_outpath).unwrap_or_else(|_| {
            panic!(
                "Could not write sample chain wav file: idx={idx:#?} path={wav_sliced_outpath:#?}"
            )
        });
        info!("Creating chain audio file: {wav_sliced_outpath:#?}");

        let mut ot_outpath = wav_sliced_outpath.clone();
        ot_outpath.set_extension("ot");
        chain_data.to_path(&ot_outpath).unwrap_or_else(|_| {
            panic!(
                "Could not write sample chain attributes file: idx={idx:#?} path={ot_outpath:#?}"
            )
        });
        info!("Created chain attributes file: {ot_outpath:#?}");
    }
    info!("Created sample chain: name={outchain_name:#?}");
    Ok(())
}

// todo: needs tests
/// Extract a slices from a sliced sample chain into individual samples.
pub fn deconstruct_samplechain_from_pathbufs_only(
    audio_fpath: &PathBuf,
    attributes_fpath: &PathBuf,
    out_dirpath: &PathBuf,
) -> RBoxErr<Vec<PathBuf>> {
    if !out_dirpath.is_dir() {
        panic!("Output dirpath argument is not a directory. Must be a directory.");
    }

    let wavfile = WavFile::from_path(audio_fpath).expect("Could not read wavfile.");
    let attrs = SampleAttributes::from_path(attributes_fpath).unwrap_or_else(|_| {
        panic!("Could not read `.ot` attributes file: path={attributes_fpath:#?}")
    });
    // todo: this feels fragile
    let base_sample_fname = audio_fpath
        .file_stem()
        .unwrap_or(std::ffi::OsStr::new("deconstructed_samplechain"))
        .to_str()
        .unwrap_or("deconstructed_samplechain");

    let mut out_fpaths: Vec<PathBuf> = vec![];

    for i in 0..attrs.slices_len {
        let slice = attrs.slices[i as usize];
        let w = wavfile.samples[(slice.trim_start as usize)..(slice.trim_end as usize)].to_vec();

        let wavslice = WavFile {
            spec: wavfile.spec,
            len: slice.trim_end - slice.trim_end,
            samples: w,
            file_path: PathBuf::from("/tmp/dummy.wav"),
        };

        let sample_fname = format!("{base_sample_fname}_{i:#?}");
        let mut out_fpath = out_dirpath.clone().join(sample_fname);
        out_fpath.set_extension("wav");

        wavslice
            .to_path(&out_fpath)
            .unwrap_or_else(|_| panic!("Could not write slice to wavfile: path={out_fpath:#?}"));

        out_fpaths.push(out_fpath);
    }
    Ok(out_fpaths)
}

pub fn deconstruct_samplechains_from_yaml(yaml_conf_fpath: &PathBuf) -> RBoxErr<()> {
    let chain_conf = YamlChainDeconstruct::from_yaml(yaml_conf_fpath)
        .unwrap_or_else(|_| panic!("Could not load yaml file: path={yaml_conf_fpath:#?}"));

    info!("Deconstructing sample chains from yaml config.");
    trace!("Yaml contents: {chain_conf:#?}");

    for chain_config in &chain_conf.chains {
        deconstruct_samplechain_from_pathbufs_only(
            &chain_config.sample,
            &chain_config.otfile,
            &chain_conf.global_settings.out_dir_path,
        )
        .unwrap_or_else(|_| {
            panic!(
                "Could not deconstruct sample chain: sample={:#?} otfile={:#?}",
                &chain_config.sample, &chain_config.otfile
            )
        });
    }

    Ok(())
}

/// Given a wavfile, create Nx random slices stored in a sample attributes file.
pub fn create_randomly_sliced_sample(wav_fp: &PathBuf, n_slices: usize) -> RBoxErr<()> {
    if n_slices > 64 {
        panic!("Maximum number of slices in a sample file is 64.");
    };

    let wavfile = WavFile::from_path(wav_fp).expect("Could not read wav file.");

    if wavfile.len < 64 {
        panic!("Wav file too short, needs to be at least 64 samples in length.");
    };

    let mut rng = rand::thread_rng();

    let default_slice = Slice {
        trim_end: 0,
        trim_start: 0,
        loop_start: 0,
    };

    let mut slices_arr: [Slice; 64] = [default_slice; 64];

    for i in 0..n_slices {
        let trim_start: u32 = rng.gen_range(0..=(wavfile.len - 64));
        let trim_end: u32 = rng.gen_range(trim_start..=wavfile.len);
        let loop_start: u32 = 0xFFFFFFFF;

        let slice = Slice {
            trim_start,
            trim_end,
            loop_start,
        };

        slices_arr[i] = slice;
    }

    let slices = Slices {
        slices: slices_arr,
        count: n_slices as u32,
    };

    let bars = get_otsample_nbars_from_wavfile(&wavfile, &120.0)?;

    let trim_config = SampleTrimConfig {
        start: 0,
        end: wavfile.len,
        length: bars,
    };

    let loop_config = SampleLoopConfig {
        start: 0,
        length: bars,
        mode: SampleAttributeLoopMode::default(),
    };

    let chain_data = SampleAttributes::new(
        &120.0,
        &SampleAttributeTimestrechMode::default(),
        &SampleAttributeTrigQuantizationMode::default(),
        &0.0,
        &trim_config,
        &loop_config,
        &slices,
    )
    .expect("Could not create sample attributes data for sample chain.");

    let mut ot_outpath = wav_fp.clone();
    ot_outpath.set_extension("ot");

    chain_data.to_path(&ot_outpath).unwrap_or_else(|_| {
        panic!("Could not write sample chain attributes file: path={ot_outpath:#?}")
    });
    info!("Created chain attributes file: {ot_outpath:#?}");
    Ok(())
}

/// Given a wavfile, create Nx equal length slices stored in a sample attributes file.
pub fn create_equally_sliced_sample(wav_fp: &PathBuf, n_slices: usize) -> RBoxErr<()> {
    if n_slices > 64 {
        panic!("Maximum number of slices in a sample file is 64.");
    };

    let wavfile = WavFile::from_path(wav_fp).expect("Could not read wav file.");

    if wavfile.len < 64 {
        panic!("Wav file too short, needs to be at least 64 samples in length.");
    };

    let default_slice = Slice {
        trim_end: 0,
        trim_start: 0,
        loop_start: 0,
    };
    let mut slices_arr: [Slice; 64] = [default_slice; 64];
    let len = wavfile.len / (n_slices as u32);

    for i in 0..n_slices {
        let trim_start: u32 = (i as u32) * len;
        let trim_end: u32 = trim_start + len;
        let loop_start: u32 = 0xFFFFFFFF;

        let slice = Slice {
            trim_start,
            trim_end,
            loop_start,
        };

        slices_arr[i] = slice;
    }

    let slices = Slices {
        slices: slices_arr,
        count: n_slices as u32,
    };

    let bars = get_otsample_nbars_from_wavfile(&wavfile, &120.0)?;

    let trim_config = SampleTrimConfig {
        start: 0,
        end: wavfile.len,
        length: bars,
    };

    let loop_config = SampleLoopConfig {
        start: 0,
        length: bars,
        mode: SampleAttributeLoopMode::default(),
    };

    let chain_data = SampleAttributes::new(
        &120.0,
        &SampleAttributeTimestrechMode::default(),
        &SampleAttributeTrigQuantizationMode::default(),
        &0.0,
        &trim_config,
        &loop_config,
        &slices,
    )
    .expect("Could not create sample attributes data for sample chain.");

    let mut ot_outpath = wav_fp.clone();
    ot_outpath.set_extension("ot");

    chain_data.to_path(&ot_outpath).unwrap_or_else(|_| {
        panic!("Could not write sample chain attributes file: path={ot_outpath:#?}")
    });
    info!("Created chain attributes file: {ot_outpath:#?}");
    Ok(())
}

/// Use input files from `resouces/test-data/` to create an OT file output
/// and compare it to what should exist.
/// Read relevant WAV files, create an OT file of some description, write
/// the OT file then compare it to the known good output from OctaChainer.

#[cfg(test)]
mod tests {

    mod chain_deconstruct {

        use crate::actions::chains::deconstruct_samplechain_from_pathbufs_only;
        use std::{fs, path::PathBuf};
        use walkdir::{DirEntry, WalkDir};

        #[test]
        fn test_basic() {
            let audio_fpath = PathBuf::from("data/tests/chains/deconstruct/test.wav");
            let attributes_fpath = PathBuf::from("data/tests/chains/deconstruct/test.ot");
            let outdir = PathBuf::from("/tmp/");

            let res = deconstruct_samplechain_from_pathbufs_only(
                &audio_fpath,
                &attributes_fpath,
                &outdir,
            );

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

        use std::path::PathBuf;
        use walkdir::{DirEntry, WalkDir};

        use crate::common::RBoxErr;

        use crate::audio::wav::WavFile;
        use serde_octatrack::FromPath;

        use serde_octatrack::samples::{
            configs::{SampleLoopConfig, SampleTrimConfig},
            options::{
                SampleAttributeLoopMode, SampleAttributeTimestrechMode,
                SampleAttributeTrigQuantizationMode,
            },
            slices::{Slice, Slices},
            SampleAttributes,
        };

        use crate::utils::{create_slices_from_wavfiles, get_otsample_nbars_from_wavfiles};

        fn walkdir_filter_is_wav(entry: &DirEntry) -> bool {
            entry
                .file_name()
                .to_str()
                .map(|s| s.ends_with(".wav"))
                .unwrap_or(false)
        }

        fn get_test_wav_paths(path: &str) -> RBoxErr<Vec<PathBuf>> {
            let paths_iter: _ = WalkDir::new(path)
                .sort_by_file_name()
                .max_depth(1)
                .min_depth(1)
                .into_iter()
                .filter_entry(|e| walkdir_filter_is_wav(e));

            let mut fpaths: Vec<PathBuf> = Vec::new();
            for entry in paths_iter {
                let unwrapped = entry.unwrap();
                let fpath = unwrapped.path().to_path_buf();
                fpaths.push(fpath);
            }

            Ok(fpaths)
        }

        fn create_sample_chain_encoded_from_wavfiles(
            wav_fps: Vec<PathBuf>,
        ) -> RBoxErr<(SampleLoopConfig, SampleTrimConfig, Slices)> {
            let mut wavs: Vec<WavFile> = Vec::new();
            for fp in wav_fps {
                let wav = WavFile::from_path(&fp).unwrap();
                wavs.push(wav);
            }

            let slices_config = create_slices_from_wavfiles(&wavs, 0).unwrap();

            let bars = get_otsample_nbars_from_wavfiles(&wavs, &125.0).unwrap();

            let trim_config = SampleTrimConfig {
                start: 0,
                end: wavs.iter().map(|x| x.len as u32).sum(),
                length: bars,
            };

            let loop_config = SampleLoopConfig {
                start: 0,
                length: bars,
                mode: SampleAttributeLoopMode::Off,
            };

            Ok((loop_config, trim_config, slices_config))
        }

        fn read_valid_sample_chain(path: &PathBuf) -> RBoxErr<SampleAttributes> {
            let read_chain = SampleAttributes::from_path(path).unwrap();
            Ok(read_chain)
        }

        #[test]
        fn test_default_10_samples() {
            let wav_fps = get_test_wav_paths("data/tests/1/wavs/").unwrap();
            let (loop_config, trim_config, slices) =
                create_sample_chain_encoded_from_wavfiles(wav_fps).unwrap();

            let composed_chain_res = SampleAttributes::new(
                &125.0,
                &SampleAttributeTimestrechMode::Off,
                &SampleAttributeTrigQuantizationMode::PatternLength,
                &-24.0,
                &trim_config,
                &loop_config,
                &slices,
            );

            let valid_ot_fp = PathBuf::from("data/tests/1/chain.ot");
            let valid_sample_chain = read_valid_sample_chain(&valid_ot_fp).unwrap();

            assert_eq!(
                composed_chain_res.unwrap().encode().unwrap(),
                valid_sample_chain.encode().unwrap(),
            );
        }

        #[test]
        fn test_default_3_samples() {
            let wav_fps = get_test_wav_paths("data/tests/2/wavs/").unwrap();
            let (loop_config, trim_config, slices) =
                create_sample_chain_encoded_from_wavfiles(wav_fps).unwrap();

            let composed_chain = SampleAttributes::new(
                &125.0,
                &SampleAttributeTimestrechMode::Off,
                &SampleAttributeTrigQuantizationMode::PatternLength,
                &-24.0,
                &trim_config,
                &loop_config,
                &slices,
            )
            .unwrap();

            let valid_ot_fp = PathBuf::from("data/tests/2/chain.ot");
            let valid_sample_chain = read_valid_sample_chain(&valid_ot_fp).unwrap();

            assert_eq!(
                composed_chain.encode().unwrap(),
                valid_sample_chain.encode().unwrap(),
            );
        }

        #[ignore]
        #[test]
        fn test_default_64_samples() {
            let wav_fps = get_test_wav_paths("data/tests/3/wavs/").unwrap();
            let (loop_config, trim_config, slices) =
                create_sample_chain_encoded_from_wavfiles(wav_fps).unwrap();

            let composed_chain = SampleAttributes::new(
                &175.0,
                &SampleAttributeTimestrechMode::Off,
                &SampleAttributeTrigQuantizationMode::PatternLength,
                &24.0,
                &trim_config,
                &loop_config,
                &slices,
            )
            .unwrap();

            let valid_ot_fp = PathBuf::from("data/tests/3/chain.ot");
            let valid_sample_chain = read_valid_sample_chain(&valid_ot_fp).unwrap();

            assert_eq!(composed_chain, valid_sample_chain,);

            assert_eq!(
                composed_chain.encode().unwrap(),
                valid_sample_chain.encode().unwrap(),
            );
        }

        // how to handle > 64 samples
        #[ignore]
        #[test]
        fn test_default_67_samples() {
            let wav_fps = get_test_wav_paths("data/tests/3/wavs/").unwrap();
            let (loop_config, trim_config, slices) =
                create_sample_chain_encoded_from_wavfiles(wav_fps).unwrap();

            let composed_chain = SampleAttributes::new(
                &175.0,
                &SampleAttributeTimestrechMode::Off,
                &SampleAttributeTrigQuantizationMode::PatternLength,
                &24.0,
                &trim_config,
                &loop_config,
                &slices,
            )
            .unwrap();

            let valid_ot_fp = PathBuf::from("data/tests/3/chain.ot");
            let valid_sample_chain = read_valid_sample_chain(&valid_ot_fp).unwrap();

            assert_eq!(composed_chain, valid_sample_chain,);

            assert_eq!(
                composed_chain.encode().unwrap(),
                valid_sample_chain.encode().unwrap(),
            );
        }

        fn create_mock_configs_blank() -> (SampleTrimConfig, SampleLoopConfig, Slices) {
            let trim_config = SampleTrimConfig {
                start: 0,
                end: 0,
                length: 0,
            };

            let loop_config = SampleLoopConfig {
                start: 0,
                length: 0,
                mode: SampleAttributeLoopMode::Normal,
            };

            let default_slice = Slice {
                trim_start: 0,
                trim_end: 0,
                loop_start: 0,
            };

            let slices: [Slice; 64] = [default_slice; 64];

            let slice_conf = Slices {
                slices: slices,
                count: 0,
            };

            (trim_config, loop_config, slice_conf)
        }

        #[ignore]
        #[test]
        fn test_non_default_tempo_3_samples() {
            let (trim_conf, loop_conf, slices) = create_mock_configs_blank();

            let composed_chain = SampleAttributes::new(
                &147.0,
                &SampleAttributeTimestrechMode::Off,
                &SampleAttributeTrigQuantizationMode::PatternLength,
                &0.0,
                &trim_conf,
                &loop_conf,
                &slices,
            );

            assert!(composed_chain.is_err());
        }

        #[ignore]
        #[test]
        fn test_non_default_quantize_3_samples() {
            let wav_fps = get_test_wav_paths("data/tests/3/wavs/").unwrap();
            let (loop_config, trim_config, slices) =
                create_sample_chain_encoded_from_wavfiles(wav_fps).unwrap();

            let composed_chain = SampleAttributes::new(
                &125.0,
                &SampleAttributeTimestrechMode::Off,
                &SampleAttributeTrigQuantizationMode::PatternLength,
                &0.0,
                &trim_config,
                &loop_config,
                &slices,
            )
            .unwrap();

            let valid_ot_fp = PathBuf::from("data/tests/3/chain.ot");
            let valid_sample_chain = read_valid_sample_chain(&valid_ot_fp).unwrap();

            assert_eq!(composed_chain, valid_sample_chain,);

            assert_eq!(
                composed_chain.encode().unwrap(),
                valid_sample_chain.encode().unwrap(),
            );
        }

        #[ignore]
        #[test]
        fn test_non_default_gain_3_samples() {
            let wav_fps = get_test_wav_paths("data/tests/3/wavs/").unwrap();
            let (loop_config, trim_config, slices) =
                create_sample_chain_encoded_from_wavfiles(wav_fps).unwrap();

            let composed_chain = SampleAttributes::new(
                &125.0,
                &SampleAttributeTimestrechMode::Off,
                &SampleAttributeTrigQuantizationMode::PatternLength,
                &24.0,
                &trim_config,
                &loop_config,
                &slices,
            )
            .unwrap();

            let valid_ot_fp = PathBuf::from("data/tests/3/chain.ot");
            let valid_sample_chain = read_valid_sample_chain(&valid_ot_fp).unwrap();

            assert_eq!(composed_chain, valid_sample_chain,);

            assert_eq!(
                composed_chain.encode().unwrap(),
                valid_sample_chain.encode().unwrap(),
            );

            assert_eq!(composed_chain, valid_sample_chain);
        }

        #[test]
        fn test_oob_tempo() {
            let (trim_conf, loop_conf, slices) = create_mock_configs_blank();

            let composed_chain = SampleAttributes::new(
                &10000.0,
                &SampleAttributeTimestrechMode::Off,
                &SampleAttributeTrigQuantizationMode::PatternLength,
                &0.0,
                &trim_conf,
                &loop_conf,
                &slices,
            );

            assert!(composed_chain.is_err());
        }

        #[test]
        fn test_invalid_gain() {
            let (trim_conf, loop_conf, slices) = create_mock_configs_blank();

            let composed_chain = SampleAttributes::new(
                &125.0,
                &SampleAttributeTimestrechMode::Off,
                &SampleAttributeTrigQuantizationMode::PatternLength,
                &300.0,
                &trim_conf,
                &loop_conf,
                &slices,
            );

            assert!(composed_chain.is_err());
        }
    }
}
