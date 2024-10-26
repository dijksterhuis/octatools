mod cli;
mod yaml_io;
mod indexing;
mod results;
mod constants;
mod octatrack;
mod audio;
mod common;

use clap::Parser;
use env_logger::{Builder, Target};
use log::{info, debug, LevelFilter};
use std::path::PathBuf;

use crate::common::{ToYamlFile, FromYamlFile};
use crate::yaml_io::samplechains::YamlChainConfig;
use crate::indexing::samplesdir::{SamplesDirIndexFull, SamplesDirIndexSimple};
use crate::indexing::cfcard::CompactFlashDrive;
use crate::cli::{Cli, Commands, Chains, CreateChain};

use crate::octatrack::samples::{
    SampleAttributes,
    get_otsample_nbars_from_wavfiles,
};

use crate::octatrack::samples::{
    slices::Slices,
    configs::{SampleLoopConfig, SampleTrimConfig}
};

use crate::octatrack::options::{
    SampleAttributeLoopMode,
    SampleAttributeTimestrechMode,
    SampleAttributeTrigQuantizationMode,
};

use crate::octatrack::samples::SampleFilePair;
use crate::audio::wavfile::{
    WavFile,
    chain_wavfiles_64_batch,
};

/// Create Octatrack samplechain file-pairs from a loaded yaml config.

fn create_samplechains_from_yaml(yaml_conf: &YamlChainConfig) -> Result<Vec<SampleFilePair>, ()> {

    let mut outchains_files: Vec<SampleFilePair> = vec![];
    let mut outchains_samplechains: Vec<SampleAttributes> = vec![];

    for chain_config in &yaml_conf.chains {

        info!("Creating chain: {}", &chain_config.chain_name);

        debug!("Reading wav files: n={:#?}", &chain_config.sample_file_paths.len());
        let mut wavfiles: Vec<WavFile> = Vec::new();
        for wav_file_path in &chain_config.sample_file_paths {
            // TODO: Clone
            let wavfile = WavFile::from_file(wav_file_path.clone()).unwrap();
            wavfiles.push(wavfile);
        };

        debug!("Batching wav files ...");
        // first element is the chained wavfile output
        // second is the individual wav files that made the chain
        let wavfiles_batched: Vec<(WavFile, Vec<WavFile>)> = chain_wavfiles_64_batch(&wavfiles).unwrap();

        for (idx, (single_wav, vec_wavs)) in wavfiles_batched.iter().enumerate() {

            info!("Processing batch: {} / {}", idx + 1, wavfiles_batched.len());

            debug!(
                "Have {:1?} WAV chains from {:2?} samples",
                &wavfiles_batched.len(),
                &wavfiles.len()
            );

            let slices = Slices
                ::from_wavfiles(&vec_wavs, &0)
                .unwrap()
            ;

            // let chain = SampleChain::from_yaml_conf(&chain_config).unwrap();
            // chains.insert(chain);

            // TODO -- can use single wavfile here?! would make the funtion more generally applicable.
            let bars = get_otsample_nbars_from_wavfiles(&vec_wavs, &125.0).unwrap();

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

            let chain_data = SampleAttributes
                ::new(
                    &chain_config.octatrack_settings.bpm,
                    &chain_config.octatrack_settings.timestretch_mode,
                    &chain_config.octatrack_settings.quantization_mode,
                    &chain_config.octatrack_settings.gain,
                    &trim_config,
                    &loop_config,
                    &slices
                )
                .unwrap()
            ;

            let base_outchain_path = yaml_conf
                .global_settings
                .out_dir_path
                .join(fstem);
    
            let mut ot_outpath = base_outchain_path.clone();
            let mut wav_sliced_outpath = base_outchain_path.clone();
    
            ot_outpath.set_extension("ot");
            wav_sliced_outpath.set_extension("wav");
    
            let _chain_res = chain_data.to_file(&ot_outpath);
            let _wav_slice_res = single_wav.to_file(&wav_sliced_outpath);

            info!("Created chain files: audio={:?} ot={:?}", wav_sliced_outpath, ot_outpath);

            let sample = SampleFilePair
                ::from_pathbufs(&wav_sliced_outpath, &Some(ot_outpath))
                .unwrap()
            ;

            outchains_samplechains.push(chain_data);
            outchains_files.push(sample);
        };
        info!("Created sample chain(s): {}", &chain_config.chain_name);
    };
    debug!("SAMPLE CHAINS GENERATED: {:#?}", outchains_samplechains);

    Ok(outchains_files)
}


/// Create Octatrack samplechain file-pairs from a loaded yaml config.

fn create_samplechain_from_pathbufs(outchain_name: String, wav_fps: Vec<PathBuf>) -> Result<(), ()> {

    let mut wavfiles: Vec<WavFile> = Vec::new();

    let wavfiles: Vec<WavFile> = wav_fps
        .into_iter()
        .map(|fp: PathBuf| WavFile::from_file(fp).unwrap())
        .collect()
    ;

    let wavfiles_batched: Vec<(WavFile, Vec<WavFile>)> = chain_wavfiles_64_batch(&wavfiles).unwrap();

    for (idx, (single_wav, vec_wavs)) in wavfiles_batched.iter().enumerate() {

        let slices = Slices
            ::from_wavfiles(&vec_wavs, &0)
            .unwrap()
        ;

        // TODO -- can use single wavfile here?! would make the funtion more generally applicable.
        let bars = get_otsample_nbars_from_wavfiles(&vec_wavs, &125.0).unwrap();

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

        let chain_data = SampleAttributes
            ::new(
                &120.0,
                &SampleAttributeTimestrechMode::default(),
                &SampleAttributeTrigQuantizationMode::default(),
                &0.0,
                &trim_config,
                &loop_config,
                &slices
            )
            .unwrap()
        ;

        let base_outchain_path = PathBuf::from(&outchain_name);

        let mut wav_sliced_outpath = base_outchain_path;
        wav_sliced_outpath.set_extension("wav");
        let _wav_slice_res = single_wav.to_file(&wav_sliced_outpath);
        info!("Created chain audio file: {wav_sliced_outpath:#?}");

        let mut ot_outpath = wav_sliced_outpath;
        ot_outpath.set_extension("ot");
        let _chain_res = chain_data.to_file(&ot_outpath);
        info!("Created chain attributes file: {ot_outpath:#?}");

    };

    Ok(())
}


fn main() -> () {

    let mut logger = Builder::new();
    logger.filter_level(LevelFilter::Debug);
    logger.target(Target::Stdout).init();

    let args = Cli::parse();

    info!("ARGS: {:#?}", args);

    match args.command {
        // Commands::ScanSamplesDirFull { samples_dir_path, yaml_file_path}
        // => {
        //     let sample_index = SamplesDirIndexFull
        //         ::new(samples_dir_path, yaml_file_path)
        //         .unwrap()
        //     ;

        //     // TODO: clone
        //     if ! sample_index.index_file_path.is_none() {
        //         let _ = sample_index.to_yaml(sample_index.index_file_path.clone().unwrap());
        //     };

        // },
        // Commands::ScanSamplesDirSimple { samples_dir_path, yaml_file_path}
        // => {
        //     let sample_index = SamplesDirIndexSimple
        //         ::new(samples_dir_path, yaml_file_path)
        //         .unwrap()
        //     ;

        //     // TODO: clone
        //     if ! sample_index.index_file_path.is_none() {
        //         let _ = sample_index.to_yaml(sample_index.index_file_path.clone().unwrap());
        //     };

        // },
        // Commands::ScanCfCard { cfcard_dir_path, csv_file_path} 
        // => {
        //     let cf = CompactFlashDrive
        //         ::from_pathbuf(cfcard_dir_path, csv_file_path)
        //         .unwrap()
        //     ;

        //     // TODO: clone
        //     if ! cf.index_file_path.is_none() {
        //         let _ = cf.to_yaml(cf.index_file_path.clone().unwrap());
        //     };
        // },

        Commands::Chains(subcommand) => {
            println!("{subcommand:#?}");

            match &subcommand {
                CreateChain => {
                    println!("CREATE");
                    match &subcommand {
                        Yaml => {
                            println!("yaml");
                        }
                        Cli => {
                            println!("yaml");
                        }
                        _ => {}
                    }

                }
                _ => {}

            }
        }
        
        // ::Chain::Create::Yaml => { yaml_file_path } 
        // => {
        //     let chain_conf = YamlChainConfig
        //         ::from_yaml(yaml_file_path)
        //         .unwrap()
        //     ;

        //     let _ = create_samplechains_from_yaml(&chain_conf);
        // },
        // Commands::CreateChainCli { wav_file_paths, chain_name } 
        // => {
        //     let _ = create_samplechain_from_pathbufs(chain_name, wav_file_paths);
        // }
        x => {
            println!("{x:#?}");
        }
    }

}
