mod actions;
mod audio;
mod cli;
mod common;
mod constants;
mod indexing;
mod octatrack;
mod yaml_io;

use clap::Parser;
use env_logger::{Builder, Target};
use log::{debug, info, LevelFilter};

use crate::cli::{Cli, Commands, Indexing};
use crate::common::{FromYamlFile, ToYamlFile};
use crate::indexing::cfcard::CompactFlashDrive;
use crate::indexing::samplesdir::{SamplesDirIndexFull, SamplesDirIndexSimple};
use crate::yaml_io::samplechains::YamlChainConfig;

fn main() -> () {
    let mut logger = Builder::new();
    logger.filter_level(LevelFilter::Debug);
    logger.target(Target::Stdout).init();

    let args = Cli::parse();

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
                    let _ = actions::create_samplechain_from_pathbufs(
                        wav_file_paths,
                        out_dir_path,
                        chain_name,
                    );
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
        Commands::Copy(transfer_subcmd) => match transfer_subcmd {
            cli::Copy::Bank {
                source_bank_file_path,
                dest_bank_file_path,
                copy_samples_to_project,
                merge_duplicate_sample_slots,
                accept_liability,
            } => {
                let _ = actions::transfer_bank(
                    source_bank_file_path,
                    dest_bank_file_path,
                    merge_duplicate_sample_slots.is_some(),
                );
            }
            cli::Copy::Project {
                source_project,
                dest_set_dir_path,
                copy_samples_to_project,
                accept_liability,
            } => {
                unimplemented!();
            }
        },
    }
}
