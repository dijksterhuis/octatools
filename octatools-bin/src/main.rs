//! # `octatools`
//!
//! CLI tools to interact with data files used by the [Elektron Octatrack DPS](https://www.elektron.se/en/octratrack-mkii-explorer)

mod actions;
mod audio;
mod cli;
mod common;
mod octatrack_sets;
mod utils;

use clap::Parser;
use env_logger::{Builder, Target};
use log::LevelFilter;

use cli::{Cli, Commands};
use std::error::Error;

use actions::{
    arrangements::show_arrangement_bytes,
    banks::{batch_copy_banks, copy_bank, show_bank_bytes},
    drive::create_file_index_yaml,
    parts::{show_saved_parts, show_unsaved_parts},
    patterns::show_pattern,
    projects::{
        consolidate_sample_slots_to_audio_pool, consolidate_sample_slots_to_project_pool,
        list_project_sample_slots, purge_project_pool,
    },
    samples::{
        create_equally_sliced_sample, create_index_samples_dir_full,
        create_index_samples_dir_simple, create_randomly_sliced_sample,
        create_samplechain_from_pathbufs_only, create_samplechains_from_yaml,
        deconstruct_samplechain_from_pathbufs_only, deconstruct_samplechains_from_yaml,
        show_ot_file_bytes,
    },
};
use serde_octatrack::arrangements::ArrangementFile;
use serde_octatrack::banks::Bank;
use serde_octatrack::projects::Project;
use serde_octatrack::samples::SampleAttributes;

pub type RBoxErr<T> = Result<T, Box<dyn Error>>;
pub type RVoidError<T> = Result<T, ()>;

#[derive(Debug)]
pub enum OctatoolErrors {
    PathDoesNotExist,
    PathIsNotADirectory,
    PathIsNotAFile,
    PathIsNotASet,
    CliInvalidPartIndices,
    CliMissingPartIndices,
    CliInvalidPatternIndices,
    CliMissingPatternIndices,
    Unknown,
}
impl std::fmt::Display for OctatoolErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::PathDoesNotExist => write!(f, "path does not exist"),
            Self::PathIsNotADirectory => write!(f, "path is not a directory"),
            Self::PathIsNotAFile => write!(f, "path is not a file"),
            Self::PathIsNotASet => write!(
                f,
                "path is not an Octatrack set directory (no 'AUDIO' subdirectory found)"
            ),
            Self::CliMissingPartIndices => write!(
                f,
                "Missing part number(s) - part number(s) between 1-4 must be be provided"
            ),
            Self::CliInvalidPartIndices => write!(
                f,
                "Invalid part number(s) - only part numbers between 1-4 can be provided"
            ),
            Self::CliMissingPatternIndices => write!(
                f,
                "Missing pattern number(s) - pattern number(s) between 1-16 must be be provided"
            ),
            Self::CliInvalidPatternIndices => write!(
                f,
                "Invalid pattern number(s) - only numbers between 1-16 can be provided"
            ),
            Self::Unknown => write!(f, "unknown error (please investigate/report)"),
        }
    }
}
impl std::error::Error for OctatoolErrors {}

#[doc(hidden)]
fn main() {
    let mut logger = Builder::new();
    logger.filter_level(LevelFilter::Debug);
    logger.target(Target::Stdout).init();

    let args = Cli::parse();

    match args.command {
        Commands::Drive(x) => match x {
            cli::Drive::Dump {
                cfcard_dir_path,
                yaml_file_path,
            } => {
                let _ = create_file_index_yaml(&cfcard_dir_path, &yaml_file_path);
            }
        },
        Commands::Projects(x) => match x {
            cli::Projects::Inspect { path } => {
                let _ = serde_octatrack::show_type::<Project>(&path, None);
            }
            cli::Projects::Settings(y) => match y {
                cli::ProjectData::Inspect { path: _path } => {
                    unimplemented!();
                }
            },
            cli::Projects::Metadata(y) => match y {
                cli::ProjectData::Inspect { path: _path } => {
                    unimplemented!();
                }
            },
            cli::Projects::State(y) => match y {
                cli::ProjectData::Inspect { path: _path } => {
                    unimplemented!();
                }
            },
            cli::Projects::Sampleslots(y) => match y {
                cli::SampleSlots::Inspect { path: _path } => {
                    unimplemented!();
                }
                cli::SampleSlots::List { path } => {
                    let _ = list_project_sample_slots(&path);
                }
                cli::SampleSlots::Purge { path } => {
                    let _ = purge_project_pool(&path);
                }
                cli::SampleSlots::Consolidate { path } => {
                    let _ = consolidate_sample_slots_to_project_pool(&path);
                }
                cli::SampleSlots::Centralise { path } => {
                    let _ = consolidate_sample_slots_to_audio_pool(&path);
                }
            },
            cli::Projects::Dump {
                project_file_path,
                yaml_file_path,
            } => {
                let _ = serde_octatrack::bin_file_to_yaml_file::<Project>(
                    &project_file_path,
                    &yaml_file_path,
                );
            }
            cli::Projects::Load {
                yaml_file_path,
                project_file_path,
            } => {
                let _ = serde_octatrack::yaml_file_to_bin_file::<Project>(
                    &yaml_file_path,
                    &project_file_path,
                );
            }
        },
        Commands::Arrangements(x) => match x {
            cli::Arrangements::Inspect { path } => {
                let _ = serde_octatrack::show_type::<ArrangementFile>(&path, None);
            }
            cli::Arrangements::InspectBytes {
                path,
                byte_start_idx,
                n_bytes,
            } => {
                let _ = show_arrangement_bytes(&path, &byte_start_idx, &n_bytes);
            }
            cli::Arrangements::Dump {
                arrangement_file_path,
                yaml_file_path,
            } => {
                let r = serde_octatrack::bin_file_to_yaml_file::<ArrangementFile>(
                    &arrangement_file_path,
                    &yaml_file_path,
                );
                if r.is_err() {
                    println!("ERROR: {r:?}")
                };
            }
            cli::Arrangements::Load {
                yaml_file_path,
                arrangement_file_path,
            } => {
                let r = serde_octatrack::yaml_file_to_bin_file::<ArrangementFile>(
                    &yaml_file_path,
                    &arrangement_file_path,
                );
                if r.is_err() {
                    println!("ERROR: {r:?}")
                };
            }
        },
        Commands::Banks(x) => match x {
            cli::Banks::Inspect { path } => {
                let _ = serde_octatrack::show_type::<Bank>(&path, None);
            }
            cli::Banks::InspectBytes {
                path,
                byte_start_idx,
                n_bytes,
            } => {
                let _ = show_bank_bytes(&path, &byte_start_idx, &n_bytes);
            }
            cli::Banks::Copy {
                source_bank_filepath,
                source_project_filepath,
                destination_bank_filepath,
                destination_project_filepath,
            } => {
                let _ = copy_bank(
                    &source_bank_filepath,
                    &source_project_filepath,
                    &destination_bank_filepath,
                    &destination_project_filepath,
                );
            }
            cli::Banks::CopyN { yaml_file_path } => {
                let _ = batch_copy_banks(&yaml_file_path);
            }
            cli::Banks::Dump {
                bank_file_path,
                yaml_file_path,
            } => {
                let _ = serde_octatrack::bin_file_to_yaml_file::<Bank>(
                    &bank_file_path,
                    &yaml_file_path,
                );
            }
            cli::Banks::Load {
                yaml_file_path,
                bank_file_path,
            } => {
                let _ = serde_octatrack::yaml_file_to_bin_file::<Bank>(
                    &yaml_file_path,
                    &bank_file_path,
                );
            }
        },
        Commands::Patterns(x) => match x {
            crate::cli::Patterns::Inspect {
                bank_file_path,
                index,
            } => {
                let _ = show_pattern(&bank_file_path, index);
            }
        },
        Commands::Parts(x) => match x {
            crate::cli::Parts::Saved(y) => match y {
                crate::cli::PartsCmd::Inspect {
                    bank_file_path,
                    index,
                } => {
                    let _ = show_saved_parts(&bank_file_path, index);
                }
            },
            crate::cli::Parts::Unsaved(y) => match y {
                crate::cli::PartsCmd::Inspect {
                    bank_file_path,
                    index,
                } => {
                    let _ = show_unsaved_parts(&bank_file_path, index);
                }
            },
        },
        Commands::Samples(x) => match x {
            crate::cli::Samples::Chain(y) => match y {
                crate::cli::SampleChains::Create {
                    chain_name,
                    out_dir_path,
                    wav_file_paths,
                } => {
                    let _ = create_samplechain_from_pathbufs_only(
                        &wav_file_paths,
                        &out_dir_path,
                        &chain_name,
                    );
                }
                crate::cli::SampleChains::CreateN { yaml_file_path } => {
                    let _ = create_samplechains_from_yaml(&yaml_file_path);
                }
                crate::cli::SampleChains::Deconstruct {
                    ot_file_path,
                    audio_file_path,
                    out_dir_path,
                } => {
                    let _ = deconstruct_samplechain_from_pathbufs_only(
                        &audio_file_path,
                        &ot_file_path,
                        &out_dir_path,
                    );
                }
                crate::cli::SampleChains::DeconstructN { yaml_file_path } => {
                    let _ = deconstruct_samplechains_from_yaml(&yaml_file_path);
                }
            },
            crate::cli::Samples::Grid(y) => match y {
                crate::cli::SampleSliceGrid::Random {
                    wav_file_path,
                    n_slices,
                } => {
                    let _ = create_randomly_sliced_sample(&wav_file_path, n_slices);
                }
                crate::cli::SampleSliceGrid::Linear {
                    wav_file_path,
                    n_slices,
                } => {
                    let _ = create_equally_sliced_sample(&wav_file_path, n_slices);
                }
            },
            crate::cli::Samples::Otfile(y) => match y {
                cli::Otfile::Inspect { path } => {
                    let _ = serde_octatrack::show_type::<SampleAttributes>(&path, None);
                }
                cli::Otfile::InspectBytes {
                    path,
                    byte_start_idx,
                    n_bytes,
                } => {
                    let _ = show_ot_file_bytes(&path, &byte_start_idx, &n_bytes);
                }
                cli::Otfile::CreateDefault {
                    wav_file_path: _wav_file_path,
                } => {
                    unimplemented!();
                }
                cli::Otfile::Dump {
                    ot_file_path,
                    yaml_file_path,
                } => {
                    let _ = serde_octatrack::bin_file_to_yaml_file::<SampleAttributes>(
                        &ot_file_path,
                        &yaml_file_path,
                    );
                }
                cli::Otfile::Load {
                    yaml_file_path,
                    ot_file_path,
                } => {
                    let _ = serde_octatrack::yaml_file_to_bin_file::<SampleAttributes>(
                        &yaml_file_path,
                        &ot_file_path,
                    );
                }
            },
            crate::cli::Samples::Search(y) => match y {
                crate::cli::SampleSearch::Simple {
                    samples_dir_path,
                    yaml_file_path,
                } => {
                    let _ = create_index_samples_dir_simple(&samples_dir_path, &yaml_file_path);
                }
                crate::cli::SampleSearch::Full {
                    samples_dir_path,
                    yaml_file_path,
                } => {
                    let _ = create_index_samples_dir_full(&samples_dir_path, &yaml_file_path);
                }
            },
        },
    };
}
