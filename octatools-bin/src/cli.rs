//! Module for CLAP based CLI arguments.

use clap::{command, Args, Parser, Subcommand, ValueEnum, ValueHint};
use std::path::PathBuf;

#[doc(hidden)]
#[derive(Parser, Debug, PartialEq)]
#[command(version, long_about = None, about = "CLI tool for handling Elektron Octatrack DPS-1 data files.")]
#[command(propagate_version = false)]
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
#[derive(Args, Debug, PartialEq)]
pub struct Inspect {
    /// Path of the OctaTrack binary data file
    #[arg(value_hint = ValueHint::FilePath)]
    pub bin_path: PathBuf,
}

/// Read a binary data file and print raw u8 byte values to stdout
#[derive(Args, Debug, PartialEq)]
pub struct InspectBytes {
    /// Path of the OctaTrack binary data file
    #[arg(value_hint = ValueHint::FilePath)]
    pub bin_path: PathBuf,
    /// Index of starting byte range to inspect
    #[arg(value_hint = ValueHint::Other)]
    pub start: Option<usize>,
    /// Number of bytes to display after starting byte index
    #[arg(value_hint = ValueHint::Other)]
    pub len: Option<usize>,
}

/// Create a OctaTrack binary data file with default data (default values not set up yet!)
#[derive(Args, Debug, PartialEq)]
pub struct CreateDefault {
    /// Write path
    #[arg(value_hint = ValueHint::FilePath)]
    pub path: PathBuf,
}

/// Use a human-readable data file to create a new binary data file
#[derive(Args, Debug, PartialEq)]
pub struct HumanToBin {
    /// Read from this type of human-readable format
    #[arg(value_enum)]
    pub source_type: HumanReadableFileFormat,
    /// Path to the human-readable source file
    #[arg(value_hint = ValueHint::FilePath)]
    pub source_path: PathBuf,
    /// Path to the output OctaTrack data file
    #[arg(value_hint = ValueHint::FilePath)]
    pub bin_path: PathBuf,
}

/// Create a human-readable data file from an OctaTrack's binary data file
#[derive(Args, Debug, PartialEq)]
pub struct BinToHuman {
    /// Path to the source OctaTrack data file
    #[arg(value_hint = ValueHint::FilePath)]
    pub bin_path: PathBuf,
    /// Convert to this type of human-readable format
    #[arg(value_enum)]
    pub dest_type: HumanReadableFileFormat,
    /// Path to the human-readable output file
    #[arg(value_hint = ValueHint::FilePath)]
    pub dest_path: PathBuf,
}

/// Commands related to data in OctaTrack Arrangement files (examples: arr01.work, arr01.strd)
#[derive(Subcommand, Debug, PartialEq)]
pub enum Arrangements {
    Inspect(Inspect),
    InspectBytes(InspectBytes),
    CreateDefault(CreateDefault),
    BinToHuman(BinToHuman),
    HumanToBin(HumanToBin),
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum ProjectData {
    Inspect(Inspect),
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum SampleSlots {
    Inspect(Inspect),

    /// List Sample Slots used in a Project.
    List {
        /// File path of the project.work or project.strd file
        #[arg(value_hint = ValueHint::FilePath)]
        path: PathBuf,
    },

    /// Remove Project Sample Slots when the slot is not being used in any related Bank files.
    Purge {
        /// Project directory path, NOT the project.work/project.strd file (command needs to inspect related bank files)
        #[arg(value_hint = ValueHint::DirPath)]
        path: PathBuf,
    },

    /// Copy relevant audio files to the Project directory
    Consolidate {
        /// Project directory path, NOT the project.work/project.strd file (command needs to find the related Set Audio Pool)
        #[arg(value_hint = ValueHint::DirPath)]
        path: PathBuf,
    },

    /// Copy relevant audio files to the Project Set's Audio Pool directory
    Centralise {
        /// Project directory path, NOT the project.work/project.strd file (command needs to find the related Set Audio Pool)
        #[arg(value_hint = ValueHint::DirPath)]
        path: PathBuf,
    },

    /// Deduplicate sample slots within a project.
    #[command(long_about = "\
Deduplicate sample slots within a project, removing duplicates based on slot settings.
Slot uniqueness is determined by the file path of the registered sample file, gain, tempo, trim length etc.
The command will also update slot references in all bankXX.work files within the project \
(slot assignments are changed to point at the remaining unique slot).

** WARNING ** Does not check whether sample files are unique based on content -- be careful naming your sample files!
")]
    Deduplicate {
        /// Project directory path to perform de-duplication on
        #[arg(value_hint = ValueHint::DirPath)]
        project_dirpath: PathBuf,
    },
}

/// Commands related to data contained in OctaTrack Project files (examples: project.work, project.strd)
#[derive(Subcommand, Debug, PartialEq)]
pub enum Projects {
    Inspect(Inspect),
    CreateDefault(CreateDefault),

    /// Specific commands for Project Metadata
    #[command(subcommand)]
    Metadata(ProjectData),

    /// Specific commands for Project State
    #[command(subcommand)]
    State(ProjectData),

    /// Specific commands for Project Settings
    #[command(subcommand)]
    Settings(ProjectData),

    /// Specific commands for Project Sample Slots
    #[command(subcommand)]
    SampleSlots(SampleSlots),

    BinToHuman(BinToHuman),
    HumanToBin(HumanToBin),
}

#[derive(Debug, clap::Args, PartialEq, Clone)]
#[group(required = false, multiple = false)]
pub(crate) struct ListSlotUsageOpts {
    /// Don't list usages for sample slots without an audio file loaded,
    /// conflicts with `--exclude-loaded`.
    #[clap(long, action)]
    pub(crate) exclude_empty: bool,
}

/// Commands related to data contained in OctaTrack Bank files (examples: bank01.work, bank01.strd)
#[derive(Subcommand, Debug, PartialEq)]
pub enum Banks {
    Inspect(Inspect),
    InspectBytes(InspectBytes),
    CreateDefault(CreateDefault),

    /// Move a Bank from one Project to another Project while updating active sample slot
    /// assignments in the destination Project.
    Copy {
        /// Directory path of the source project
        #[arg(value_hint = ValueHint::DirPath)]
        src_project_path: PathBuf,
        /// Directory path of the destination project
        #[arg(value_hint = ValueHint::DirPath)]
        dest_project_path: PathBuf,
        /// Number 1-16 (inclusive) of the source bank to copy
        #[arg(value_hint = ValueHint::Other)]
        src_bank_id: usize,
        /// Number 1-16 (inclusive) of the bank location in the destination project
        #[arg(value_hint = ValueHint::DirPath)]
        dest_bank_id: usize,
    },

    /// List sample slot usages within the given bank
    ListSlotUsage {
        /// Directory path of the project
        #[arg(value_hint = ValueHint::DirPath)]
        project_path: PathBuf,
        /// Number 1-16 (inclusive) of the source bank to search for sample slot usages
        #[arg(value_hint = ValueHint::Other)]
        bank_id: usize,
        #[clap(flatten)]
        list_opts: ListSlotUsageOpts,
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
#[derive(Subcommand, Debug, PartialEq)]
pub enum Patterns {
    /// Show the deserialized representation of one or more Patterns
    Inspect {
        #[arg(value_hint = ValueHint::FilePath)]
        bin_path: PathBuf,
        #[arg(value_hint = ValueHint::Other)]
        index: Vec<usize>,
    },

    /// List sample slot usages within the given pattern
    ListSlotUsage {
        /// Directory path of the project
        #[arg(value_hint = ValueHint::DirPath)]
        project_path: PathBuf,
        /// Number 1-16 (inclusive) of the bank
        #[arg(value_hint = ValueHint::Other)]
        bank_id: usize,
        /// Number 1-16 (inclusive) of the pattern
        #[arg(value_hint = ValueHint::Other)]
        pattern_id: usize,
        #[clap(flatten)]
        list_opts: ListSlotUsageOpts,
    },
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum PartsCmd {
    /// Show the deserialized representation of one or more Parts
    Inspect {
        #[arg(value_hint = ValueHint::FilePath)]
        bin_path: PathBuf,
        #[arg(value_hint = ValueHint::Other)]
        index: Vec<usize>,
    },

    /// List sample slot usages within the given part
    ListSlotUsage {
        /// Directory path of the project
        #[arg(value_hint = ValueHint::DirPath)]
        project_path: PathBuf,
        /// Number 1-16 (inclusive) of the bank
        #[arg(value_hint = ValueHint::Other)]
        bank_id: usize,
        /// Number 1-16 (inclusive) of the pattern
        #[arg(value_hint = ValueHint::Other)]
        part_id: usize,
        #[clap(flatten)]
        list_opts: ListSlotUsageOpts,
    },
}

/// Commands related to Part data in OctaTrack Bank files (examples: bank01.work, bank01.strd).
#[derive(Subcommand, Debug, PartialEq)]
pub enum Parts {
    /// Commands related to SAVED Part data in OctaTrack Bank files (examples: bank01.work, bank01.strd)
    #[command(subcommand)]
    Saved(PartsCmd),

    /// Commands related to UNSAVED Part data in OctaTrack Bank files (examples: bank01.work, bank01.strd)
    #[command(subcommand)]
    Unsaved(PartsCmd),
}

/// Create slice grids for existing audio files (no chaining sample files together, just slice grids).
#[derive(Subcommand, Debug, PartialEq)]
pub enum SampleSliceGrid {
    /// Create an `.ot` file with random slice grid from the cli
    Random {
        /// Location of the audio file to generate a random slices for
        #[arg(value_hint = ValueHint::FilePath)]
        wav_file_path: PathBuf,

        /// How many random slices to create
        #[arg(value_hint = ValueHint::Other)]
        n_slices: usize,
    },
    /// Create an `.ot` file with linear slice grid from the cli
    Linear {
        /// Location of the audio file to generate a linear grid for
        #[arg(value_hint = ValueHint::FilePath)]
        wav_file_path: PathBuf,

        /// How many slices to put in the slice grid
        #[arg(value_hint = ValueHint::Other)]
        n_slices: usize,
    },
}

/// Create a 'sliced sample chain' from multiple audio files (chaining audio files together into one audio file with slice grids a la Octachainer).
#[derive(Subcommand, Debug, PartialEq)]
pub enum SampleChains {
    Create {
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
    CreateN {
        /// File path of the YAML file for batched samplechains construction.
        #[arg(value_hint = ValueHint::FilePath)]
        yaml_file_path: PathBuf,
    },
    /// Use the CLI to deconstruct an individual sliced samplechain.
    Deconstruct {
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
    /// Use a YAML config to deconstruct batches of sliced samplechains.
    DeconstructN {
        /// File path of the YAML file.
        #[arg(value_hint = ValueHint::FilePath)]
        yaml_file_path: PathBuf,
    },
}

/// Find OctaTrack compatible audio files on filesystems
#[derive(Subcommand, Debug, PartialEq)]
pub enum SampleSearch {
    /// Creates a YAML file output just listing all compatible files.
    Simple {
        /// Path to the top of the directory tree to search through.
        #[arg(value_hint = ValueHint::DirPath)]
        samples_dir_path: PathBuf,

        /// File path for the output YAML file
        #[arg(value_hint = ValueHint::FilePath)]
        yaml_file_path: Option<PathBuf>,
    },

    /// Creates a YAML file output including useful file metadata.
    Full {
        /// Path to the top of the directory tree to search through.
        #[arg(value_hint = ValueHint::DirPath)]
        samples_dir_path: PathBuf,

        /// File path for the output YAML file
        #[arg(value_hint = ValueHint::FilePath)]
        yaml_file_path: Option<PathBuf>,
    },
}

/// Commands related to OctaTrack '.ot' sample metadata files (examples: sampleName.ot, anotherSampleName.ot)
#[derive(Subcommand, Debug, PartialEq)]
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

/// Commands related to samples (audio files and metadata files for those audio files).
#[derive(Subcommand, Debug, PartialEq)]
pub enum Samples {
    #[command(subcommand)]
    Chain(SampleChains),

    #[command(subcommand)]
    Grid(SampleSliceGrid),

    #[command(subcommand)]
    Otfile(Otfile),

    #[command(subcommand)]
    Search(SampleSearch),
}

/// Commands related to the 'drive' i.e. the whole Compact Flash Card
#[derive(Subcommand, Debug, PartialEq)]
pub enum Drive {
    /// Generate an in-depth YAML representation of OctaTrack data for a Set.
    /// WARNING: A 1x Project Set will generate a circa 200MB YAML file!
    Scan {
        /// Directory path of the Compact Flash Card directory
        #[arg(value_hint = ValueHint::DirPath)]
        cfcard_dir_path: PathBuf,

        /// File path location where the output YAML file will be written
        #[arg(value_hint = ValueHint::FilePath)]
        yaml_file_path: Option<PathBuf>,
    },
}

/// Generates completion files for the specified shell.
#[derive(Subcommand, Debug, PartialEq, Clone)]
pub enum ShellCompletions {
    /// Example usage:
    /// `octatools-bin shell-completion bash > ./octatools.bash && . ./octatools.bash`
    Bash,
    /// Example usage:
    /// `octatools-bin shell-completion powershell > ./octatools.ps && . ./octatools.ps`
    // #[arg(alias = "ps")]
    Powershell,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Commands {
    /// Prints a list of all available commands and a description of what they do
    HelpFull,

    #[command(subcommand)]
    ShellCompletion(ShellCompletions),

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
