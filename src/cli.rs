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
    Chains(Chains),

    #[command(subcommand)]
    Scan(Indexing),

    // Working on this now.
    #[command(subcommand)]
    Copy(Copy),

    // TODOs

    // #[command(subcommand)]
    // List(List),

    // #[command(subcommand)]
    // Consolidate(Consolidate),

    // #[command(subcommand)]
    // Purge(Purge),

}


/// (Safely-ish) Copy Octatrack Projects / Banks between Sets / Projects.
#[derive(Subcommand, Debug)]
pub enum Copy {
    /// Copy a Bank from one Project to another.
    /// Default behaviour will copy in-use audio files to the destination Set's Audio Pool.
    /// WARNING: Will overwrite the destination bank!
    Bank {
        /// Bank file to copy to another project
        source_bank_file_path: PathBuf,

        /// Bank file that will be overwritten with the source Bank file
        dest_bank_file_path: PathBuf,

        /// Copy sample audio files to the destination bank's audio pool
        /// or project folder.
        // TODO: flag
        copy_samples_to_project: Option<bool>,

        /// Edit source bank to deduplicate sample slots when copying to the destination.
        /// WARNING: Source bank sample attribute files (`.ot` files) will be dropped/ignored.
        // TODO: flag
        merge_duplicate_sample_slots: Option<bool>,

        /// No destructive actions will be taken without this flag.
        /// You accept all liability for any actions taken by running this command.
        accept_liability: Option<bool>,
    },

    /// Transfer a Project from one Set to another Set.
    /// Copies active samples to the audio pool of the new project's set.
    Project {
        /// Project data file or directory path of the project
        source_project: PathBuf,

        /// Destination Set for the new project location
        dest_set_dir_path: PathBuf,

        /// Copy audio files to the destination set's audio pool or to the new project.
        copy_samples_to_project: Option<bool>,

        /// No destructive actions will be taken without this flag.
        /// You accept all liability for any actions taken by running this command.
        accept_liability: Option<bool>,
    },
}

/// Commands related to samplechains.
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


// //////////////////////////////// NOT DOING YET ////////////////////////////////

/// List various things
#[derive(Subcommand, Debug)]
pub enum List {

    /// List the different octatrack sets (basically list all top level directories)
    Sets {
        /// Path to the Compact Flash Card data.
        cfcard_path: PathBuf,
    },

    /// List all Projects within a Set.
    Projects {

        /// Path to the Octatrack Set.
        set_path: PathBuf,
    },

    /// List all Arrangements within a Set.
    Arrangements {

        /// Path to the Octatrack Set.
        set_path: PathBuf,
    },

    /// List all slots within a Project
    Slots {
        /// Path to the Project data file or directory path of the project
        project_path: PathBuf,

        /// Only list slots that are in use somewhere within the project
        ignore_inactive: Option<bool>,

        /// Ignore recording buffers when listing flex sample slots
        ignore_buffers: Option<bool>,

        /// Flex sample slots only
        flex_only: Option<bool>,

        /// Static sample slots only
        static_only: Option<bool>,
    },

    /// List all samples within a Project
    Samples {

        /// Project data file or directory path of the project
        source_project: PathBuf,

        /// Only list slots that are in use somewhere within the project
        ignore_inactive: Option<bool>,

        /// Ignore recording buffers when listing flex sample slots
        ignore_buffers: Option<bool>,

        /// Only list Flex assigned samples
        flex_only: Option<bool>,

        /// Only list Static assigned samples
        static_only: Option<bool>,
    },

}


/// Consolidate samples
#[derive(Subcommand, Debug)]
pub enum Consolidate {

    /// Copy all Project audio files to the Set's Audio Pool, 
    /// modify all samples slots to point to the new location
    /// and (optionally) delete the original audio file.
    ToPool {
        /// Path to the Project data file or directory path of the project
        project_path: PathBuf,

        /// Do not delete the original audio files.
        no_delete: Option<bool>,

        /// Ignore recording buffers when listing flex sample slots
        ignore_buffers: Option<bool>,

        /// Flex sample slots only
        flex_only: Option<bool>,

        /// Static sample slots only
        static_only: Option<bool>,
    },

    /// Same as `to-pool`, but reversed -- 
    /// move everything to the Project folder.
    ToProject {
        /// Path to the Project data file or directory path of the project
        project_path: PathBuf,

        /// Do not delete the original audio files.
        no_delete: Option<bool>,

        /// Ignore recording buffers when listing flex sample slots
        ignore_buffers: Option<bool>,

        /// Flex sample slots only
        flex_only: Option<bool>,

        /// Static sample slots only
        static_only: Option<bool>,
    },
}


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