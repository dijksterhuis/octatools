use crate::print_err;
use clap::{Subcommand, ValueEnum, ValueHint};
use ot_tools_ops::actions::banks::list_bank_sample_slot_references;
use ot_tools_ops::actions::parts::{
    list_saved_part_sample_slot_references, list_unsaved_part_sample_slot_references,
};
use ot_tools_ops::actions::patterns::list_pattern_sample_slot_references;
use ot_tools_ops::actions::projects::list_project_sample_slots;
use std::path::PathBuf;

#[derive(Debug, clap::Args, PartialEq, Clone)]
#[group(required = false, multiple = false)]
pub(crate) struct ListSlotUsageOpts {
    /// Don't list usages for sample slots without an audio file loaded
    #[clap(long, action)]
    exclude_empty: bool,
}
#[derive(Debug, PartialEq, Clone, ValueEnum)]
pub(crate) enum ListSlotsTypes {
    Project,
    Bank,
    Part,
    Pattern,
}

#[derive(Debug, PartialEq, Clone, ValueEnum)]
pub(crate) enum PartStateOpts {
    Saved,
    Unsaved,
}

/// List sample slots within sections of an existing project
#[derive(Subcommand, Debug, PartialEq)]
pub(crate) enum SubCmds {
    // TODO
    // /// List sample slots assigned within all projects of this set
    // Set {
    //     /// Directory path of the project
    //     #[arg(value_hint = ValueHint::DirPath)]
    //     set_dirpath: PathBuf,
    // },
    /// List sample slots assigned in a specific project
    Project {
        /// Directory path of the project
        #[arg(value_hint = ValueHint::DirPath)]
        project_dirpath: PathBuf,
    },
    /// List sample slots assigned in a specific bank of a specific project
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
    /// List sample slots assigned in a specific part of a bank (of a specific project)
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
    /// List sample slots used within a specific pattern of a bank (of a specific project)
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

#[doc(hidden)]
pub(crate) fn subcmd_runner(x: SubCmds) {
    match x {
        SubCmds::Project { project_dirpath } => {
            print_err(|| list_project_sample_slots(&project_dirpath));
        }
        SubCmds::Bank {
            project_dirpath,
            bank_id,
            list_opts,
        } => {
            print_err(|| {
                let ListSlotUsageOpts { exclude_empty } = list_opts;
                list_bank_sample_slot_references(&project_dirpath, bank_id, exclude_empty)
            });
        }
        SubCmds::Part {
            project_dirpath,
            bank_id,
            part_id,
            part_state,
            list_opts,
        } => {
            match part_state {
                PartStateOpts::Saved => {
                    print_err(|| {
                        let ListSlotUsageOpts { exclude_empty } = list_opts;
                        list_saved_part_sample_slot_references(
                            &project_dirpath,
                            bank_id,
                            part_id,
                            exclude_empty,
                        )
                    });
                }
                PartStateOpts::Unsaved => {
                    print_err(|| {
                        let ListSlotUsageOpts { exclude_empty } = list_opts;
                        list_unsaved_part_sample_slot_references(
                            &project_dirpath,
                            bank_id,
                            part_id,
                            exclude_empty,
                        )
                    });
                }
            };
        }
        SubCmds::Pattern {
            project_dirpath,
            bank_id,
            pattern_id,
            list_opts,
        } => {
            print_err(|| {
                let ListSlotUsageOpts { exclude_empty } = list_opts;
                list_pattern_sample_slot_references(
                    &project_dirpath,
                    bank_id,
                    pattern_id,
                    exclude_empty,
                )
            });
        }
    }
}
