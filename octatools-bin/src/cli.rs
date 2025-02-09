//! Module for CLAP based CLI arguments.

use clap::{command, Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[doc(hidden)]
#[derive(Parser, Debug)]
#[command(version, long_about = None, about = "CLI tool for handling Elektron Octatrack DPS-1 data files.")]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Available file formats for converting to/from human-readable data formats
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum HumanReadableFileFormat {
    Json,
    Yaml,
}

/// Read a binary data file and print the deserialized output to stdout
#[derive(Args, Debug)]
pub struct Inspect {
    /// Path of the OctaTrack binary data file
    pub bin_path: PathBuf,
}

/// Read a binary data file and print raw u8 byte values to stdout
#[derive(Args, Debug)]
pub struct InspectBytes {
    /// Path of the OctaTrack binary data file
    pub bin_path: PathBuf,
    /// Index of starting byte range to inspect
    pub start: Option<usize>,
    /// Number of bytes to display after starting byte index
    pub len: Option<usize>,
}

/// Create a OctaTrack binary data file with default data (default values not set up yet!)
#[derive(Args, Debug)]
pub struct CreateDefault {
    /// Write path
    pub path: PathBuf,
}


/// Use a human-readable data file to create a new binary data file
#[derive(Args, Debug)]
pub struct HumanToBin {
    /// Read from this type of human-readable format
    #[arg(value_enum)]
    pub source_type: HumanReadableFileFormat,
    /// Path to the human-readable source file
    pub source_path: PathBuf,
    /// Path to the output OctaTrack data file
    pub bin_path: PathBuf,
}

/// Create a human-readable data file from an OctaTrack's binary data file
#[derive(Args, Debug)]
pub struct BinToHuman {
    /// Path to the source OctaTrack data file
    pub bin_path: PathBuf,
    /// Convert to this type of human-readable format
    #[arg(value_enum)]
    pub dest_type: HumanReadableFileFormat,
    /// Path to the human-readable output file
    pub dest_path: PathBuf,
}

/// Commands related to data in OctaTrack Arrangement files (examples: arr01.work, arr01.strd)
#[derive(Subcommand, Debug)]
#[command(flatten_help = true)]
#[command(help_template = "{name}: {about}\n{usage-heading}\n{tab}{tab}{tab} {usage}")]
pub enum Arrangements {
    Inspect(Inspect),
    InspectBytes(InspectBytes),
    CreateDefault(CreateDefault),
    BinToHuman(BinToHuman),
    HumanToBin(HumanToBin),
}

#[derive(Subcommand, Debug)]
pub enum ProjectData {
    Inspect(Inspect),
}

#[derive(Subcommand, Debug)]
pub enum SampleSlots {
    Inspect(Inspect),

    /// List Sample Slots used in a Project.
    List {
        /// File path of the project.work or project.strd file
        path: PathBuf,
    },

    /// Remove Project Sample Slots when the slot is not being used in any related Bank files.
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

/// Commands related to data contained in OctaTrack Project files (examples: project.work, project.strd)
#[derive(Subcommand, Debug)]
#[command(flatten_help = true)]
#[command(help_template = "{name}: {about}\n{usage-heading}\n{tab}{tab}{tab} {usage}")]
pub enum Projects {
    Inspect(Inspect),
    CreateDefault(CreateDefault),

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
    SampleSlots(SampleSlots),

    BinToHuman(BinToHuman),
    HumanToBin(HumanToBin),
}

/// Commands related to data contained in OctaTrack Bank files (examples: bank01.work, bank01.strd)
#[derive(Subcommand, Debug)]
#[command(flatten_help = true)]
#[command(help_template = "{name}: {about}\n{usage-heading}\n{tab}{tab}{tab} {usage}")]
pub enum Banks {
    Inspect(Inspect),
    InspectBytes(InspectBytes),
    CreateDefault(CreateDefault),

    /// Move a Bank from one Project to another Project while updating active sample slot
    /// assignments in the destination Project.
    Copy {
        /// File path of the source `bank??.work` or `bank??.strd` file
        src_bank_path: PathBuf,
        /// File path of the source project.work or project.strd file
        src_project_path: PathBuf,
        /// File path of the destination `bank??.work` or `bank??.strd` file
        dest_bank_path: PathBuf,
        /// File path of the destination `project.work` or `project.strd` file
        dest_project_path: PathBuf,
    },

    /// Move Nx Banks from their source Project to another destination Project while updating active
    /// sample slot assignments in each destination Projects.
    CopyN {
        /// File path of the YAML config detailing the changes to make
        yaml_file_path: PathBuf,
    },

    BinToHuman(BinToHuman),
    HumanToBin(HumanToBin),
}

/// Commands related to Pattern data in OctaTrack Bank files (examples: bank01.work, bank01.strd)
#[derive(Subcommand, Debug)]
#[command(flatten_help = true)]
#[command(help_template = "{name}: {about}\n{usage-heading}\n{tab}{tab}{tab} {usage}")]
pub enum Patterns {
    /// Show the deserialized representation of one or more Patterns
    Inspect {
        bin_path: PathBuf,
        index: Vec<usize>,
    },
}

#[derive(Subcommand, Debug)]
#[command(flatten_help = true)]
#[command(help_template = "{name}: {about}\n{usage-heading}\n{tab}{tab}{tab} {usage}")]
pub enum PartsCmd {
    /// Show the deserialized representation of one or more Parts
    Inspect {
        bin_path: PathBuf,
        index: Vec<usize>,
    },
}

/// Commands related to Part data in OctaTrack Bank files (examples: bank01.work, bank01.strd)
#[derive(Subcommand, Debug)]
#[command(flatten_help = true)]
#[command(help_template = "{name}: {about}\n{usage-heading}\n{tab}{tab}{tab} {usage}")]
pub enum Parts {
    /// Commands related to SAVED Part data in OctaTrack Bank files (examples: bank01.work, bank01.strd)
    #[command(subcommand)]
    Saved(PartsCmd),

    /// Commands related to UNSAVED Part data in OctaTrack Bank files (examples: bank01.work, bank01.strd)
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

/// Commands related to finding compatible audio files to use with an OctaTrack
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

/// Commands related to Sample Attributes data in OctaTrack 'OT' files (examples: sampleName.ot, anotherSampleName.ot)
#[derive(Subcommand, Debug)]
#[command(flatten_help = true)]
#[command(help_template = "{name}: {about}\n{usage-heading}\n{tab}{tab}{tab} {usage}")]
pub enum Otfile {
    Inspect(Inspect),
    InspectBytes(InspectBytes),
    CreateDefault(CreateDefault),

    /// Create Nx OctaTrack binary data files with default data
    CreateDefaultN {
        /// Wav File paths to generate default sample attribute files for
        paths: Vec<PathBuf>,
    },

    BinToHuman(BinToHuman),
    HumanToBin(HumanToBin),
}

/// Commands related to Sample Attributes data in OctaTrack 'OT' files (examples: sampleName.ot, anotherSampleName.ot)
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
    /// Generate an in-depth YAML representation of OctaTrack data for a Set.
    /// WARNING: A 1x Project Set will generate a circa 200MB YAML file!
    Scan {
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
