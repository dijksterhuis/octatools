mod audio;
mod cli;
mod common;
mod constants;
mod indexing;
mod octatrack;
mod results;
mod yaml_io;

use clap::Parser;
use env_logger::{Builder, Target};
use log::{debug, info, LevelFilter};

use crate::cli::{Cli, Commands, Indexing};
use crate::common::{FromYamlFile, ToYamlFile};
use crate::indexing::cfcard::CompactFlashDrive;
use crate::indexing::samplesdir::{SamplesDirIndexFull, SamplesDirIndexSimple};
use crate::yaml_io::samplechains::YamlChainConfig;


mod actions {


    use log::{debug, info, error, warn};
    use std::fs::copy;
    use std::path::PathBuf;

    use crate::yaml_io::samplechains::YamlChainConfig;
    use crate::octatrack::samples::{
        get_otsample_nbars_from_wavfiles, 
        SampleAttributes,
        configs::{SampleLoopConfig, SampleTrimConfig},
        slices::Slices,
    };

    use crate::octatrack::common::FromFileAtPathBuf;

    use crate::octatrack::projects::Project;
    use crate::octatrack::banks::Bank;

    use crate::octatrack::options::{
        SampleAttributeLoopMode, SampleAttributeTimestrechMode, SampleAttributeTrigQuantizationMode,
    };

    use crate::audio::wavfile::{chain_wavfiles_64_batch, WavFile};
    use crate::octatrack::samples::SampleFilePair;

    /// Create Octatrack samplechain file-pairs from a loaded yaml config.

    pub fn create_samplechains_from_yaml(yaml_conf: &YamlChainConfig) -> Result<Vec<SampleFilePair>, ()> {
        let mut outchains_files: Vec<SampleFilePair> = vec![];
        let mut outchains_samplechains: Vec<SampleAttributes> = vec![];

        for chain_config in &yaml_conf.chains {
            info!("Creating chain: {}", &chain_config.chain_name);

            debug!(
                "Reading wav files: n={:#?}",
                &chain_config.sample_file_paths.len()
            );
            let mut wavfiles: Vec<WavFile> = Vec::new();
            for wav_file_path in &chain_config.sample_file_paths {
                // TODO: Clone
                let wavfile = WavFile::from_file(wav_file_path.clone()).unwrap();
                wavfiles.push(wavfile);
            }

            debug!("Batching wav files ...");
            // first element is the chained wavfile output
            // second is the individual wav files that made the chain
            let wavfiles_batched: Vec<(WavFile, Vec<WavFile>)> =
                chain_wavfiles_64_batch(&wavfiles).unwrap();

            for (idx, (single_wav, vec_wavs)) in wavfiles_batched.iter().enumerate() {
                info!("Processing batch: {} / {}", idx + 1, wavfiles_batched.len());

                debug!(
                    "Have {:1?} WAV chains from {:2?} samples",
                    &wavfiles_batched.len(),
                    &wavfiles.len()
                );

                let slices = Slices::from_wavfiles(&vec_wavs, &0).unwrap();

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

                let mut ot_outpath = base_outchain_path.clone();
                let mut wav_sliced_outpath = base_outchain_path.clone();

                ot_outpath.set_extension("ot");
                wav_sliced_outpath.set_extension("wav");

                let _chain_res = chain_data.to_file(&ot_outpath);
                let _wav_slice_res = single_wav.to_file(&wav_sliced_outpath);

                info!(
                    "Created chain files: audio={:?} ot={:?}",
                    wav_sliced_outpath, ot_outpath
                );

                let sample =
                    SampleFilePair::from_pathbufs(&wav_sliced_outpath, &Some(ot_outpath)).unwrap();

                outchains_samplechains.push(chain_data);
                outchains_files.push(sample);
            }
            info!("Created sample chain(s): {}", &chain_config.chain_name);
        }
        debug!("SAMPLE CHAINS GENERATED: {:#?}", outchains_samplechains);

        Ok(outchains_files)
    }

    /// Create Octatrack samplechain file-pairs from a loaded yaml config.

    pub fn create_samplechain_from_pathbufs(
        wav_fps: Vec<PathBuf>,
        outdir_path: PathBuf,
        outchain_name: String,
    ) -> Result<(), ()> {
        let wavfiles: Vec<WavFile> = wav_fps
            .into_iter()
            .map(|fp: PathBuf| WavFile::from_file(fp).unwrap())
            .collect();

        let wavfiles_batched: Vec<(WavFile, Vec<WavFile>)> =
            chain_wavfiles_64_batch(&wavfiles).unwrap();

        for (idx, (single_wav, vec_wavs)) in wavfiles_batched.iter().enumerate() {
            let slices = Slices::from_wavfiles(&vec_wavs, &0).unwrap();

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

            let base_outchain_path = outdir_path.join(&outchain_name);

            let mut wav_sliced_outpath = base_outchain_path;
            wav_sliced_outpath.set_extension("wav");
            let _wav_slice_res = single_wav.to_file(&wav_sliced_outpath);
            info!("Created chain audio file: {wav_sliced_outpath:#?}");

            let mut ot_outpath = wav_sliced_outpath;
            ot_outpath.set_extension("ot");
            let _chain_res = chain_data.to_file(&ot_outpath);
            info!("Created chain attributes file: {ot_outpath:#?}");
        }

        Ok(())
    }

    /// Transfer a bank from one project to another project

    pub fn transfer_bank(
        source_bank_file_path: PathBuf,
        dest_bank_file_path: PathBuf,
        merge_duplicate_sample_slots: bool,
    ) -> Result<(), ()> {


        let dst_proj_path = dest_bank_file_path
            .parent()
            .unwrap()
            .to_path_buf()
            .join("project.work")
        ;

        let dest_project = Project::from_pathbuf(dst_proj_path).unwrap();

        let dst_dirpath = &dest_bank_file_path.parent().unwrap().to_path_buf();
        let dest_sample_slots = dest_project.slots;

        let src_proj_path = source_bank_file_path
            .parent()
            .unwrap()
            .to_path_buf()
            .join("project.work")
        ;

        let src_dirpath = &src_proj_path.parent().unwrap().to_path_buf();
        let src_project = Project::from_pathbuf(src_proj_path).unwrap();


        let src = Bank::from_pathbuf(source_bank_file_path).unwrap();


        let sample_slots = todo!();
        let trig_sample_locks = todo!();
        let sample_slots_active_machines = todo!();

        // take sample slots and copy them to new slots in new project
        // 1. read old project
        // 2. get sample slots
        // 3. read new project
        // 4. find space in new project sample slots
        // 5. read old bank
        //  * machine assigned sample slots
        //  * trig sample lock assigned sample slots
        // 6. create backup files 
        //  * new project
        //  * new bank file
        // 7. copy samples to new project folder
        // 8. add samples to new project
        // 9. edit bank sample slot usage
        // 10. edit sample slots in read bank data
        //  *  machine assignment 
        //  *  trig smaple lock assignment 
        // 11. write new bank data

        todo!();

        // Copy bank file
        copy(src_dirpath, dst_dirpath);


        // TODO: move them

        Ok(())
    }

}



fn main() -> () {
    let mut logger = Builder::new();
    logger.filter_level(LevelFilter::Debug);
    logger.target(Target::Stdout).init();

    let args = Cli::parse();

    info!("ARGS: {:#?}", args);

    match args.command {
        Commands::Scan(subcmd_scan) => {
            match subcmd_scan {
                cli::Indexing::Cfcard {
                    cfcard_dir_path,
                    yaml_file_path,
                } => {
                    let cf =
                        CompactFlashDrive::from_pathbuf(cfcard_dir_path, yaml_file_path).unwrap();

                    // TODO: clone
                    if !cf.index_file_path.is_none() {
                        let _ = cf.to_yaml(cf.index_file_path.clone().unwrap());
                    };
                }
                Indexing::Samples(scan_samples_subcmd) => {
                    match scan_samples_subcmd {
                        cli::IndexSamples::Simple {
                            samples_dir_path,
                            yaml_file_path,
                        } => {
                            let sample_index =
                                SamplesDirIndexSimple::new(samples_dir_path, yaml_file_path)
                                    .unwrap();

                            // TODO: clone
                            if !sample_index.index_file_path.is_none() {
                                let _ = sample_index
                                    .to_yaml(sample_index.index_file_path.clone().unwrap());
                            };
                        }
                        cli::IndexSamples::Full {
                            samples_dir_path,
                            yaml_file_path,
                        } => {
                            let sample_index =
                                SamplesDirIndexFull::new(samples_dir_path, yaml_file_path).unwrap();

                            // TODO: clone
                            if !sample_index.index_file_path.is_none() {
                                let _ = sample_index
                                    .to_yaml(sample_index.index_file_path.clone().unwrap());
                            };
                        }
                    }
                }
            }
        }

        Commands::Chains(chains_subcmd) => match chains_subcmd {
            cli::Chains::Create(chains_create_subcmd) => match chains_create_subcmd {
                cli::CreateChain::Cli {
                    chain_name,
                    out_dir_path,
                    wav_file_paths,
                } => {
                    let _ =
                        actions::create_samplechain_from_pathbufs(wav_file_paths, out_dir_path, chain_name);
                }
                cli::CreateChain::Yaml { yaml_file_path } => {
                    let chain_conf = YamlChainConfig::from_yaml(yaml_file_path).unwrap();

                    let _ = actions::create_samplechains_from_yaml(&chain_conf);
                }
            },
            cli::Chains::Deconstruct(chains_deconstruct_subcmd) => {
                match chains_deconstruct_subcmd {
                    cli::DesconstructChain::Cli {
                        ot_file_path,
                        audio_file_path,
                        out_dir_path,
                    } => {
                        todo!()
                    }
                    cli::DesconstructChain::Yaml { yaml_file_path } => {
                        todo!()
                    }
                }
            }
        },
        Commands::TransferBank {
            source_bank_file_path,
            dest_bank_file_path,
            copy_samples_to_project,
            merge_duplicate_sample_slots,
            accept_liability,
        } => {}
    }
}
