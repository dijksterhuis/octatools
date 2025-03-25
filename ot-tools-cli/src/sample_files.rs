use crate::print_err;
use clap::{Subcommand, ValueHint};
use ot_tools_ops::actions::samples::{
    batch_create_samplechains, create_equally_sliced_sample, create_randomly_sliced_sample,
    create_samplechains_from_yaml, deconstruct_samplechain_from_paths,
    deconstruct_samplechains_from_yaml,
};
use std::path::PathBuf;

/// Create sample chains, slice grids and other utilities for audio sample files
#[derive(Subcommand, Debug, PartialEq)]
pub(crate) enum SubCmds {
    /// Create a sample chain from the CLI
    Chain {
        /// Name of the new sliced sample chain.
        /// Will automatically be suffixed with an index number
        /// e.g. 'my_sample_chain_0'
        #[arg(value_hint = ValueHint::Other)]
        chain_name: String,

        /// Directory path where the audio files will be written
        #[arg(value_hint = ValueHint::DirPath)]
        out_dir_path: PathBuf,

        /// File paths of wav files to include in the sliced sample chain.
        /// Shell glob patterns work here too.
        #[arg(value_hint = ValueHint::FilePath)]
        wav_file_paths: Vec<PathBuf>,
    },
    /// Create batches of sample chains from a YAML config file
    ChainYaml {
        /// File path of the YAML file for batched samplechains construction.
        #[arg(value_hint = ValueHint::FilePath)]
        yaml_file_path: PathBuf,
    },
    /// Use the CLI to split an individual sample using slices.
    SplitSlices {
        /// Path to the '.ot' file to use for deconstruction.
        #[arg(value_hint = ValueHint::FilePath)]
        ot_file_path: PathBuf,
        /// Path to the audio file to use for deconstruction.
        #[arg(value_hint = ValueHint::FilePath)]
        audio_file_path: PathBuf,
        /// Directory path where the audio files will be written
        #[arg(value_hint = ValueHint::DirPath)]
        out_dir_path: PathBuf,
    },
    /// Use a YAML config to split batches of sliced samples.
    SplitSlicesYaml {
        /// File path of the YAML file.
        #[arg(value_hint = ValueHint::FilePath)]
        yaml_file_path: PathBuf,
    },
    /// Create an `.ot` file with random slice grid from the cli
    GridRandom {
        /// Location of the audio file to generate a random slices for
        #[arg(value_hint = ValueHint::FilePath)]
        wav_file_path: PathBuf,

        /// How many random slices to create
        #[arg(value_hint = ValueHint::Other)]
        n_slices: usize,
    },
    /// Create an `.ot` file with linear slice grid from the cli
    GridLinear {
        /// Location of the audio file to generate a linear grid for
        #[arg(value_hint = ValueHint::FilePath)]
        wav_file_path: PathBuf,

        /// How many slices to put in the slice grid
        #[arg(value_hint = ValueHint::Other)]
        n_slices: usize,
    },
    // #[command(subcommand)]
    // Search(SampleSearch),
}

#[doc(hidden)]
pub(crate) fn subcmd_runner(x: SubCmds) {
    match x {
        SubCmds::Chain {
            chain_name,
            out_dir_path,
            wav_file_paths,
        } => {
            print_err(|| {
                batch_create_samplechains(
                    &wav_file_paths,
                    &out_dir_path,
                    &chain_name,
                    None, // no detailed options allowed from command line
                    None,
                    None,
                )
            });
        }
        SubCmds::ChainYaml { yaml_file_path } => {
            print_err(|| create_samplechains_from_yaml(&yaml_file_path));
        }
        SubCmds::SplitSlices {
            ot_file_path,
            audio_file_path,
            out_dir_path,
        } => {
            let _ =
                deconstruct_samplechain_from_paths(&audio_file_path, &ot_file_path, &out_dir_path);
        }
        SubCmds::SplitSlicesYaml { yaml_file_path } => {
            print_err(|| deconstruct_samplechains_from_yaml(&yaml_file_path));
        }
        SubCmds::GridRandom {
            wav_file_path,
            n_slices,
        } => {
            print_err(|| create_randomly_sliced_sample(&wav_file_path, n_slices));
        }
        SubCmds::GridLinear {
            wav_file_path,
            n_slices,
        } => {
            print_err(|| create_equally_sliced_sample(&wav_file_path, n_slices));
        }
    }
}
