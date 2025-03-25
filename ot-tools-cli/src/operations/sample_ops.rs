use crate::print_err;
use clap::{Subcommand, ValueHint};
use ot_tools_ops::actions::projects::{
    consolidate_sample_slots_to_audio_pool, consolidate_sample_slots_to_project_pool,
};
use std::path::PathBuf;

/// Make changes to sample files used by a project, or contained in the project directory
#[derive(Subcommand, Debug, PartialEq)]
pub(crate) enum SubCmds {
    /// Transfer sample files used by a project to the project directory (will update project sample slots)
    Consolidate {
        /// Project directory path
        #[arg(value_hint = ValueHint::DirPath)]
        project_dirpath: PathBuf,
    },

    /// Transfer sample files used by a project to the set's audio pool directory (will update project sample slots)
    Centralise {
        /// Project directory path
        #[arg(value_hint = ValueHint::DirPath)]
        project_dirpath: PathBuf,
    },

    /// TODO: Remove any sample files in the project directory that are unused by the project
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
        SubCmds::Consolidate { project_dirpath } => {
            print_err(|| consolidate_sample_slots_to_project_pool(&project_dirpath));
        }
        SubCmds::Centralise { project_dirpath } => {
            print_err(|| consolidate_sample_slots_to_audio_pool(&project_dirpath));
        }
        SubCmds::Purge { project_dirpath: _ } => {
            unimplemented!()
        }
    }
}
