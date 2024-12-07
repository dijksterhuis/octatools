//! Module for CLAP based CLI arguments.

use std::path::PathBuf;

use clap::{command, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, long_about = None, about = "CLI tools for the Elektron Octatrack DPS (Dynamic Performance Sampler).")]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Commands related to data in Octatrack Arrangment files (examples: arr01.work, arr01.strd)
#[derive(Subcommand, Debug)]
// #[command(flatten_help = true)]
pub enum Arrangements {
    /// Print the deserialised form of Arrangement data
    Inspect { path: PathBuf },
    /// Print the deserialised raw u8 byte values of Arrangement data
    InspectBytes {
        path: PathBuf,
        byte_start_idx: Option<usize>,
        n_bytes: Option<usize>,
    },

    /// Dump the Arrangement data to a YAML file
    Dump {
        /// File path of the arrangement file
        arrangement_file_path: PathBuf,
        /// Destination yaml file
        yaml_file_path: PathBuf,
    },

    /// Load Arrangement data from a YAML file
    Load {
        /// Source yaml file
        yaml_file_path: PathBuf,
        /// File path of the arrangement file
        arrangement_file_path: PathBuf,
    },
}

#[derive(Subcommand, Debug)]
// #[command(flatten_help = true)]
pub enum ProjectData {
    /// Print the deserialised form of Project data
    Inspect {
        /// File path of the project.work or project.strd file to inspect
        path: PathBuf,
    },
}

#[derive(Subcommand, Debug)]
pub enum SampleSlots {
    /// Print the deserialised form of a Project's Sample Slot data.
    Inspect {
        /// File path of the project.work or project.strd file
        // #[arg(required=true)]
        path: PathBuf,
    },

    /// List Sample Slots used in a Project.
    List {
        /// File path of the project.work or project.strd file
        // #[arg(required=true)]
        path: PathBuf,
    },

    /// Remove Project Sample Slots when the slot is not beging used in any related Bank files.
    Purge {
        /// Project directory path, NOT the project.work/project.strd file (command needs to inspect related bank files)
        path: PathBuf,
    },

    /// Copy relevant audio files to the Project directory
    Consolidate {
        /// Project directory path, NOT the project.work/project.strd file (command needs to find the related Set Audio Pool)
        path: PathBuf,
    },

    /// Copy relevant audio files to the Project Set's Audio Pool directory
    Centralise {
        /// Project directory path, NOT the project.work/project.strd file (command needs to find the related Set Audio Pool)
        path: PathBuf,
    },
}

/// Commands related to data contained in Octatrack Project files (examples: project.work, project.strd)
#[derive(Subcommand, Debug)]
#[command(flatten_help = true)]
#[command(help_template = "{name}: {about}\n{usage-heading}\n{tab}{tab}{tab} {usage}")]
pub enum Projects {
    /// Print the deserialised form of all Project data.
    Inspect {
        /// File path of the project.work or project.strd file
        path: PathBuf,
    },

    /// Specific commands for Project Metadata
    #[command(subcommand)]
    #[command(flatten_help = true)]
    Metadata(ProjectData),

    /// Specific commands for Project State
    #[command(subcommand)]
    #[command(flatten_help = true)]
    State(ProjectData),

    /// Specific commands for Project Settings
    #[command(subcommand)]
    Settings(ProjectData),

    /// Specific commands for Project Sample Slots
    #[command(subcommand)]
    #[command(flatten_help = true)]
    Sampleslots(SampleSlots),

    /// Dump the current Project metadata, state, settings and slots to a YAML file
    Dump {
        /// File path of the project.work or project.strd file
        project_file_path: PathBuf,
        /// Destination yaml file
        yaml_file_path: PathBuf,
    },

    /// Load an existing Project file with metadata, state, settings and slots from a YAML file
    Load {
        /// Source yaml file
        yaml_file_path: PathBuf,
        /// Path to the destination project.work or project.strd file that will be created
        project_file_path: PathBuf,
    },
}

/// Commands related to data contained in Octatrack Bank files (examples: bank01.work, bank01.strd)
#[derive(Subcommand, Debug)]
#[command(flatten_help = true)]
#[command(help_template = "{name}: {about}\n{usage-heading}\n{tab}{tab}{tab} {usage}")]
pub enum Banks {
    /// Print the deserialised form of a Bank file.
    Inspect {
        /// File path of the bank??.work or bank??.strd file
        path: PathBuf,
    },

    /// Print the raw deserialised u8 byte values of a Bank file.
    InspectBytes {
        /// File path of the bank??.work or bank??.strd file
        path: PathBuf,
        byte_start_idx: Option<usize>,
        n_bytes: Option<usize>,
    },

    /// Move a Bank from one location to another,
    /// updating active sample slot assignments in destination Projects while moving.
    Copy {
        /// File path of the source `bank??.work` or `bank??.strd` file
        source_bank_filepath: PathBuf,
        /// File path of the source project.work or project.strd file
        source_project_filepath: PathBuf,
        /// File path of the destination `bank??.work` or `bank??.strd` file
        destination_bank_filepath: PathBuf,
        /// File path of the destination `project.work` or `project.strd` file
        destination_project_filepath: PathBuf,
    },

    /// Move Nx Banks from one location to another,
    /// updating active sample slot assignments in destination Projects while moving.
    CopyN {
        /// File path of the YAML config for the changes
        yaml_file_path: PathBuf,
    },

    /// Dump data from a Bank file to a YAML file
    Dump {
        /// File path of the source bank??.work or bank??.strd file
        bank_file_path: PathBuf,
        /// File path of the destination YAML data dump
        yaml_file_path: PathBuf,
    },

    /// Write Bank data with content of a YAML file
    Load {
        /// File path of the source YAML data dump
        yaml_file_path: PathBuf,
        /// File path of the destiantion bank??.work or bank??.strd file
        bank_file_path: PathBuf,
    },
}

/// Commands related to Pattern data in Octatrack Bank files (examples: bank01.work, bank01.strd)
#[derive(Subcommand, Debug)]
#[command(flatten_help = true)]
#[command(help_template = "{name}: {about}\n{usage-heading}\n{tab}{tab}{tab} {usage}")]
pub enum Patterns {
    /// Show the deserialised representation of one or more Patterns
    Inspect {
        bank_file_path: PathBuf,
        index: Vec<usize>,
    },
}

#[derive(Subcommand, Debug)]
#[command(flatten_help = true)]
#[command(help_template = "{name}: {about}\n{usage-heading}\n{tab}{tab}{tab} {usage}")]
pub enum PartsCmd {
    /// Show the deserialised representation of one or more Parts
    Inspect {
        bank_file_path: PathBuf,
        index: Vec<usize>,
    },
}

/// Commands related to Part data in Octatrack Bank files (examples: bank01.work, bank01.strd)
#[derive(Subcommand, Debug)]
#[command(flatten_help = true)]
#[command(help_template = "{name}: {about}\n{usage-heading}\n{tab}{tab}{tab} {usage}")]
pub enum Parts {
    /// Commands related to SAVED Part data in Octatrack Bank files (examples: bank01.work, bank01.strd)
    #[command(subcommand)]
    Saved(PartsCmd),

    /// Commands related to UNSAVED Part data in Octatrack Bank files (examples: bank01.work, bank01.strd)
    #[command(subcommand)]
    Unsaved(PartsCmd),
}

#[derive(Subcommand, Debug)]
#[command(flatten_help = true)]
#[command(help_template = "{name}: {about}\n{usage-heading}\n{tab}{tab}{tab} {usage}")]
pub enum SampleSliceGrid {
    /// Create an otfile with random slice grid from the cli
    Random {
        /// Location of the audio file to generate a random slices for
        wav_file_path: PathBuf,

        /// How many random slices to create
        n_slices: usize,
    },
    /// Create an otfile with linear slice grid from the cli
    Linear {
        /// Location of the audio file to generate a linear grid for
        wav_file_path: PathBuf,

        /// How many slices to put in the slice grid
        n_slices: usize,
    },
}

/// Commands related to creating sliced 'chains' for Nx audio files.
#[derive(Subcommand, Debug)]
#[command(flatten_help = true)]
#[command(help_template = "{name}: {about}\n{usage-heading}\n{tab}{tab}{tab} {usage}")]
pub enum SampleChains {
    Create {
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
    CreateN {
        /// File path of the YAML file for batched samplechains construction.
        yaml_file_path: PathBuf,
    },
    /// Use the CLI to deconstruct an individual sliced samplechain.
    Deconstruct {
        /// Path to the '.ot' file to use for deconstruction.
        ot_file_path: PathBuf,
        /// Path to the audio file to use for deconstruction.
        audio_file_path: PathBuf,
        /// Directory path where the audio files will be written
        out_dir_path: PathBuf,
    },
    /// Use a YAML config to deconstruct batches of sliced samplechains.
    DeconstructN {
        /// File path of the YAML file.
        yaml_file_path: PathBuf,
    },
}

/// Commands related to finding compatible audio files to use with an Octatrack
#[derive(Subcommand, Debug)]
#[command(flatten_help = true)]
#[command(help_template = "{name}: {about}\n{usage-heading}\n{tab}{tab}{tab} {usage}")]
pub enum SampleSearch {
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

/// Commands related to Sample Attributes data in Octatrack 'OT' files (examples: sampleName.ot, anotherSampleName.ot)
#[derive(Subcommand, Debug)]
#[command(flatten_help = true)]
#[command(help_template = "{name}: {about}\n{usage-heading}\n{tab}{tab}{tab} {usage}")]
pub enum Otfile {
    /// Print the deserialised form of a sample attributes (ot) file.
    Inspect {
        /// File path of the *.ot file
        path: PathBuf,
    },

    /// Print the raw deserialised u8 byte values of a sample attributes (ot) file.
    InspectBytes {
        /// File path of the *.ot file
        path: PathBuf,
        byte_start_idx: Option<usize>,
        n_bytes: Option<usize>,
    },

    CreateDefault {
        /// create a default sample attributes ('.ot') file for wav files
        wav_file_path: Vec<PathBuf>,
    },

    /// Dump Part data from a Bank file to a YAML file
    Dump {
        ot_file_path: PathBuf,
        yaml_file_path: PathBuf,
    },

    /// Write Part data in a Bank file with content of a YAML file
    Load {
        yaml_file_path: PathBuf,
        ot_file_path: PathBuf,
    },
}

/// Commands related to Sample Attributes data in Octatrack 'OT' files (examples: sampleName.ot, anotherSampleName.ot)
#[derive(Subcommand, Debug)]
#[command(flatten_help = true)]
#[command(help_template = "{name}: {about}\n{usage-heading}\n{tab}{tab}{tab} {usage}")]
pub enum Samples {
    #[command(subcommand)]
    Chain(SampleChains),

    /// Commands related to creating slices for existing audio files.
    #[command(subcommand)]
    Grid(SampleSliceGrid),

    #[command(subcommand)]
    Otfile(Otfile),

    #[command(subcommand)]
    Search(SampleSearch),
}

/// Commands related to the 'Drive' i.e. the whole Compact Flash Card
#[derive(Subcommand, Debug)]
#[command(flatten_help = true)]
#[command(help_template = "{name}: {about}\n{usage-heading}\n{tab}{tab}{tab} {usage}")]
pub enum Drive {
    /// Generate an in-depth YAML representation of Octatrack data for a Set.
    /// WARNING: A 1x Project Set will generate a circa 200MB YAML file!
    Dump {
        /// Directory path of the Compact Flash Card directory
        cfcard_dir_path: PathBuf,

        /// File path location where the output YAML file will be written
        yaml_file_path: Option<PathBuf>,
    },
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(subcommand)]
    Arrangements(Arrangements),

    #[command(subcommand)]
    Banks(Banks),

    #[command(subcommand)]
    Drive(Drive),

    #[command(subcommand)]
    Parts(Parts),

    #[command(subcommand)]
    Patterns(Patterns),

    #[command(subcommand)]
    Projects(Projects),

    #[command(subcommand)]
    Samples(Samples),
}
