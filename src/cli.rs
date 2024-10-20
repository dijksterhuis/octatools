//! Configuration settings for creating the command line interface arguments.

use std::path::PathBuf;

use clap::{command, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/*
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate index files by recursively scanning through 
    /// an Octatrack Compact Flash card or a local samples directory 
    #[command(subcommand)]
    Index(Index),
    /// Inspect differences between an index file and it's source
    #[command(subcommand)]
    Diff(Diff),
    /// Create audio sample chains
    #[command(subcommand)]
    Chain(Chain),
    /// Push changes
    #[command(subcommand)]
    Push(Push),
}
*/

#[derive(Subcommand, Debug)]
pub enum Commands {
    ScanSamplesDir {
        samples_dir_path: PathBuf,
        csv_file_path: Option<PathBuf>
    },
    ScanCfCard {
        cfcard_dir_path: PathBuf,
        csv_file_path: Option<PathBuf>
    },
    CreateChainsYaml {
        yaml_file_path: PathBuf,
    },
}


#[derive(Subcommand, Debug)]
enum Chain {
    /// Create a simple sample chain from source files
    Construct { 
        /// Directory path where the sample chain audio file and .ot file will be written
        // chain_dir_path: String,
        /// File name for both audio and .ot files.
        // chain_name: String,
        /// Paths to the audio files to include in the sample chain.
        // audio_file_paths: Vec<String>,

        /// The '.samples-index' file which holds sample chains configs (will be updated during processing)
        samples_index_file_path: String,
    },
    /// Use an Octatrack '.ot' file to deconstruct a sample chain into component parts
    Deconstruct { 
        /// Path to the '.ot' file to use for deconstruction.
        ot_file_path: String,
        /// Path to the audio file to use for deconstruction.
        audio_file_path: String,
        /// Directory path where the audio files will be written
        out_dir_path: String,
        /// \[OPTIONAL\] Which '.samples-index' file these chains will belong to
        samples_index_file_path: Option<String>,
    },

    // TODO: EHHHHH don't like the idea of adding a new index file here.
    /// Use a '.combi-index' file to generate combinatorial sample chains for a selection of samples
    Combinator { 
        /// Which '.samples-index' file to add these chains to
        samples_index_file_path: String,
        /// '.combi-index' file to generate the chains from
        combi_index_file_path: String,
    },
}

#[derive(Subcommand, Debug)]
enum Index {
    /// Recursively scan local directories for audio sample files and build an index
    Samples { 
        /// Output location of the '.samples-index' file.
        index_file_path: String,
        /// Directory paths of where to scan for samples
        source_dir_paths: Vec<String>,
        /// Create a Octatrack compatible copy of any audio sample files
        /// that are not suitable for use on an Octatrack 
        /// (44.1kHz 16-bit WAV files)
        #[arg(long, required = false, default_value_t = false)]
        convert: bool,
    },
    /// Scan a CF Card with Octatrack sets and build an index file for it
    Cf {
        /// Location to write the '.cf-index' file
        index_file_path: String,
        /// Directory path on your machine to the CF Card,
        /// on Windows this would be 'X:\' or similar
        cf_card_dir_path: String,
        /// Overwrite existing 'index' file if it exists
        /// (default behaviour is to exit if index exists)
        #[arg(long, required = false, default_value_t = false)]
        force: bool,
    },
}

#[derive(Subcommand, Debug)]
enum Diff {
    /// List differences in the '.samples-index' file versus a samples directory
    Samples { 
        /// Location of an existing '.samples-index' file
        samples_index_file_path: String,
        /// Directory path of where to scan for samples
        saples_dir_path: String,
    },
    /// List differences in the '.stage-index' file versus a 'cf-index' file
    Stage { 
        /// Location of an existing '.stage-index' file
        stage_index_file_path: String,
        /// Location of an existing '.cf-index' file
        cf_index_file_path: String,
    },
    /// List differences in the '.cf-index' file versus an Octatrack CF Card
    Cf { 
        /// Location of an existing '.cf-index' file for the CF card
        cf_index_file_path: String,
        /// Directory path on your machine to the CF Card,
        /// on Windows this would be 'X:\' or similar
        cf_card_dir_path: String,
    },
}

#[derive(Subcommand, Debug)]
enum Push {
    /// Create a '.stage-index' file from a '.samples-index' file.
    Stage { 
        /// Location of the '.samples-index' file.
        samples_index_file_path: String,
        /// Location of the '.cf-index' file -- used to inspect for any problematic samples and/or chages.
        cf_index_file_path: String,
        /// Output location of the '.stage-index' file.
        stage_index_file_path: String,
        /// Default behaviour for this command is to do dry runs for safety reasons,
        /// provide this flag to make changes
        #[arg(long, required = false, default_value_t = false)]
        commit: bool,
        /// Default behaviour is to skip any changes that could cause a conflict,
        /// provide this flag to overrule
        #[arg(long, required = false, default_value_t = false)]
        ignore_conflicts: bool,
    },
    /// Reads a '.stage-index' file and pushes the changes to an Octratrack Compact Flash card.
    /// WARNING -- this is a dangerous operation as it involves writing data to your CF Card!
    Cf { 
        /// Location of an existing '.stage-index' file for the CF card
        index_file_path: String,
        /// Directory path on your machine to the CF Card,
        /// on Windows this would be 'X:\' or similar
        cf_card_dir_path: String,
        /// Default behaviour for this command is to do dry runs for safety reasons,
        /// provide this flag to make changes
        #[arg(long, required = false, default_value_t = false)]
        commit: bool,
    },
}
