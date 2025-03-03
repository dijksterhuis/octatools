//! Functions for CLI actions related to chaining samples into sliced sample chains.

mod yaml;

use log::{debug, info, trace};
use octatools_lib::samples::{
    configs::{SampleLoopConfig, SampleTrimConfig},
    options::{
        SampleAttributeLoopMode, SampleAttributeTimestrechMode, SampleAttributeTrigQuantizationMode,
    },
    slices::{Slice, Slices},
    SampleAttributes, SampleAttributesRawBytes,
};
use rand::Rng;
use std::array::from_fn;
use std::path::{Path, PathBuf};

use crate::{
    audio::wav::WavFile,
    utils::{
        create_slices_from_wavfiles, get_otsample_nbars_from_wavfile,
        get_otsample_nbars_from_wavfiles,
    },
    RBoxErr,
};

use octatools_lib::{
    get_bytes_slice, read_type_from_bin_file, type_to_yaml_file, write_type_to_bin_file,
    yaml_file_to_type,
};
use yaml::{
    create::YamlChainCreate,
    deconstruct::YamlChainDeconstruct,
    samplesdir::{SamplesDirIndexFull, SamplesDirIndexSimple},
};

/// Show bytes output as u8 values for a Sample Attributes file located at `path`
pub fn show_ot_file_bytes(
    path: &Path,
    start_idx: &Option<usize>,
    len: &Option<usize>,
) -> RBoxErr<()> {
    let raw =
        read_type_from_bin_file::<SampleAttributesRawBytes>(path).expect("Could not load ot file");

    let bytes = get_bytes_slice(raw.data.to_vec(), start_idx, len);
    println!("{:#?}", bytes);
    Ok(())
}

// todo: tests
/// Create a default OctaTrack sample attributes file for some wav file
pub fn create_default_ot_file_for_wav_file(path: &Path) -> RBoxErr<()> {
    let mut ot_path = path.to_path_buf();
    ot_path.set_extension("ot");

    let wavfile = WavFile::from_path(path)?;

    let ot_data = SampleAttributes::new(
        &120.0,
        &SampleAttributeTimestrechMode::default(),
        &SampleAttributeTrigQuantizationMode::default(),
        &0.0,
        &SampleTrimConfig {
            start: 0,
            end: wavfile.len,
            length: wavfile.len,
        },
        &SampleLoopConfig {
            start: 0,
            length: wavfile.len,
            mode: SampleAttributeLoopMode::default(),
        },
        &Slices {
            slices: from_fn(|_| Slice {
                trim_end: 0,
                trim_start: 0,
                loop_start: 0,
            }),
            count: 0,
        },
    );

    type_to_yaml_file::<SampleAttributes>(&ot_data?, &ot_path)?;

    Ok(())
}

// todo: test
// todo: better error handling
/// Create Nx default OctaTrack sample attributes file for Nx wav files
pub fn create_default_ot_files_for_wav_files(paths: &[PathBuf]) -> RBoxErr<()> {
    for path in paths {
        create_default_ot_file_for_wav_file(path).expect("Failed to create an ot file");
    }
    Ok(())
}

/// Chain together a wav sample vector into individual wav file(s).
///
/// Each individual output can have a maximum of 64 samples,
/// so results are batched up with a max size of 64.
pub fn chain_wavfiles_64_batch(wavfiles: &[WavFile]) -> Result<Vec<(WavFile, Vec<WavFile>)>, ()> {
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
pub fn create_samplechains_from_yaml(yaml_conf_fpath: &Path) -> RBoxErr<()> {
    let chain_conf = yaml_file_to_type::<YamlChainCreate>(yaml_conf_fpath)
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
    wav_fps: &[PathBuf],
    outdir_path: &Path,
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

        // always suffix the batch number onto the same chain
        // if someone generates a sample chain with 64 files the first time,
        // then attempts to create a second version later on with 65 files then
        // they'll end up with different file names and have to manually load the
        // samples to every slot where they've been used.
        //
        // example use-case: users creating "godchains" containing all their
        // favourite samples. If they start with <64 samples, subsequently
        // adding more and regenerating their "godchain" will mean they have to
        // edit the existing sample slot in all their projects.
        //
        // so just always suffix `-{idx}` to the filenames.
        let base_outchain_path =
            outdir_path
                .to_path_buf()
                .join(format!["{}-{}", outchain_name, idx + 1]);

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

        write_type_to_bin_file::<SampleAttributes>(&chain_data, &ot_outpath).unwrap_or_else(|_| {
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
pub fn deconstruct_samplechain_from_paths(
    audio_fpath: &Path,
    attributes_fpath: &Path,
    out_dirpath: &Path,
) -> RBoxErr<Vec<PathBuf>> {
    if !out_dirpath.is_dir() {
        panic!("Output dirpath argument is not a directory. Must be a directory.");
    }

    let wavfile = WavFile::from_path(audio_fpath).expect("Could not read wavfile.");

    let attrs =
        read_type_from_bin_file::<SampleAttributes>(attributes_fpath).unwrap_or_else(|_| {
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
        let wav_len = slice.trim_end - slice.trim_start;

        let wavslice = WavFile {
            spec: wavfile.spec,
            len: wav_len,
            samples: w,
            file_path: std::env::temp_dir().join("dummy.wav"),
        };

        let sample_fname = format!("{base_sample_fname}_{i:#?}");
        let mut out_fpath = out_dirpath.to_path_buf().join(sample_fname);
        out_fpath.set_extension("wav");

        wavslice
            .to_path(&out_fpath)
            .unwrap_or_else(|_| panic!("Could not write slice to wavfile: path={out_fpath:#?}"));

        out_fpaths.push(out_fpath);
    }
    Ok(out_fpaths)
}

pub fn deconstruct_samplechains_from_yaml(yaml_conf_fpath: &PathBuf) -> RBoxErr<()> {
    let chain_conf = yaml_file_to_type::<YamlChainDeconstruct>(yaml_conf_fpath)
        .unwrap_or_else(|_| panic!("Could not load yaml file: path={yaml_conf_fpath:#?}"));

    info!("Deconstructing sample chains from yaml config.");
    trace!("Yaml contents: {chain_conf:#?}");

    for chain_config in &chain_conf.chains {
        deconstruct_samplechain_from_paths(
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
pub fn create_randomly_sliced_sample(wav_fp: &Path, n_slices: usize) -> RBoxErr<()> {
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

    #[allow(clippy::needless_range_loop)]
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

    let mut ot_outpath = wav_fp.to_path_buf();
    ot_outpath.set_extension("ot");

    write_type_to_bin_file::<SampleAttributes>(&chain_data, &ot_outpath).unwrap_or_else(|_| {
        panic!("Could not write sample chain attributes file: path={ot_outpath:#?}")
    });
    info!("Created chain attributes file: {ot_outpath:#?}");
    Ok(())
}

/// Given a wavfile, create Nx equal length slices stored in a sample attributes file.
pub fn create_equally_sliced_sample(wav_fp: &Path, n_slices: usize) -> RBoxErr<()> {
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

    #[allow(clippy::needless_range_loop)]
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

    let mut ot_outpath = wav_fp.to_path_buf();
    ot_outpath.set_extension("ot");

    write_type_to_bin_file::<SampleAttributes>(&chain_data, &ot_outpath).unwrap_or_else(|_| {
        panic!("Could not write sample chain attributes file: path={ot_outpath:#?}")
    });
    info!("Created chain attributes file: {ot_outpath:#?}");
    Ok(())
}

pub fn create_index_samples_dir_simple(
    samples_dir_path: &PathBuf,
    yaml_file_path: &Option<PathBuf>,
) -> RBoxErr<()> {
    debug!("Indexing samples directory with 'simple' output: path={samples_dir_path:#?}");
    let sample_index = SamplesDirIndexSimple::new(samples_dir_path)?;

    if !yaml_file_path.is_none() {
        type_to_yaml_file(&sample_index, yaml_file_path.as_ref().unwrap())
            .expect("Could not write yaml file.");
    }

    Ok(())
}

pub fn create_index_samples_dir_full(
    samples_dir_path: &PathBuf,
    yaml_file_path: &Option<PathBuf>,
) -> RBoxErr<()> {
    let sample_index = SamplesDirIndexFull::new(samples_dir_path)?;

    if !yaml_file_path.is_none() {
        type_to_yaml_file(&sample_index, yaml_file_path.as_ref().unwrap())
            .expect("Could not write yaml file.");
    }
    Ok(())
}

/// Use input files from `resouces/test-data/` to create an OT file output
/// and compare it to what should exist.
/// Read relevant WAV files, create an OT file of some description, write
/// the OT file then compare it to the known good output from OctaChainer.
#[cfg(test)]
#[allow(unused_imports)]
mod tests {

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
        use std::path::{Path, PathBuf};
        use walkdir::{DirEntry, WalkDir};

        use crate::RBoxErr;

        use crate::audio::wav::WavFile;

        use crate::utils::{create_slices_from_wavfiles, get_otsample_nbars_from_wavfiles};
        use octatools_lib::read_type_from_bin_file;
        use octatools_lib::samples::{
            configs::{SampleLoopConfig, SampleTrimConfig},
            options::{
                SampleAttributeLoopMode, SampleAttributeTimestrechMode,
                SampleAttributeTrigQuantizationMode,
            },
            slices::{Slice, Slices},
            SampleAttributes,
        };
        use octatools_lib::Encode;

        fn walkdir_filter_is_wav(entry: &DirEntry) -> bool {
            entry
                .file_name()
                .to_str()
                .map(|s| s.ends_with(".wav"))
                .unwrap_or(false)
        }

        fn get_test_wav_paths(path: &str) -> RBoxErr<Vec<PathBuf>> {
            let paths_iter = WalkDir::new(path)
                .sort_by_file_name()
                .max_depth(1)
                .min_depth(1)
                .into_iter()
                .filter_entry(walkdir_filter_is_wav);

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
                end: wavs.iter().map(|x| x.len).sum(),
                length: bars,
            };

            let loop_config = SampleLoopConfig {
                start: 0,
                length: bars,
                mode: SampleAttributeLoopMode::Off,
            };

            Ok((loop_config, trim_config, slices_config))
        }

        fn read_valid_sample_chain(path: &Path) -> RBoxErr<SampleAttributes> {
            let read_chain = read_type_from_bin_file::<SampleAttributes>(path)?;
            Ok(read_chain)
        }

        #[test]
        fn test_default_10_samples() {
            let wav_fps = get_test_wav_paths("../data/tests/1/wavs/").unwrap();
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

            let valid_ot_fp = PathBuf::from("../data/tests/1/chain.ot");
            let valid_sample_chain = read_valid_sample_chain(&valid_ot_fp).unwrap();

            assert_eq!(
                composed_chain_res.unwrap().encode().unwrap(),
                valid_sample_chain.encode().unwrap(),
            );
        }

        #[test]
        fn test_default_3_samples() {
            let wav_fps = get_test_wav_paths("../data/tests/2/wavs/").unwrap();
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

            let valid_ot_fp = PathBuf::from("../data/tests/2/chain.ot");
            let valid_sample_chain = read_valid_sample_chain(&valid_ot_fp).unwrap();

            assert_eq!(
                composed_chain.encode().unwrap(),
                valid_sample_chain.encode().unwrap(),
            );
        }

        #[ignore]
        #[test]
        fn test_default_64_samples() {
            let wav_fps = get_test_wav_paths("../data/tests/3/wavs/").unwrap();
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

            let valid_ot_fp = PathBuf::from("../data/tests/3/chain.ot");
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
            let wav_fps = get_test_wav_paths("../data/tests/3/wavs/").unwrap();
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

            let valid_ot_fp = PathBuf::from("../data/tests/3/chain.ot");
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

            let slice_conf = Slices { slices, count: 0 };

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
            let wav_fps = get_test_wav_paths("../data/tests/3/wavs/").unwrap();
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

            let valid_ot_fp = PathBuf::from("../data/tests/3/chain.ot");
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
            let wav_fps = get_test_wav_paths("../data/tests/3/wavs/").unwrap();
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

            let valid_ot_fp = PathBuf::from("../data/tests/3/chain.ot");
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

    mod indexing {
        use crate::actions::samples::{
            create_index_samples_dir_full, create_index_samples_dir_simple,
        };
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
            use octatools_lib::yaml_file_to_type;

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
            use octatools_lib::yaml_file_to_type;

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
}
