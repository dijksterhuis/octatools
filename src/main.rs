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
use log::LevelFilter;

use actions::{
    chains::{
        create_equally_sliced_sample, create_randomly_sliced_sample,
        create_samplechain_from_pathbufs_only, create_samplechains_from_yaml,
        deconstruct_samplechain_from_pathbufs_only, deconstruct_samplechains_from_yaml,
    },
    copy::{batch_copy_banks, copy_bank},
    dump::{dump_arrangement, dump_bank, dump_ot_file, dump_project},
    indexing::{
        create_index_compact_flash_drive_yaml, create_index_samples_dir_full,
        create_index_samples_dir_simple,
    },
    inspect::{
        show_arrangement, show_arrangement_bytes, show_bank, show_bank_bytes, show_ot_file,
        show_ot_file_bytes, show_pattern, show_project, show_saved_parts, show_unsaved_parts,
    },
    list::list_project_sample_slots,
    load::{load_arrangement, load_bank, load_ot_file, load_project},
};

use cli::{Cli, Commands};

#[doc(hidden)]
fn main() {
    let mut logger = Builder::new();
    logger.filter_level(LevelFilter::Debug);
    logger.target(Target::Stdout).init();

    let args = Cli::parse();

    println!("ARGS: {:#?}", args);

    match args.command {
        Commands::Drive(x) => match x {
            crate::cli::Drive::Dump {
                cfcard_dir_path,
                yaml_file_path,
            } => {
                let _ = create_index_compact_flash_drive_yaml(&cfcard_dir_path, &yaml_file_path);
            }
        },
        Commands::Projects(x) => match x {
            crate::cli::Projects::Inspect { path } => {
                let _ = show_project(&path);
            }
            crate::cli::Projects::Settings(y) => match y {
                crate::cli::ProjectData::Inspect { path } => {
                    unimplemented!();
                }
            },
            crate::cli::Projects::Metadata(y) => match y {
                crate::cli::ProjectData::Inspect { path } => {
                    unimplemented!();
                }
            },
            crate::cli::Projects::State(y) => match y {
                crate::cli::ProjectData::Inspect { path } => {
                    unimplemented!();
                }
            },
            crate::cli::Projects::Sampleslots(y) => match y {
                crate::cli::SampleSlots::Inspect { path } => {
                    unimplemented!();
                }
                crate::cli::SampleSlots::List { path } => {
                    let _ = list_project_sample_slots(&path);
                }
                crate::cli::SampleSlots::Purge { path } => {
                    unimplemented!();
                }
                crate::cli::SampleSlots::Consolidate { path } => {
                    unimplemented!();
                }
                crate::cli::SampleSlots::Centralise { path } => {
                    unimplemented!();
                }
            },
            crate::cli::Projects::Dump {
                project_file_path,
                yaml_file_path,
            } => {
                let _ = dump_project(&project_file_path, &yaml_file_path);
            }
            crate::cli::Projects::Load {
                yaml_file_path,
                project_file_path,
            } => {
                let _ = load_project(&yaml_file_path, &project_file_path);
            }
        },
        Commands::Arrangements(x) => match x {
            crate::cli::Arrangements::Inspect { path } => {
                let _ = show_arrangement(&path);
            }
            crate::cli::Arrangements::InspectBytes {
                path,
                byte_start_idx,
                n_bytes,
            } => {
                let _ = show_arrangement_bytes(&path, &byte_start_idx, &n_bytes);
            }
            crate::cli::Arrangements::Dump {
                arrangement_file_path,
                yaml_file_path,
            } => {
                let _ = dump_arrangement(&arrangement_file_path, &yaml_file_path);
            }
            crate::cli::Arrangements::Load {
                yaml_file_path,
                arrangement_file_path,
            } => {
                let _ = load_arrangement(&yaml_file_path, &arrangement_file_path);
            }
        },
        Commands::Banks(x) => match x {
            crate::cli::Banks::Inspect { path } => {
                let _ = show_bank(&path);
            }
            crate::cli::Banks::InspectBytes {
                path,
                byte_start_idx,
                n_bytes,
            } => {
                let _ = show_bank_bytes(&path, &byte_start_idx, &n_bytes);
            }
            crate::cli::Banks::Copy {
                src_bank_file_path,
                dest_bank_file_path,
            } => {
                let _ = copy_bank(&src_bank_file_path, &dest_bank_file_path);
            }
            crate::cli::Banks::CopyN { yaml_file_path } => {
                let _ = batch_copy_banks(&yaml_file_path);
            }
            crate::cli::Banks::Dump {
                bank_file_path,
                yaml_file_path,
            } => {
                let _ = dump_bank(&bank_file_path, &yaml_file_path);
            }
            crate::cli::Banks::Load {
                yaml_file_path,
                bank_file_path,
            } => {
                let _ = load_bank(&yaml_file_path, &bank_file_path);
            }
        },
        Commands::Patterns(x) => match x {
            crate::cli::Patterns::Inspect {
                bank_file_path,
                index,
            } => {
                let _ = show_pattern(&bank_file_path, index);
            }
            crate::cli::Patterns::Copy {
                src_bank_file_path,
                src_pattern_index,
                dest_bank_file_path,
                dest_pattern_index,
            } => {
                unimplemented!();
            }
            crate::cli::Patterns::CopyN { yaml_file_path } => {
                unimplemented!();
            }
            crate::cli::Patterns::Dump {
                bank_file_path,
                pattern_index,
                yaml_file_path,
            } => {
                unimplemented!();
            }
            crate::cli::Patterns::Load {
                yaml_file_path,
                bank_file_path,
                pattern_index,
            } => {
                unimplemented!();
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
                crate::cli::PartsCmd::Copy {
                    src_bank_file_path,
                    src_part_index,
                    dest_bank_file_path,
                    dest_part_index,
                } => {
                    unimplemented!();
                }
                crate::cli::PartsCmd::CopyN { yaml_file_path } => {
                    unimplemented!();
                }
                crate::cli::PartsCmd::Dump {
                    bank_file_path,
                    part_index,
                    yaml_file_path,
                } => {
                    unimplemented!();
                }
                crate::cli::PartsCmd::Load {
                    yaml_file_path,
                    bank_file_path,
                    part_index,
                } => {
                    unimplemented!();
                }
            },
            crate::cli::Parts::Unsaved(y) => match y {
                crate::cli::PartsCmd::Inspect {
                    bank_file_path,
                    index,
                } => {
                    let _ = show_unsaved_parts(&bank_file_path, index);
                }
                crate::cli::PartsCmd::Copy {
                    src_bank_file_path,
                    src_part_index,
                    dest_bank_file_path,
                    dest_part_index,
                } => {
                    unimplemented!();
                }
                crate::cli::PartsCmd::CopyN { yaml_file_path } => {
                    unimplemented!();
                }
                crate::cli::PartsCmd::Dump {
                    bank_file_path,
                    part_index,
                    yaml_file_path,
                } => {
                    unimplemented!();
                }
                crate::cli::PartsCmd::Load {
                    yaml_file_path,
                    bank_file_path,
                    part_index,
                } => {
                    unimplemented!();
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
                crate::cli::Otfile::Inspect { path } => {
                    let _ = show_ot_file(&path);
                }
                crate::cli::Otfile::InspectBytes {
                    path,
                    byte_start_idx,
                    n_bytes,
                } => {
                    let _ = show_ot_file_bytes(&path, &byte_start_idx, &n_bytes);
                }
                crate::cli::Otfile::CreateDefault { wav_file_path } => {
                    unimplemented!();
                }
                crate::cli::Otfile::Dump {
                    ot_file_path,
                    yaml_file_path,
                } => {
                    let _ = dump_ot_file(&ot_file_path, &yaml_file_path);
                }
                crate::cli::Otfile::Load {
                    yaml_file_path,
                    ot_file_path,
                } => {
                    let _ = load_ot_file(&yaml_file_path, &ot_file_path);
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
