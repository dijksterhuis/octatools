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

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(subcommand)]
    List(List),

    #[command(subcommand)]
    Inspect(Inspect),

    #[command(subcommand)]
    Transfer(Transfer),

    #[command(subcommand)]
    Chains(Chains),

    #[command(subcommand)]
    Scan(Indexing),

    // TODOs

    #[command(subcommand)]
    Consolidate(ConsolidateSamples),

    // #[command(subcommand)]
    // Purge(Purge),
}

/// List various things
#[derive(Subcommand, Debug)]
pub enum List {
    /// List all Arrangements in an arrangements file.
    Arrangements {
        /// Path to the arrangement file.
        path: PathBuf,
    },

    /// List all sample slots in a Project
    ProjectSlots {
        /// Path to the Project data file
        path: PathBuf,
    },
}

/// Inspect Octatrack data file contents.
#[derive(Subcommand, Debug)]
pub enum Inspect {
    Project {
        /// Path to the Project file.
        path: PathBuf,
    },

    Bank {
        /// Path to the Bank file.
        path: PathBuf,
    },

    Parts {
        /// Path to the Bank file containing all the Parts to inspect.
        path: PathBuf,
    },

    Part {
        /// Path to the Bank file containing a specific Part to inspect.
        path: PathBuf,
        /// The Part number (1/2/3/4)
        index: usize,
    },

    Patterns {
        /// Path to the Bank file containing all the Patterns to inspect.
        path: PathBuf,
    },

    Pattern {
        /// Path to the Bank file containing a specific Pattern to inspect.
        path: PathBuf,
        /// The Pattern number (1/2/3/4/5/6/7/8/9/10/11/12/13/14/15/16)
        index: usize,
    },

    Sample {
        /// Path to the `.ot` Sample Attributes file.
        path: PathBuf,
    },

}

/// Transfer Octatrack Project(s)/Bank(s) to new location(s).
#[derive(Subcommand, Debug)]
pub enum Transfer {
    #[command(subcommand)]
    Banks(TransferBank),

    #[command(subcommand)]
    Projects(TransferProject),
}

/// Transfer Bank(s) from source location to a new location.
/// Will copy in-use audio files to the destination Set's Audio Pool.
/// WARNING: Will overwrite the destination bank(s)!
#[derive(Subcommand, Debug)]
pub enum TransferBank {
    /// Transfer one source Bank to the new Bank location.
    Cli {
        /// Bank file to copy.
        source_bank_file_path: PathBuf,

        /// Destination Bank file to replace.
        dest_bank_file_path: PathBuf,
    },
    /// Batched transfers of source Banks to multiple destination Banks.
    Yaml {
        /// Yaml config file path.
        yaml_config_path: PathBuf,
    },
}

/// Transfer Projects(s) from Set to a new Set.
/// Will also copy all in-use samples to the new Set's Audio Pool.
#[derive(Subcommand, Debug)]
pub enum TransferProject {
    /// Transfer a Project from one Set to another Set.
    Cli {
        /// Project data file or directory path of the project
        source_project: PathBuf,

        /// Destination Set for the new project location
        dest_set_dir_path: PathBuf,
    },
    /// Batch transfer of Project(s).
    Yaml {
        /// Yaml config to manage the batched copying.
        yaml_config_path: PathBuf,
    },
}

/// Create/Deconstruct sliced sample chains.
#[derive(Subcommand, Debug)]
pub enum Chains {
    #[command(subcommand)]
    Create(CreateChain),

    #[command(subcommand)]
    Deconstruct(DesconstructChain),
}

/// Create sample chains
#[derive(Subcommand, Debug)]
pub enum CreateChain {
    /// Create a single sample chain from the cli
    Cli {
        /// Name of the new sliced samplechain.
        /// Will automatically be suffixed with an index number
        /// e.g. 'my_sample_chain_0'
        chain_name: String,

        /// Directory path where the audio files will be written
        out_dir_path: PathBuf,

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
pub enum DesconstructChain {
    /// Use a YAML config to deconstruct batches of sliced samplechains.
    Yaml {
        /// File path of the YAML file.
        yaml_file_path: PathBuf,
    },

    /// Use the CLI to deconstruct an individual sliced samplechain.
    Cli {
        /// Path to the '.ot' file to use for deconstruction.
        ot_file_path: PathBuf,
        /// Path to the audio file to use for deconstruction.
        audio_file_path: PathBuf,
        /// Directory path where the audio files will be written
        out_dir_path: PathBuf,
    },
}

/// Generate YAML files after scanning / searching various places.
#[derive(Subcommand, Debug)]
pub enum Indexing {
    #[command(subcommand)]
    Samples(IndexSamples),

    /// Build a YAML representation of all Sets on a Compact Flash Card.
    Cfcard {
        /// Directory path of the Compact Flash Card directory
        cfcard_dir_path: PathBuf,

        /// File path location where the output YAML file will be written
        yaml_file_path: Option<PathBuf>,
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


/// Consolidate Project audio files to either the Project or the Set's Audio Pool. 
#[derive(Subcommand, Debug)]
pub enum ConsolidateSamples {
    /// Copy all Project audio files to the Set's Audio Pool
    /// and modify all samples slots to point to the new sample location(s).
    ToPool {
        /// Path to the Project data file.
        path: PathBuf,
    },

    /// Copy all Project audio files to the Project folder
    /// and modify all samples slots to point to the new location.
    ToProject {
        /// Path to the Project data file.
        path: PathBuf,
    },
}


// //////////////////////////////// NOT DOING YET ////////////////////////////////

/// Purge unused audio files.
/// Will only delete files that are not being used in a project.
#[derive(Subcommand, Debug)]
pub enum Purge {
    /// Delete unused samples from a Set's Audio Pool.
    Pool {
        /// Path to the Set or Audio Pool directory.
        set_path: PathBuf,

        /// No destructive actions will be taken without this flag.
        /// You accept all liability for any actions taken by running this command.
        accept_liability: Option<bool>,
    },

    /// Delete unused samples from a Set's Audio Pool.
    Project {
        /// Path to the Project data file or directory path of the project
        project_path: PathBuf,

        /// No destructive actions will be taken without this flag.
        /// You accept all liability for any actions taken by running this command.
        accept_liability: Option<bool>,
    },
}
