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
    #[command(subcommand)]
    Chains(Chains),

    #[command(subcommand)]
    Scan(Index),

    // ScanSamplesDirFull {
    //     samples_dir_path: PathBuf,
    //     yaml_file_path: Option<PathBuf>,
    // },
    // ScanSamplesDirSimple {
    //     samples_dir_path: PathBuf,
    //     yaml_file_path: Option<PathBuf>,
    // },
    // ScanCfCard {
    //     cfcard_dir_path: PathBuf,
    //     csv_file_path: Option<PathBuf>
    // },
}

/// Commands related to samplechains.
#[derive(Subcommand, Debug)]
pub enum Chains {

    #[command(subcommand)]
    Create(CreateChain),

    #[command(subcommand)]
    Deconstruct(DesconstructChain),

    // // TODO: EHHHHH don't like the idea of adding a new index file here.
    // /// Use a '.combi-index' file to generate combinatorial sample chains for a selection of samples
    // Combinator { 
    //     /// Which '.samples-index' file to add these chains to
    //     samples_index_file_path: String,
    //     /// '.combi-index' file to generate the chains from
    //     combi_index_file_path: String,
    // },
}



/// Generate YAML files after scanning / searching various places.
#[derive(Subcommand, Debug)]
pub enum Index {

    #[command(subcommand)]
    Samples(IndexSamples),

    /// Build a YAML representation of all Sets on a Compact Flash Card.
    Cfcard {

        /// Directory path of the Compact Flash Card directory
        cfcard_dir_path: PathBuf,

        /// File path location where the output YAML file will be written
        yaml_file_path: Option<PathBuf>,
    }
}


/// Create sample chains
#[derive(Subcommand, Debug)]
pub enum CreateChain {

    /// Create a single sample chain from the cli
    Cli {
        /// Name of the new sliced samplechain.
        /// Will be suffixed with an index number.
        chain_name: String,

        /// Directory path where the audio files will be written
        out_dir_path: String,
        
        /// File paths of wav files to include in the sliced sample chain.
        /// Shell glob patterns work here too.
        wav_file_paths: Vec<PathBuf>,
    },

    /// Create batches of sample chains from a YAML config file
    Yaml {
        /// File path of the YAML file for batched samplechains construction.
        yaml_file_path: PathBuf,
    },
}

/// Use an Octatrack '.ot' file to deconstruct a 'sliced' samplechain into component sample files
#[derive(Subcommand, Debug)]
enum DesconstructChain {

    /// Use a YAML config to deconstruct batches of sliced samplechains.
    Yaml {
        /// File path of the YAML file.
        yaml_file_path: PathBuf,
    },

    /// Use the CLI to deconstruct an individual sliced samplechain.
    Cli {
        /// Path to the '.ot' file to use for deconstruction.
        ot_file_path: String,
        /// Path to the audio file to use for deconstruction.
        audio_file_path: String,
        /// Directory path where the audio files will be written
        out_dir_path: String,
    },
}

/// Recursively search through local directories for Octatrack compatible audio files.
#[derive(Subcommand, Debug)]
pub enum IndexSamples {

    /// Creates a YAML file output just listing all compatible files.
    Simple {

        /// Path to the top of the directory tree to search through.
        samples_dir_path: PathBuf,

        /// File path for the output YAML file
        yaml_file_path: Option<PathBuf>,
    },

    /// Creates a YAML file output including useful file metadata.
    Full {

        /// Path to the top of the directory tree to search through.
        samples_dir_path: PathBuf,

        /// File path for the output YAML file
        yaml_file_path: Option<PathBuf>,
    },
}

/*
#[derive(Subcommand, Debug)]
pub enum IndexCfcard {
    /// Output location of the '.samples-index' file.
    index_file_path: String,
    /// Directory paths of where to scan for samples
    source_dir_paths: Vec<String>,
    /// Create a Octatrack compatible copy of any audio sample files
    /// that are not suitable for use on an Octatrack 
    /// (44.1kHz 16-bit WAV files)
    #[arg(long, required = false, default_value_t = false)]
    convert: bool,
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
*/