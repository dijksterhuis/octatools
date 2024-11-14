mod actions;
mod audio;
mod cli;
mod common;
mod octatrack_sets;
mod utils;

use clap::Parser;
use env_logger::{Builder, Target};
use log::{info, LevelFilter};

use actions::{
    chains::{
        create_samplechain_from_pathbufs_only, create_samplechains_from_yaml,
        deconstruct_samplechain_from_pathbufs_only,
    },
    copy::{batch_copy_banks, copy_bank},
    indexing::{
        create_index_compact_flash_drive_yaml, create_index_samples_dir_full,
        create_index_samples_dir_simple,
    },
    inspect::{
        show_bank, show_ot_file, show_part, show_parts, show_pattern, show_patterns, show_project,
    },
    list::list_project_sample_slots,
};

use cli::{Cli, Commands, Indexing};

fn main() -> () {
    let mut logger = Builder::new();
    logger.filter_level(LevelFilter::Debug);
    logger.target(Target::Stdout).init();

    let args = Cli::parse();

    match args.command {
        /* =========================================================================== */
        Commands::Inspect(x) => match x {
            cli::Inspect::Project { path } => {
                let _ = show_project(&path);
            }
            cli::Inspect::Bank { path } => {
                let _ = show_bank(&path);
            }
            cli::Inspect::Parts { path } => {
                let _ = show_parts(&path);
            }
            cli::Inspect::Part { path, index } => {
                let _ = show_part(&path, index);
            }
            cli::Inspect::Patterns { path } => {
                let _ = show_patterns(&path);
            }
            cli::Inspect::Pattern { path, index } => {
                let _ = show_pattern(&path, index);
            }
            cli::Inspect::Sample { path } => {
                let _ = show_ot_file(&path);
            }
        },
        /* =========================================================================== */
        Commands::List(x) => match x {
            cli::List::Arrangements { path: _ } => {
                todo!()
            }
            cli::List::ProjectSlots { path } => {
                let _ = list_project_sample_slots(&path);
            }
        },

        /* =========================================================================== */
        Commands::Consolidate(x) => match x {
            cli::ConsolidateSamples::ToPool { path: _ } => {
                todo!()
            }
            cli::ConsolidateSamples::ToProject { path: _ } => {
                todo!()
            }
        },

        /* =========================================================================== */
        Commands::Transfer(transfer_subcmd) => match transfer_subcmd {
            cli::Transfer::Banks(x) => match x {
                cli::TransferBank::Cli {
                    source_bank_file_path,
                    dest_bank_file_path,
                } => {
                    info!("Copying bank: src={source_bank_file_path:#?} dest={dest_bank_file_path:#?}");
                    let _ = copy_bank(source_bank_file_path, dest_bank_file_path);
                }
                cli::TransferBank::Yaml { yaml_config_path } => {
                    info!("Copying bank using yaml config: {yaml_config_path:#?}");
                    let _ = batch_copy_banks(yaml_config_path);
                }
            },
            cli::Transfer::Projects(x) => match x {
                cli::TransferProject::Cli {
                    source_project: _,
                    dest_set_dir_path: _,
                } => {
                    todo!()
                }
                cli::TransferProject::Yaml {
                    yaml_config_path: _,
                } => {
                    todo!()
                }
            },
        },
        /* =========================================================================== */
        Commands::Chains(chains_subcmd) => match chains_subcmd {
            cli::Chains::Create(chains_create_subcmd) => match chains_create_subcmd {
                cli::CreateChain::Cli {
                    chain_name,
                    out_dir_path,
                    wav_file_paths,
                } => {
                    info!("Creating sliced sample chain via CLI args: name={chain_name:#?}");
                    let _ = create_samplechain_from_pathbufs_only(
                        &wav_file_paths,
                        &out_dir_path,
                        &chain_name,
                    );
                }
                cli::CreateChain::Yaml { yaml_file_path } => {
                    info!("Creating sliced sample chains: yaml={yaml_file_path:#?}");
                    let _ = create_samplechains_from_yaml(&yaml_file_path);
                }
            },
            cli::Chains::Deconstruct(chains_deconstruct_subcmd) => {
                match chains_deconstruct_subcmd {
                    cli::DesconstructChain::Cli {
                        ot_file_path,
                        audio_file_path,
                        out_dir_path,
                    } => {
                        info!("Deconstructing sliced sample chain from CLI args ...");
                        let _ = deconstruct_samplechain_from_pathbufs_only(
                            audio_file_path,
                            ot_file_path,
                            out_dir_path,
                        );
                    }
                    cli::DesconstructChain::Yaml { yaml_file_path: _ } => {
                        info!("Deconstructing sliced sample chains from YAML file ...");
                        todo!()
                    }
                }
            }
        },
        /* =========================================================================== */
        Commands::Scan(subcmd_scan) => match subcmd_scan {
            cli::Indexing::Cfcard {
                cfcard_dir_path,
                yaml_file_path,
            } => {
                info!("Indexing CF card: path={cfcard_dir_path:#?}");
                let _ = create_index_compact_flash_drive_yaml(&cfcard_dir_path, &yaml_file_path);
            }
            Indexing::Samples(scan_samples_subcmd) => {
                match scan_samples_subcmd {
                    cli::IndexSamples::Simple {
                        samples_dir_path,
                        yaml_file_path,
                    } => {
                        let _ = create_index_samples_dir_simple(&samples_dir_path, &yaml_file_path);
                    }
                    cli::IndexSamples::Full {
                        samples_dir_path,
                        yaml_file_path,
                    } => {
                        info!("Indexing samples directory with 'full' output: path={samples_dir_path:#?}");
                        let _ = create_index_samples_dir_full(&samples_dir_path, &yaml_file_path);
                    }
                }
            }
        },
    }
}
