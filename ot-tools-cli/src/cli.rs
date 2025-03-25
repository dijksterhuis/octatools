//! Module for CLAP based CLI arguments.

use clap::{command, Parser, Subcommand, ValueEnum, ValueHint};
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

#[derive(Debug, PartialEq, Clone, ValueEnum)]
// #[group(required = false, multiple = false)]
pub enum BinTypes {
    /// Binary data file is a `project.work` or `project.strd`
    Project,
    /// Binary data file is a `bank??.work` or `bank??.strd`
    Bank,
    /// Binary data file is an `arr??.work` or `arr??.strd`
    Arrangement,
    /// Binary data file is an `*.ot` file
    SampleAttributes,
}

/// Commands for working with individual binary data files directly.
#[derive(Subcommand, Debug, PartialEq)]
pub enum BinFiles {
    /// Read a binary data file and print the deserialized output to stdout
    Inspect {
        /// Type of binary data file
        #[arg(value_enum)]
        bin_type: BinTypes,
        /// Path of the binary data file
        #[arg(value_hint = ValueHint::FilePath)]
        bin_path: PathBuf,
    },
    /// Read a binary data file and print raw u8 byte values to stdout
    InspectBytes {
        /// Type of binary data file
        #[arg(value_enum)]
        bin_type: BinTypes,
        /// Path of the OctaTrack binary data file
        #[arg(value_hint = ValueHint::FilePath)]
        bin_path: PathBuf,
        /// Index of starting byte range to inspect
        #[arg(value_hint = ValueHint::Other)]
        start: Option<usize>,
        /// Number of bytes to display after starting byte index
        #[arg(value_hint = ValueHint::Other)]
        len: Option<usize>,
    },
    /// Create a binary data file with default data
    CreateDefault {
        /// Type of binary data file
        #[arg(value_enum)]
        bin_type: BinTypes,
        /// Path of where to write the new binary data file to
        #[arg(value_hint = ValueHint::FilePath)]
        bin_path: PathBuf,
    },
    /// Create a human-readable data file from a binary data file
    BinToHuman {
        /// Type of binary data file
        #[arg(value_enum)]
        bin_type: BinTypes,
        /// Path to the source binary data file
        #[arg(value_hint = ValueHint::FilePath)]
        bin_path: PathBuf,
        /// Convert to this type of human-readable format
        #[arg(value_enum)]
        dest_type: HumanReadableFileFormat,
        /// Path to the human-readable output file
        #[arg(value_hint = ValueHint::FilePath)]
        dest_path: PathBuf,
    },
    /// Create a binary data file from a human-readable data file
    HumanToBin {
        /// Read from this type of human-readable format
        #[arg(value_enum)]
        source_type: HumanReadableFileFormat,
        /// Path to the human-readable source file
        #[arg(value_hint = ValueHint::FilePath)]
        source_path: PathBuf,
        /// Type of binary data file
        #[arg(value_enum)]
        bin_type: BinTypes,
        /// Path to the output OctaTrack data file
        #[arg(value_hint = ValueHint::FilePath)]
        bin_path: PathBuf,
    },
}

/// Copy sections of a project from one location to another, e.g. banks between projects
#[derive(Subcommand, Debug, PartialEq)]
pub enum Copying {
    /// Copy a bank between projects via the CLI
    /// (updates active sample slot assignments if required)
    Bank {
        /// Directory path of the source project
        #[arg(value_hint = ValueHint::DirPath)]
        src_project_dirpath: PathBuf,
        /// Number 1-16 (inclusive) of the source bank
        #[arg(value_hint = ValueHint::Other)]
        src_bank_id: usize,
        /// Directory path of the destination project
        #[arg(value_hint = ValueHint::DirPath)]
        dest_project_dirpath: PathBuf,
        /// Number 1-16 (inclusive) of the destination bank
        #[arg(value_hint = ValueHint::DirPath)]
        dest_bank_id: usize,
        /// Force overwrite previously modified destination banks (default behaviour is to exit)
        #[clap(short = 'f', long, action)]
        force: bool,
        // // TODO
        // /// Do not reassign sample slots in destination project (not in use currently!)
        // #[clap(long, action)]
        // _no_reassign_slots: bool,
    },

    /// Copy bank(s) between projects via YAML config
    /// (updates active sample slot assignments if required)
    BankYaml {
        /// File path of the YAML config detailing the changes to make
        yaml_file_path: PathBuf,
    },
    /*
    TODO!

    /// Copy a part between banks/projects via the CLI
    /// (updates active sample slot assignments if required)
    Part {
        /// Directory path of the source project
        #[arg(value_hint = ValueHint::DirPath)]
        src_project_dirpath: PathBuf,
        /// Number 1-16 (inclusive) of the source bank
        #[arg(value_hint = ValueHint::Other)]
        src_bank_id: usize,
        /// Number 1-4 (inclusive) of the source part
        #[arg(value_hint = ValueHint::Other)]
        src_part_id: usize,
        /// State of the source part to copy
        #[arg(value_hint = ValueHint::Other)]
        src_part_state: PartStateOpts,
        /// Directory path of the destination project
        #[arg(value_hint = ValueHint::DirPath)]
        dest_project_dirpath: PathBuf,
        /// Number 1-16 (inclusive) of the destination bank
        #[arg(value_hint = ValueHint::DirPath)]
        dest_bank_id: usize,
        /// Number 1-4 (inclusive) of the destination part
        #[arg(value_hint = ValueHint::Other)]
        dest_part_id: usize,
        /// State of the destination part to copy to
        #[arg(value_hint = ValueHint::Other)]
        dest_part_state: PartStateOpts,
        /// Force overwrite previously modified destination banks (default behaviour is to exit)
        #[clap(short = 'f', long, action)]
        force: bool,
    },

    /// Copy part(s) between banks/projects via YAML config
    /// (updates active sample slot assignments if required)
    PartYaml {
        /// File path of the YAML config detailing the changes to make
        yaml_file_path: PathBuf,
    },
    /// Copy a pattern between banks/projects via the CLI
    /// (updates active sample slot assignments if required)
    Pattern {
        /// Directory path of the source project
        #[arg(value_hint = ValueHint::DirPath)]
        src_project_dirpath: PathBuf,
        /// Number 1-16 (inclusive) of the source bank
        #[arg(value_hint = ValueHint::Other)]
        src_bank_id: usize,
        /// Number 1-4 (inclusive) of the source part
        #[arg(value_hint = ValueHint::Other)]
        src_pattern_id: usize,
        /// Directory path of the destination project
        #[arg(value_hint = ValueHint::DirPath)]
        dest_project_dirpath: PathBuf,
        /// Number 1-16 (inclusive) of the destination bank
        #[arg(value_hint = ValueHint::DirPath)]
        dest_bank_id: usize,
        /// Number 1-4 (inclusive) of the destination part
        #[arg(value_hint = ValueHint::Other)]
        dest_pattern_id: usize,
        /// Force overwrite previously modified destination banks (default behaviour is to exit)
        #[clap(short = 'f', long, action)]
        force: bool,
    },

    /// Copy patterns(s) between banks/projects via YAML config
    /// (updates active sample slot assignments if required)
    PatternYaml {
        /// File path of the YAML config detailing the changes to make
        yaml_file_path: PathBuf,
    },
    */
}

#[derive(Debug, clap::Args, PartialEq, Clone)]
#[group(required = false, multiple = false)]
pub(crate) struct ListSlotUsageOpts {
    /// Don't list usages for sample slots without an audio file loaded
    #[clap(long, action)]
    pub(crate) exclude_empty: bool,
}
#[derive(Debug, PartialEq, Clone, ValueEnum)]
pub enum ListSlotsTypes {
    Project,
    Bank,
    Part,
    Pattern,
}

#[derive(Debug, PartialEq, Clone, ValueEnum)]
pub enum PartStateOpts {
    Saved,
    Unsaved,
}

/// List sample slots within sections of an existing project
#[derive(Subcommand, Debug, PartialEq)]
pub enum ListSampleSlotUsage {
    /// List sample slots assigned in a project
    Project {
        /// Directory path of the project
        #[arg(value_hint = ValueHint::DirPath)]
        project_dirpath: PathBuf,
    },
    /// List sample slots assigned in a specific bank of a project
    Bank {
        /// Directory path of the project
        #[arg(value_hint = ValueHint::DirPath)]
        project_dirpath: PathBuf,
        /// Number 1-16 (inclusive) of the source bank
        #[arg(value_hint = ValueHint::Other)]
        bank_id: usize,
        #[clap(flatten)]
        list_opts: ListSlotUsageOpts,
    },
    /// List sample slots assigned in a specific part of a bank (of a project)
    Part {
        /// Directory path of the project
        #[arg(value_hint = ValueHint::DirPath)]
        project_dirpath: PathBuf,
        /// Number 1-16 (inclusive) of the source bank
        #[arg(value_hint = ValueHint::Other)]
        bank_id: usize,
        /// Number 1-4 (inclusive) of the pattern
        #[arg(value_hint = ValueHint::Other)]
        part_id: usize,
        /// Whether to list slots for saved or unsaved part state
        #[arg(value_hint = ValueHint::Other)]
        part_state: PartStateOpts,
        #[clap(flatten)]
        list_opts: ListSlotUsageOpts,
    },
    /// List sample slots used within a specific pattern of a bank (of a project)
    Pattern {
        /// Directory path of the project
        #[arg(value_hint = ValueHint::DirPath)]
        project_dirpath: PathBuf,
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

/// Modifying sample slots within an existing project
#[derive(Subcommand, Debug, PartialEq)]
pub enum ProjectSamples {
    /// Remove sample slots if a slot is not being in any project banks
    Purge {
        /// Project directory path
        #[arg(value_hint = ValueHint::DirPath)]
        project_dirpath: PathBuf,
    },

    /// Copy all sample files to the project directory and change file path of the slot
    Consolidate {
        /// Project directory path
        #[arg(value_hint = ValueHint::DirPath)]
        project_dirpath: PathBuf,
    },

    /// Copy all sample files to the set's audio pool directory and change file path of the slot
    Centralise {
        /// Project directory path
        #[arg(value_hint = ValueHint::DirPath)]
        project_dirpath: PathBuf,
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

/// Create sample chains, slice grids and other utilities for audio sample files
#[derive(Subcommand, Debug, PartialEq)]
pub enum SampleFiles {
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

/// Generates completion files for the specified shell.
#[derive(Subcommand, Debug, PartialEq, Clone)]
pub enum ShellCompletions {
    /// Example usage:
    /// `ot-tools-cli shell-completion bash > ./ot-tools.bash && . ./ot-tools.bash`
    Bash,
    /// Example usage:
    /// `ot-tools-cli shell-completion powershell > ./ot-tools.ps && . ./ot-tools.ps`
    // #[arg(alias = "ps")]
    Powershell,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Commands {
    #[command(subcommand, visible_aliases = &["bin"])]
    BinFiles(BinFiles),

    #[command(subcommand, visible_aliases = &["copy", "cp"])]
    Copying(Copying),

    #[command(subcommand, visible_aliases = &["list", "ls"])]
    ListSlots(ListSampleSlotUsage),

    // TODO: disabled until i work out content hashing stuff
    // #[command(subcommand, visible_aliases = &["project", "slots"])]
    // ProjectSamples(ProjectSamples),
    #[command(subcommand, visible_aliases = &["samples"])]
    SampleFiles(SampleFiles),

    #[command(subcommand, visible_aliases = &["shell"])]
    ShellCompletion(ShellCompletions),

    /// Prints a list of all available commands and a description of what they do
    HelpFull,
}
