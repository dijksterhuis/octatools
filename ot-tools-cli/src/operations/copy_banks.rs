use crate::print_err;
use clap::{Subcommand, ValueHint};
use ot_tools_ops::actions::banks::{batch_copy_banks, copy_bank_by_paths};
use std::path::PathBuf;

/// Copy sections of a project from one location to another, e.g. banks between projects
#[derive(Subcommand, Debug, PartialEq)]
pub(crate) enum SubCmds {
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

#[doc(hidden)]
pub(crate) fn subcmd_runner(x: SubCmds) {
    match x {
        SubCmds::Bank {
            src_project_dirpath,
            dest_project_dirpath,
            src_bank_id,
            dest_bank_id,
            force,
            // TODO
            // _no_reassign_slots,
        } => {
            print_err(|| {
                copy_bank_by_paths(
                    &src_project_dirpath,
                    &dest_project_dirpath,
                    src_bank_id,
                    dest_bank_id,
                    force,
                )
            });
        }
        SubCmds::BankYaml { yaml_file_path } => {
            print_err(|| batch_copy_banks(&yaml_file_path));
        }
    }
}
