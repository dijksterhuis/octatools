//! # `octatools`
//!
//! CLI tools to interact with data files used by the [Elektron OctaTrack DPS](https://www.elektron.se/en/octratrack-mkii-explorer)

mod actions;
mod audio;
mod cli;
mod common;
mod octatrack_sets;
mod utils;

use clap::{Command, CommandFactory, Parser};
use clap_complete::{generate, Generator, Shell};

use env_logger::{Builder, Target};
use log::LevelFilter;

use crate::actions::{
    arrangements::show_arrangement_bytes,
    banks::{
        batch_copy_banks, copy_bank_by_paths, list_bank_sample_slot_references, show_bank_bytes,
    },
    drive::create_file_index_yaml,
    parts::{
        list_saved_part_sample_slot_references, list_unsaved_part_sample_slot_references,
        show_saved_parts, show_unsaved_parts,
    },
    patterns::{list_pattern_sample_slot_references, show_pattern},
    projects::slots::cmd_slots_deduplicate,
    projects::{
        consolidate_sample_slots_to_audio_pool, consolidate_sample_slots_to_project_pool,
        list_project_sample_slots, purge_project_pool,
    },
    samples::{
        create_default_ot_file_for_wav_file, create_default_ot_files_for_wav_files,
        create_equally_sliced_sample, create_index_samples_dir_full,
        create_index_samples_dir_simple, create_randomly_sliced_sample,
        create_samplechain_from_pathbufs_only, create_samplechains_from_yaml,
        deconstruct_samplechain_from_paths, deconstruct_samplechains_from_yaml, show_ot_file_bytes,
    },
};
use cli::{Cli, Commands};
use octatools_lib::arrangements::ArrangementFile;
use octatools_lib::banks::Bank;
use octatools_lib::projects::Project;
use octatools_lib::samples::SampleAttributes;
use octatools_lib::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::Display;
use std::io;
use std::io::Write;
use std::path::PathBuf;

pub type RBoxErr<T> = Result<T, Box<dyn Error>>;
pub type RVoidError<T> = Result<T, ()>;

#[derive(Debug)]
pub enum OctatoolErrors {
    PathDoesNotExist,
    PathIsNotADirectory,
    PathIsNotAFile,
    PathIsNotASet,
    CliInvalidPartIndex,
    CliMissingPartIndex,
    CliInvalidPatternIndex,
    CliMissingPatternIndex,
    InvalidFilenameOrExtension,
    CliInvalidBankIndex,
    CliCopyNoSourceSlotAudioFileFound,
    CliNoFreeSampleSlots,
    // not in use yet
    CliInvalidTrackIndex,
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
            Self::CliMissingPartIndex => write!(
                f,
                "Missing part number(s) - part number(s) between 1-4 (inclusive) must be be provided"
            ),
            Self::CliInvalidPartIndex => write!(
                f,
                "Invalid part number(s) - only part numbers between 1-4 (inclusive) can be provided"
            ),
            Self::CliMissingPatternIndex => write!(
                f,
                "Missing pattern number(s) - pattern number(s) between 1-16 (inclusive) must be provided"
            ),
            Self::CliInvalidPatternIndex => write!(
                f,
                "Invalid pattern number(s) - only numbers between 1-16 (inclusive) can be provided"
            ),
            Self::InvalidFilenameOrExtension => write!(f, "path does not have a file extension"),
            Self::CliInvalidBankIndex => write!(
                f,
                "Invalid bank number(s) - only numbers between 1-16 (inclusive) can be provided"
            ),
            Self::CliCopyNoSourceSlotAudioFileFound => write!(
                f,
                "Could not find an associated audio file for source project sample slot",
            ),
            Self::CliNoFreeSampleSlots => write!(
                f,
                "Not enough sample slots in the project!.",
            ),
            // not in use yet
            Self::CliInvalidTrackIndex => write!(
                f,
                "Invalid track number(s) - only numbers between 1-8 can be provided"
            ),
            Self::Unknown => write!(f, "unknown error (please investigate/report)"),
        }
    }
}
impl std::error::Error for OctatoolErrors {}

#[doc(hidden)]
pub fn print_err<E, F>(cb: F)
where
    F: FnOnce() -> Result<(), E>,
    E: Display,
{
    let r = cb();
    if r.is_err() {
        println!("ERROR: {}", r.unwrap_err());
    }
}

enum ConvertFromTo {
    BinToHuman,
    HumanToBin,
}

#[doc(hidden)]
/// Succinctly handle converting from binary to human-readable and vice versa
fn convert_from_to<T>(
    conversion_type: ConvertFromTo,
    human_type: cli::HumanReadableFileFormat,
    human_path: PathBuf,
    bin_path: PathBuf,
) -> RBoxErr<()>
where
    T: Decode,
    T: Encode,
    T: Serialize,
    T: for<'a> Deserialize<'a>,
{
    match conversion_type {
        ConvertFromTo::BinToHuman => match human_type {
            cli::HumanReadableFileFormat::Json => {
                octatools_lib::bin_file_to_json_file::<T>(&bin_path, &human_path)
            }
            cli::HumanReadableFileFormat::Yaml => {
                octatools_lib::bin_file_to_yaml_file::<T>(&bin_path, &human_path)
            }
        },
        ConvertFromTo::HumanToBin => match human_type {
            cli::HumanReadableFileFormat::Json => {
                octatools_lib::json_file_to_bin_file::<T>(&human_path, &bin_path)
            }
            cli::HumanReadableFileFormat::Yaml => {
                octatools_lib::yaml_file_to_bin_file::<T>(&human_path, &bin_path)
            }
        },
    }
}

#[doc(hidden)]
fn cmd_select_arrangements(x: cli::Arrangements) {
    match x {
        cli::Arrangements::Inspect(cli::Inspect { bin_path }) => {
            print_err(|| octatools_lib::show_type::<ArrangementFile>(&bin_path, None));
        }
        cli::Arrangements::InspectBytes(cli::InspectBytes {
            bin_path,
            start,
            len,
        }) => {
            print_err(|| show_arrangement_bytes(&bin_path, &start, &len));
        }
        cli::Arrangements::CreateDefault(cli::CreateDefault { path }) => {
            print_err(|| octatools_lib::default_type_to_bin_file::<ArrangementFile>(&path));
        }
        cli::Arrangements::BinToHuman(cli::BinToHuman {
            bin_path,
            dest_type,
            dest_path,
        }) => {
            print_err(|| {
                convert_from_to::<ArrangementFile>(
                    ConvertFromTo::BinToHuman,
                    dest_type,
                    dest_path,
                    bin_path,
                )
            });
        }
        cli::Arrangements::HumanToBin(cli::HumanToBin {
            source_type,
            source_path,
            bin_path,
        }) => {
            print_err(|| {
                convert_from_to::<ArrangementFile>(
                    ConvertFromTo::HumanToBin,
                    source_type,
                    source_path,
                    bin_path,
                )
            });
        }
    }
}

#[doc(hidden)]
fn cmd_select_banks(x: cli::Banks) {
    match x {
        cli::Banks::Inspect(cli::Inspect { bin_path }) => {
            print_err(|| octatools_lib::show_type::<Bank>(&bin_path, None));
        }
        cli::Banks::InspectBytes(cli::InspectBytes {
            bin_path,
            start,
            len,
        }) => {
            print_err(|| show_bank_bytes(&bin_path, &start, &len));
        }
        cli::Banks::CreateDefault(cli::CreateDefault { path }) => {
            print_err(|| octatools_lib::default_type_to_bin_file::<Bank>(&path));
        }
        cli::Banks::BinToHuman(cli::BinToHuman {
            bin_path,
            dest_type,
            dest_path,
        }) => {
            print_err(|| {
                convert_from_to::<Bank>(ConvertFromTo::BinToHuman, dest_type, dest_path, bin_path)
            });
        }
        cli::Banks::HumanToBin(cli::HumanToBin {
            source_type,
            source_path,
            bin_path,
        }) => {
            print_err(|| {
                convert_from_to::<Bank>(
                    ConvertFromTo::HumanToBin,
                    source_type,
                    source_path,
                    bin_path,
                )
            });
        }
        cli::Banks::Copy {
            src_project_path,
            dest_project_path,
            src_bank_id,
            dest_bank_id,
        } => {
            print_err(|| {
                copy_bank_by_paths(
                    &src_project_path,
                    &dest_project_path,
                    src_bank_id,
                    dest_bank_id,
                )
            });
        }
        cli::Banks::ListSlotUsage {
            project_path,
            bank_id,
            list_opts,
        } => {
            print_err(|| {
                let cli::ListSlotUsageOpts { exclude_empty } = list_opts;
                list_bank_sample_slot_references(&project_path, bank_id, exclude_empty)
            });
        }
        cli::Banks::CopyN { yaml_file_path } => {
            print_err(|| batch_copy_banks(&yaml_file_path));
        }
    }
}

#[doc(hidden)]
fn cmd_select_drive(x: cli::Drive) {
    match x {
        cli::Drive::Scan {
            cfcard_dir_path,
            yaml_file_path,
        } => {
            print_err(|| create_file_index_yaml(&cfcard_dir_path, &yaml_file_path));
        }
    }
}

#[doc(hidden)]
fn cmd_select_parts(x: cli::Parts) {
    match x {
        cli::Parts::Saved(y) => match y {
            cli::PartsCmd::Inspect { bin_path, index } => {
                print_err(|| show_saved_parts(&bin_path, index));
            }
            cli::PartsCmd::ListSlotUsage {
                project_path,
                bank_id,
                part_id,
                list_opts,
            } => {
                print_err(|| {
                    let cli::ListSlotUsageOpts { exclude_empty } = list_opts;
                    list_saved_part_sample_slot_references(
                        &project_path,
                        bank_id,
                        part_id,
                        exclude_empty,
                    )
                });
            }
        },
        cli::Parts::Unsaved(y) => match y {
            cli::PartsCmd::Inspect { bin_path, index } => {
                print_err(|| show_unsaved_parts(&bin_path, index));
            }
            cli::PartsCmd::ListSlotUsage {
                project_path,
                bank_id,
                part_id,
                list_opts,
            } => {
                print_err(|| {
                    let cli::ListSlotUsageOpts { exclude_empty } = list_opts;
                    list_unsaved_part_sample_slot_references(
                        &project_path,
                        bank_id,
                        part_id,
                        exclude_empty,
                    )
                });
            }
        },
    }
}

#[doc(hidden)]
fn cmd_select_patterns(x: cli::Patterns) {
    match x {
        cli::Patterns::Inspect { bin_path, index } => {
            print_err(|| show_pattern(&bin_path, &index[..]));
        }
        cli::Patterns::ListSlotUsage {
            project_path,
            bank_id,
            pattern_id,
            list_opts,
        } => {
            print_err(|| {
                let cli::ListSlotUsageOpts { exclude_empty } = list_opts;
                list_pattern_sample_slot_references(
                    &project_path,
                    bank_id,
                    pattern_id,
                    exclude_empty,
                )
            });
        }
    }
}

#[doc(hidden)]
fn cmd_select_project(x: cli::Projects) {
    match x {
        cli::Projects::Inspect(cli::Inspect { bin_path }) => {
            print_err(|| octatools_lib::show_type::<Project>(&bin_path, None));
        }
        cli::Projects::CreateDefault(cli::CreateDefault { path }) => {
            print_err(|| octatools_lib::default_type_to_bin_file::<Project>(&path));
        }
        cli::Projects::Settings(y) => match y {
            cli::ProjectData::Inspect(cli::Inspect { bin_path: _ }) => {
                unimplemented!();
            }
        },
        cli::Projects::Metadata(y) => match y {
            cli::ProjectData::Inspect(cli::Inspect { bin_path: _ }) => {
                unimplemented!();
            }
        },
        cli::Projects::State(y) => match y {
            cli::ProjectData::Inspect(cli::Inspect { bin_path: _ }) => {
                unimplemented!();
            }
        },
        cli::Projects::SampleSlots(y) => match y {
            cli::SampleSlots::Inspect(cli::Inspect { bin_path: _ }) => {
                unimplemented!();
            }
            cli::SampleSlots::List { path } => {
                print_err(|| list_project_sample_slots(&path));
            }
            cli::SampleSlots::Purge { path } => {
                print_err(|| purge_project_pool(&path));
            }
            cli::SampleSlots::Consolidate { path } => {
                print_err(|| consolidate_sample_slots_to_project_pool(&path));
            }
            cli::SampleSlots::Centralise { path } => {
                print_err(|| consolidate_sample_slots_to_audio_pool(&path));
            }
            cli::SampleSlots::Deduplicate { project_dirpath } => {
                print_err(|| cmd_slots_deduplicate(&project_dirpath));
            }
        },
        cli::Projects::BinToHuman(cli::BinToHuman {
            bin_path,
            dest_type,
            dest_path,
        }) => {
            print_err(|| {
                convert_from_to::<Project>(
                    ConvertFromTo::BinToHuman,
                    dest_type,
                    dest_path,
                    bin_path,
                )
            });
        }
        cli::Projects::HumanToBin(cli::HumanToBin {
            source_type,
            source_path,
            bin_path,
        }) => {
            print_err(|| {
                convert_from_to::<Project>(
                    ConvertFromTo::HumanToBin,
                    source_type,
                    source_path,
                    bin_path,
                )
            });
        }
    }
}

#[doc(hidden)]
fn cmd_select_samples(x: cli::Samples) {
    match x {
        cli::Samples::Chain(y) => match y {
            cli::SampleChains::Create {
                chain_name,
                out_dir_path,
                wav_file_paths,
            } => {
                print_err(|| {
                    create_samplechain_from_pathbufs_only(
                        &wav_file_paths,
                        &out_dir_path,
                        &chain_name,
                    )
                });
            }
            cli::SampleChains::CreateN { yaml_file_path } => {
                print_err(|| create_samplechains_from_yaml(&yaml_file_path));
            }
            cli::SampleChains::Deconstruct {
                ot_file_path,
                audio_file_path,
                out_dir_path,
            } => {
                let _ = deconstruct_samplechain_from_paths(
                    &audio_file_path,
                    &ot_file_path,
                    &out_dir_path,
                );
            }
            cli::SampleChains::DeconstructN { yaml_file_path } => {
                print_err(|| deconstruct_samplechains_from_yaml(&yaml_file_path));
            }
        },
        cli::Samples::Grid(y) => match y {
            cli::SampleSliceGrid::Random {
                wav_file_path,
                n_slices,
            } => {
                print_err(|| create_randomly_sliced_sample(&wav_file_path, n_slices));
            }
            cli::SampleSliceGrid::Linear {
                wav_file_path,
                n_slices,
            } => {
                print_err(|| create_equally_sliced_sample(&wav_file_path, n_slices));
            }
        },
        cli::Samples::Otfile(y) => match y {
            cli::Otfile::Inspect(cli::Inspect { bin_path }) => {
                print_err(|| octatools_lib::show_type::<SampleAttributes>(&bin_path, None));
            }
            cli::Otfile::InspectBytes(cli::InspectBytes {
                bin_path,
                start,
                len,
            }) => {
                print_err(|| show_ot_file_bytes(&bin_path, &start, &len));
            }
            cli::Otfile::BinToHuman(cli::BinToHuman {
                bin_path,
                dest_type,
                dest_path,
            }) => {
                print_err(|| {
                    convert_from_to::<SampleAttributes>(
                        ConvertFromTo::BinToHuman,
                        dest_type,
                        dest_path,
                        bin_path,
                    )
                });
            }
            cli::Otfile::HumanToBin(cli::HumanToBin {
                source_type,
                source_path,
                bin_path,
            }) => {
                print_err(|| {
                    convert_from_to::<SampleAttributes>(
                        ConvertFromTo::HumanToBin,
                        source_type,
                        source_path,
                        bin_path,
                    )
                });
            }
            cli::Otfile::CreateDefault(cli::CreateDefault { path }) => {
                print_err(|| create_default_ot_file_for_wav_file(&path));
            }
            cli::Otfile::CreateDefaultN { paths } => {
                print_err(|| create_default_ot_files_for_wav_files(&paths));
            }
        },
        cli::Samples::Search(y) => match y {
            cli::SampleSearch::Simple {
                samples_dir_path,
                yaml_file_path,
            } => {
                print_err(|| create_index_samples_dir_simple(&samples_dir_path, &yaml_file_path));
            }
            cli::SampleSearch::Full {
                samples_dir_path,
                yaml_file_path,
            } => {
                print_err(|| create_index_samples_dir_full(&samples_dir_path, &yaml_file_path));
            }
        },
    }
}

#[doc(hidden)]
fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

#[doc(hidden)]
fn cmd_shell_completions(x: cli::ShellCompletions) {
    let mut cli_data = Cli::command();
    match x {
        cli::ShellCompletions::Bash => print_completions(Shell::Bash, &mut cli_data),
        cli::ShellCompletions::Powershell => print_completions(Shell::PowerShell, &mut cli_data),
    }
}

#[doc(hidden)]
fn cmd_help_full() {
    let mut cli_data = Cli::command();

    let mut buf = String::new();
    let mut prefix = String::new();

    /*

    SAMPLES: Some text describing `samples` commands
    ====================================
    samples chain create: some text about chaining
    samples chain create-n: some text about chaining
    sample grid linear: some text about slice grids
    sample grid random: some text about slice grids
    */
    let _ = recursive_walk_subcommands(&mut buf, &mut prefix, &mut cli_data);

    io::stdout().write_all(buf.as_bytes()).unwrap();
    io::stdout().flush().unwrap();
}

#[doc(hidden)]
fn write_command_usage(buffer: &mut String, prefix: &mut String, cmd: &mut Command) {
    /*
    {prefix} command -- Some text describing a specific command
    {prefix} command -- Some text describing a specific command
    {prefix} command -- Some text describing a specific command
    {prefix} command -- Some text describing a specific command
    */

    buffer.push_str(format!("{prefix} {}", cmd.get_name()).as_str());
    if let Some(about) = cmd.get_about() {
        buffer.push_str(format!(" -- {}", about).as_str());
    }
    buffer.push('\n');
}

#[doc(hidden)]
fn write_top_level_header(buffer: &mut String, cmd: &mut Command) {
    /*

    SAMPLES: Some text describing `samples` commands
    ====================================
    */
    buffer.push_str(format!("\n{}", cmd.get_name().to_ascii_uppercase()).as_str());
    if let Some(about) = cmd.get_about() {
        buffer.push_str(format!(": {}\n", about).as_str());
    }
    buffer.push_str("====================================\n");
}

#[doc(hidden)]
fn recursive_walk_subcommands(
    buffer: &mut String,
    prefix: &mut String,
    cmd: &mut Command,
) -> String {
    for sub in cmd.get_subcommands_mut() {
        // some sort of command/subcommand
        if sub.has_subcommands() {
            let mut sub_prefix = prefix.clone();
            if sub_prefix.is_empty() {
                // no existing prefix -- top level command, create a header block
                write_top_level_header(buffer, sub)
            } else {
                // an existing prefix -- is a subcommand so include in list with usage
                sub_prefix.push(' ');
            }
            sub_prefix.push_str(sub.get_name());
            recursive_walk_subcommands(buffer, &mut sub_prefix, sub);
        } else {
            // no subcommands, write usage details
            write_command_usage(buffer, prefix, sub);
        }
    }

    buffer.clone()
}
#[doc(hidden)]
fn main() {
    let mut logger = Builder::new();
    logger.filter_level(LevelFilter::Debug);
    logger.target(Target::Stdout).init();

    match Cli::parse().command {
        Commands::Arrangements(x) => cmd_select_arrangements(x),
        Commands::Banks(x) => cmd_select_banks(x),
        Commands::Drive(x) => cmd_select_drive(x),
        Commands::Patterns(x) => cmd_select_patterns(x),
        Commands::Parts(x) => cmd_select_parts(x),
        Commands::Projects(x) => cmd_select_project(x),
        Commands::Samples(x) => cmd_select_samples(x),
        Commands::ShellCompletion(x) => cmd_shell_completions(x),
        Commands::HelpFull => cmd_help_full(),
    };
}
