//! # `octatools`
//!
//! CLI tools to interact with with data files used by the [Elektron Octatrack DPS](https://www.elektron.se/en/octratrack-mkii-explorer)

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
        deconstruct_samplechain_from_pathbufs_only, deconstruct_samplechains_from_yaml,
    },
    copy::{batch_copy_banks, copy_bank},
    indexing::{
        create_index_compact_flash_drive_yaml, create_index_samples_dir_full,
        create_index_samples_dir_simple,
    },
    inspect::{
        show_arrangement, show_arrangement_bytes, show_bank, show_bank_bytes, show_ot_file,
        show_ot_file_bytes, show_pattern, show_project, show_saved_parts, show_unsaved_parts,
    },
    list::list_project_sample_slots,
};

use cli::{Cli, Commands};

#[doc(hidden)]
fn main() -> () {
    let mut logger = Builder::new();
    logger.filter_level(LevelFilter::Debug);
    logger.target(Target::Stdout).init();

    let args = Cli::parse();

    match args.command {
        /* =========================================================================== */
        Commands::Inspect(x) => match x {
            cli::Inspect::Project { path } => {
                info!("Showing project: path={path:#?}");
                let _ = show_project(&path);
            }
            cli::Inspect::Bank { path } => {
                info!("Showing bank: path={path:#?}");
                let _ = show_bank(&path);
            }
            cli::Inspect::PartsSaved { path, index } => {
                info!("Showing specific part in bank: path={path:#?}");
                let _ = show_saved_parts(&path, index);
            }
            cli::Inspect::PartsUnsaved { path, index } => {
                info!("Showing specific part in bank: path={path:#?}");
                let _ = show_unsaved_parts(&path, index);
            }
            cli::Inspect::Patterns { path, index } => {
                info!("Showing specific pattern in bank: path={path:#?}");
                let _ = show_pattern(&path, index);
            }
            cli::Inspect::Sample { path } => {
                info!("Showing sample attributes: path={path:#?}");
                let _ = show_ot_file(&path);
            }
            cli::Inspect::Arrangement { path } => {
                info!("Showing arrangement file: path={path:#?}");
                let _ = show_arrangement(&path);
            }
            cli::Inspect::Markers { path } => {
                info!("Showing markers file: path={path:#?}");
                todo!()
            }
            cli::Inspect::Bytes(y) => match y {
                cli::InspectBytes::Bank {
                    path,
                    start_idx,
                    nbytes,
                } => {
                    info!("Showing bank bytes: path={path:#?}");
                    let _ = show_bank_bytes(&path, &start_idx, &nbytes);
                }
                cli::InspectBytes::Sample {
                    path,
                    start_idx,
                    nbytes,
                } => {
                    info!("Showing sample attributes bytes: path={path:#?}");
                    let _ = show_ot_file_bytes(&path, &start_idx, &nbytes);
                }
                cli::InspectBytes::Arrangement {
                    path,
                    start_idx,
                    nbytes,
                } => {
                    info!("Showing arrangement file bytes: path={path:#?}");
                    let _ = show_arrangement_bytes(&path, &start_idx, &nbytes);
                }
            },
        },
        /* =========================================================================== */
        Commands::List(x) => match x {
            cli::List::Arrangements { path } => {
                info!("Listing arrangements: arrangePath={path:#?}");
                todo!()
            }
            cli::List::ProjectSlots { path } => {
                info!("Listing Project sample slots: projectPath={path:#?}");
                let _ = list_project_sample_slots(&path);
            }
        },

        /* =========================================================================== */
        Commands::Consolidate(x) => match x {
            cli::ConsolidateSamples::ToPool { path } => {
                info!("Consolidating Project samples to Set's Audio Pool: projectPath={path:#?}");
                todo!()
            }
            cli::ConsolidateSamples::ToProject { path } => {
                info!("Consolidating Project samples to Project: projectPath={path:#?}");
                todo!()
            }
        },

        /* =========================================================================== */
        Commands::Transfer(x) => match x {
            cli::Transfer::Bank {
                source_bank_file_path,
                dest_bank_file_path,
            } => {
                info!("Copying bank: src={source_bank_file_path:#?} dest={dest_bank_file_path:#?}");
                let _ = copy_bank(&source_bank_file_path, &dest_bank_file_path);
            }
            cli::Transfer::Banks { yaml_config_path } => {
                info!("Batch copying banks: {yaml_config_path:#?}");
                let _ = batch_copy_banks(&yaml_config_path);
            }
            cli::Transfer::Project {
                source_project,
                dest_set_dir_path,
            } => {
                info!("Copying project: src={source_project:#?} dest={dest_set_dir_path:#?}");
                todo!()
            }
            cli::Transfer::Projects { yaml_config_path } => {
                info!("Batch copying projects: yaml={yaml_config_path:#?}");
                todo!()
            }
        },
        /* =========================================================================== */
        Commands::Chains(x) => match x {
            cli::Chains::CreateChain {
                chain_name,
                out_dir_path,
                wav_file_paths,
            } => {
                info!(
                    "Creating sliced sample chain: outdir={:#?} name={:#?} wavs={:#?}",
                    out_dir_path, chain_name, wav_file_paths,
                );
                let _ = create_samplechain_from_pathbufs_only(
                    &wav_file_paths,
                    &out_dir_path,
                    &chain_name,
                );
            }
            cli::Chains::CreateChains { yaml_file_path } => {
                info!("Creating sliced sample chains: yaml={yaml_file_path:#?}");
                let _ = create_samplechains_from_yaml(&yaml_file_path);
            }
            cli::Chains::DeconstructChain {
                ot_file_path,
                audio_file_path,
                out_dir_path,
            } => {
                info!(
                    "Deconstructing sliced sample chain: sample={:#?} otfile={:#?} outdir={:#?}",
                    audio_file_path, ot_file_path, out_dir_path,
                );
                let _ = deconstruct_samplechain_from_pathbufs_only(
                    &audio_file_path,
                    &ot_file_path,
                    &out_dir_path,
                );
            }
            cli::Chains::DeconstructChains { yaml_file_path } => {
                info!("Batch deconstructing sliced sample chains: yaml={yaml_file_path:#?}");
                let _ = deconstruct_samplechains_from_yaml(&yaml_file_path);
            }
        },
        /* =========================================================================== */
        Commands::Index(x) => match x {
            cli::Indexing::Cfcard {
                cfcard_dir_path,
                yaml_file_path,
            } => {
                info!("Indexing CF card: path={cfcard_dir_path:#?}");
                let _ = create_index_compact_flash_drive_yaml(&cfcard_dir_path, &yaml_file_path);
            }
            cli::Indexing::SamplesdirSimple {
                samples_dir_path,
                yaml_file_path,
            } => {
                let _ = create_index_samples_dir_simple(&samples_dir_path, &yaml_file_path);
            }
            cli::Indexing::SamplesdirFull {
                samples_dir_path,
                yaml_file_path,
            } => {
                info!("Indexing samples directory with 'full' output: path={samples_dir_path:#?}");
                let _ = create_index_samples_dir_full(&samples_dir_path, &yaml_file_path);
            }
        },
    }
}
