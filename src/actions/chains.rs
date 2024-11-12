//! Functions for CLI actions related to chaining samples into sliced sample chains.

use log::{debug, info, trace};
use std::path::PathBuf;

use serde_octatrack::{
    samples::options::{
        SampleAttributeLoopMode, SampleAttributeTimestrechMode, SampleAttributeTrigQuantizationMode,
    },
    samples::{
        configs::{SampleLoopConfig, SampleTrimConfig},
        SampleAttributes,
    },
};

use crate::utils::SampleFilePair;

use crate::{
    audio::{aiff::AiffFile, wav::WavFile},
    utils::{create_slices_from_wavfiles, get_otsample_nbars_from_wavfiles},
    yaml_io::samplechains::YamlChainConfig,
};

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
    let originals: Vec<WavFile> = wavfiles.clone();
    let mut slice_vecs: Vec<Vec<WavFile>> = vec![];

    let vec_mod_length = wavfiles.len().div_euclid(64);

    trace!("Creating batches.");
    for i in 0..(vec_mod_length + 1) {
        let (start, mut end) = (i * 64, (i * 64) + 64);

        if end > originals.len() {
            end = originals.len();
        };
        let mut s: Vec<WavFile> = Vec::with_capacity(end - start);

        for o in &originals[start..end] {
            s.push(o.clone());
        }
        slice_vecs.push(s);
    }

    trace!("Creating singular sample of samples in each batch.");
    let mut chains: Vec<(WavFile, Vec<WavFile>)> = vec![];
    for slice_vec in slice_vecs {
        let mut single_chain_wav: WavFile = slice_vec[0].clone();

        for wavfile in slice_vec[1..].into_iter() {
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

pub fn create_samplechains_from_yaml(
    yaml_conf: &YamlChainConfig,
) -> Result<Vec<SampleFilePair>, ()> {
    info!("Creating sample chains from yaml config.");
    trace!("Yaml contents: {yaml_conf:#?}");

    let mut outchains_files: Vec<SampleFilePair> = vec![];
    let mut outchains_samplechains: Vec<SampleAttributes> = vec![];

    for chain_config in &yaml_conf.chains {
        info!("Creating chain: name={:#?}", &chain_config.chain_name);
        info!(
            "Getting wav files: n={:#?}",
            &chain_config.sample_file_paths.len()
        );

        let wavfiles: Vec<WavFile> = chain_config
            .sample_file_paths
            .iter()
            .map(|w| WavFile::from_file(w.to_path_buf()).unwrap())
            .collect();

        debug!("Batching wav files ...");

        // first element is the chained audio data
        // second is the individual audio files that made the chain
        let wavfiles_batched: Vec<(WavFile, Vec<WavFile>)> =
            chain_wavfiles_64_batch(&wavfiles).unwrap();

        for (idx, (single_wav, vec_wavs)) in wavfiles_batched.iter().enumerate() {
            info!("Processing batch: {} / {}", idx + 1, wavfiles_batched.len());
            debug!(
                "Have {:#?} WAV chains from {:#?} samples",
                &wavfiles_batched.len(),
                &wavfiles.len()
            );

            trace!("Making slices: {} / {}", idx + 1, wavfiles_batched.len());
            let slices = create_slices_from_wavfiles(&vec_wavs, 0).unwrap();

            // let chain = SampleChain::from_yaml_conf(&chain_config).unwrap();
            // chains.insert(chain);

            trace!(
                "Calculating bar length: {} / {}",
                idx + 1,
                wavfiles_batched.len()
            );
            // TODO -- can use single wavfile here?! would make the funtion more generally applicable.
            let bars = get_otsample_nbars_from_wavfiles(&vec_wavs, &125.0).unwrap();

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
                mode: chain_config.octatrack_settings.loop_mode,
            };

            let fstem = chain_config.chain_name.clone() + &format!("-{:?}", idx);

            trace!(
                "Creating new sample attribute: {} / {}",
                idx + 1,
                wavfiles_batched.len()
            );
            let chain_data = SampleAttributes::new(
                &chain_config.octatrack_settings.bpm,
                &chain_config.octatrack_settings.timestretch_mode,
                &chain_config.octatrack_settings.quantization_mode,
                &chain_config.octatrack_settings.gain,
                &trim_config,
                &loop_config,
                &slices,
            )
            .unwrap();

            let base_outchain_path = yaml_conf.global_settings.out_dir_path.join(fstem);

            trace!(
                "Creating WAV file: {} / {}",
                idx + 1,
                wavfiles_batched.len()
            );
            let mut wav_sliced_outpath = base_outchain_path;
            wav_sliced_outpath.set_extension("wav");
            let _wav_slice_res = single_wav.to_file(&wav_sliced_outpath);
            info!("Created chain audio file: {wav_sliced_outpath:#?}");

            trace!(
                "Creating OT sample attributes file: {} / {}",
                idx + 1,
                wavfiles_batched.len()
            );
            let mut ot_outpath = wav_sliced_outpath.clone();
            ot_outpath.set_extension("ot");
            let _chain_res = chain_data.to_file(&ot_outpath);
            info!("Created chain attributes file: {ot_outpath:#?}");

            let sample =
                SampleFilePair::from_pathbufs(&wav_sliced_outpath, &Some(ot_outpath)).unwrap();

            outchains_samplechains.push(chain_data);
            outchains_files.push(sample);
        }
        info!("Created sample chain: name={}", &chain_config.chain_name);
    }
    info!("All sample chain(s) created.");
    trace!("Sample Chains: {:#?}", outchains_samplechains);

    Ok(outchains_files)
}

/// Create 64 length sample chains and write out the files.

pub fn create_samplechain_from_pathbufs_only(
    wav_fps: Vec<PathBuf>,
    outdir_path: PathBuf,
    outchain_name: String,
) -> Result<Vec<SampleFilePair>, ()> {
    let mut outchains_files: Vec<SampleFilePair> = vec![];

    let wavfiles: Vec<WavFile> = wav_fps
        .into_iter()
        .map(|fp: PathBuf| WavFile::from_file(fp).unwrap())
        .collect();

    let wavfiles_batched: Vec<(WavFile, Vec<WavFile>)> =
        chain_wavfiles_64_batch(&wavfiles).unwrap();

    for (idx, (single_wav, vec_wavs)) in wavfiles_batched.iter().enumerate() {
        trace!("Making slices: {} / {}", idx + 1, wavfiles_batched.len());
        let slices = create_slices_from_wavfiles(&vec_wavs, 0).unwrap();

        trace!(
            "Calculating bar length: {} / {}",
            idx + 1,
            wavfiles_batched.len()
        );
        // TODO -- can use single wavfile here?! would make the funtion more generally applicable.
        let bars = get_otsample_nbars_from_wavfiles(&vec_wavs, &125.0).unwrap();

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
        .unwrap();

        trace!(
            "Modifying file paths: {} / {}",
            idx + 1,
            wavfiles_batched.len()
        );
        let base_outchain_path = outdir_path.join(&outchain_name);

        let mut wav_sliced_outpath = base_outchain_path;
        wav_sliced_outpath.set_extension("wav");
        let _wav_slice_res = single_wav.to_file(&wav_sliced_outpath);
        info!("Created chain audio file: {wav_sliced_outpath:#?}");

        let mut ot_outpath = wav_sliced_outpath.clone();
        ot_outpath.set_extension("ot");
        let _chain_res = chain_data.to_file(&ot_outpath);
        info!("Created chain attributes file: {ot_outpath:#?}");

        let sample = SampleFilePair::from_pathbufs(&wav_sliced_outpath, &Some(ot_outpath)).unwrap();

        outchains_files.push(sample);
    }
    info!("Created sample chain: name={}", &outchain_name);
    Ok(outchains_files)
}

/// Use input files from `resouces/test-data/` to create an OT file output
/// and compare it to what should exist.
/// Read relevant WAV files, create an OT file of some description, write
/// the OT file then compare it to the known good output from OctaChainer.

#[cfg(test)]
mod tests {

    mod create_vs_read {

        use std::path::PathBuf;
        use walkdir::{DirEntry, WalkDir};

        use crate::common::RBoxErr;

        use crate::audio::wav::WavFile;

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
                let wav = WavFile::from_file(fp).unwrap();
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

        fn read_valid_sample_chain(path: &str) -> RBoxErr<SampleAttributes> {
            let read_chain = SampleAttributes::from_file(path).unwrap();
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

            let composed_chain = &composed_chain_res.clone().unwrap();

            if composed_chain_res.is_err() {
                println!("ERROR IN TEST: {:#?}:", &composed_chain_res.err());
                assert!(false);
            }

            let valid_ot_fp = "data/tests/1/chain.ot";
            let valid_sample_chain = read_valid_sample_chain(&valid_ot_fp).unwrap();

            assert_eq!(
                composed_chain.encode().unwrap(),
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

            let valid_ot_fp = "data/tests/2/chain.ot";
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

            let valid_ot_fp = "data/tests/3/chain.ot";
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

            let valid_ot_fp = "data/tests/3/chain.ot";
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

            let valid_ot_fp = "data/tests/3/chain.ot";
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

            let valid_ot_fp = "data/tests/3/chain.ot";
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
