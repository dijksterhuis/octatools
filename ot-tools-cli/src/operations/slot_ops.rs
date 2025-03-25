use crate::print_err;
use clap::{Subcommand, ValueHint};
use ot_tools_ops::actions::projects::purge_project_pool;
use ot_tools_ops::actions::projects::slots::cmd_slots_deduplicate;
use std::path::PathBuf;

/// Modifying sample slots within an existing project
#[derive(Subcommand, Debug, PartialEq)]
pub(crate) enum SubCmds {
    /// Deduplicate project sample slots based on slot settings (does not remove files, does update bank data)
    Deduplicate {
        /// Project directory path to perform de-duplication on
        #[arg(value_hint = ValueHint::DirPath)]
        project_dirpath: PathBuf,
    },

    /// Delete project sample slots when not used by any project banks
    Purge {
        /// Project directory path
        #[arg(value_hint = ValueHint::DirPath)]
        project_dirpath: PathBuf,
    },
}

#[doc(hidden)]
#[allow(dead_code)] // coming back to it later
pub(crate) fn subcmd_runner(x: SubCmds) {
    match x {
        SubCmds::Purge { project_dirpath } => {
            print_err(|| purge_project_pool(&project_dirpath));
        }
        SubCmds::Deduplicate { project_dirpath } => {
            print_err(|| cmd_slots_deduplicate(&project_dirpath));
        }
    }
}
