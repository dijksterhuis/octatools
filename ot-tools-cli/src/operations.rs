use clap::Subcommand;

mod copy_banks;
mod list_slots;
mod sample_ops;
mod slot_ops;

/// Operations that can be run on Octatrack Sets or Project directories
#[derive(Subcommand, Debug, PartialEq)]
pub(crate) enum SubCmds {
    #[command(subcommand, visible_aliases = &["cp"])]
    Copy(copy_banks::SubCmds),
    #[command(subcommand, visible_aliases = &["list", "ls", "ll"])]
    ListSlots(list_slots::SubCmds),
    // ========================================
    // TODO: Needs testing
    // #[command(subcommand, visible_aliases = &["slots"])]
    // SlotOps(slot_ops::SubCmds),
    // TODO: Needs testing
    // #[command(subcommand, visible_aliases = &["samples"])]
    // SampleOps(sample_ops::SubCmds),
    // ========================================
    // TODO: List entities (bank/part/pattern/arrangement) that have been modified
    // #[command(subcommand, visible_aliases = &["list", "ls", "ll"])]
    // ListNonDefault(list_slots::SubCmds),
    // TODO: Copy a project from one set to another, copying only the sample files required
    // #[command(subcommand)]
    // CopyProjects(list_slots::SubCmds),
}

#[doc(hidden)]
pub(crate) fn subcmd_runner(x: SubCmds) {
    match x {
        SubCmds::Copy(x) => copy_banks::subcmd_runner(x),
        SubCmds::ListSlots(x) => list_slots::subcmd_runner(x),
        // SubCmds::SlotOps(x) => slot_ops::subcmd_runner(x),
        // SubCmds::SampleOps(x) => sample_ops::subcmd_runner(x),
    }
}
