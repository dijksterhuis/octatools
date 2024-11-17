//! Module for CLAP based CLI arguments.

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
    Index(Indexing),

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
    /// Inspect a Project
    Project {
        /// Path to the Project file.
        path: PathBuf,
    },

    /// Inspect a Bank
    Bank {
        /// Path to the Bank file.
        path: PathBuf,
    },

    /// Inspect all the Parts data within a Bank
    Parts {
        /// Path to the Bank file containing all the Parts to inspect.
        path: PathBuf,
    },

    /// Inspect a specific Part within a Bank
    Part {
        /// Path to the Bank file containing a specific Part to inspect.
        path: PathBuf,
        /// The Part number (1/2/3/4)
        index: usize,
    },

    /// Inspect all the Patterns within a Bank
    Patterns {
        /// Path to the Bank file containing all the Patterns to inspect.
        path: PathBuf,
    },

    /// Inspect a specific Pattern within a Bank
    Pattern {
        /// Path to the Bank file containing a specific Pattern to inspect.
        path: PathBuf,
        /// The Pattern number (1/2/3/4/5/6/7/8/9/10/11/12/13/14/15/16)
        index: usize,
    },

    /// Inspect a Sample Attributes file for an audio sample
    Sample {
        /// Path to the `.ot` Sample Attributes file.
        path: PathBuf,
    },

    /// Inspect an Arrangement file for a Project.
    Arrangement {
        /// Path to the arrangement file.
        path: PathBuf,
    },

    /// Inspect a Markers file from a Project.
    Markers {
        /// Path to the markers file.
        path: PathBuf,
    },

    #[command(subcommand)]
    Bytes(InspectBytes),
}

/// Inspect Octatrack data file contents as the raw byte streams (to be used in for reverse engineering / debugging).
#[derive(Subcommand, Debug)]
pub enum InspectBytes {
    /// Inspect a Bank as u8 bytes
    Bank {
        /// Path to the Bank file.
        path: PathBuf,
        start_idx: Option<usize>,
        nbytes: Option<usize>,
    },

    /// Inspect a Sample Attributes file for an audio sample as u8 bytes
    Sample {
        /// Path to the `.ot` Sample Attributes file.
        path: PathBuf,
        start_idx: Option<usize>,
        nbytes: Option<usize>,
    },

    /// Inspect an Arrangement file for a Project as u8 bytes.
    Arrangement {
        /// Path to the arrangement file.
        path: PathBuf,
        start_idx: Option<usize>,
        nbytes: Option<usize>,
    },
}

/// Transfer Octatrack Project(s)/Bank(s) to new location(s).
#[derive(Subcommand, Debug)]
pub enum Transfer {
    /// Transfer one source Bank to the new Bank location.
    Bank {
        /// Bank file to copy.
        source_bank_file_path: PathBuf,

        /// Destination Bank file to replace.
        dest_bank_file_path: PathBuf,
    },
    /// Batched transfers of source Banks to multiple destination Banks.
    Banks {
        /// Yaml config file path.
        yaml_config_path: PathBuf,
    },

    /// Transfer a Project from one Set to another Set.
    Project {
        /// Project data file or directory path of the project
        source_project: PathBuf,

        /// Destination Set for the new project location
        dest_set_dir_path: PathBuf,
    },
    /// Batch transfer of Project(s).
    Projects {
        /// Yaml config to manage the batched copying.
        yaml_config_path: PathBuf,
    },
}

/// Create/Deconstruct sliced sample chains.
#[derive(Subcommand, Debug)]
pub enum Chains {
    /// Create a single sample chain from the cli
    CreateChain {
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
    CreateChains {
        /// File path of the YAML file for batched samplechains construction.
        yaml_file_path: PathBuf,
    },
    /// Use the CLI to deconstruct an individual sliced samplechain.
    DeconstructChain {
        /// Path to the '.ot' file to use for deconstruction.
        ot_file_path: PathBuf,
        /// Path to the audio file to use for deconstruction.
        audio_file_path: PathBuf,
        /// Directory path where the audio files will be written
        out_dir_path: PathBuf,
    },
    /// Use a YAML config to deconstruct batches of sliced samplechains.
    DeconstructChains {
        /// File path of the YAML file.
        yaml_file_path: PathBuf,
    },
}

/// Generate YAML files after scanning / searching various places.
#[derive(Subcommand, Debug)]
pub enum Indexing {
    /// Creates a YAML file output just listing all compatible files.
    SamplesdirSimple {
        /// Path to the top of the directory tree to search through.
        samples_dir_path: PathBuf,

        /// File path for the output YAML file
        yaml_file_path: Option<PathBuf>,
    },

    /// Creates a YAML file output including useful file metadata.
    SamplesdirFull {
        /// Path to the top of the directory tree to search through.
        samples_dir_path: PathBuf,

        /// File path for the output YAML file
        yaml_file_path: Option<PathBuf>,
    },

    /// Generate an in-depth YAML representation of Octatrack data for a Set.
    /// WARNING: A 1x Project Set will generate a circa 200MB YAML file!
    Cfcard {
        /// Directory path of the Compact Flash Card directory
        cfcard_dir_path: PathBuf,

        /// File path location where the output YAML file will be written
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
