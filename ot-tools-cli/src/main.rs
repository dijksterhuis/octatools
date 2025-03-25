//! # `ot-tools-cli`
//!
//! CLI tool to interact with data files used by the [Elektron OctaTrack DPS](https://www.elektron.se/en/octratrack-mkii-explorer)
#[doc(hidden)]
mod bin_files;
#[doc(hidden)]
mod operations;
#[doc(hidden)]
mod sample_files;

use clap::{command, Command, CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Generator, Shell};
use env_logger::{Builder, Target};
use log::LevelFilter;
use std::error::Error;
use std::fmt::Display;
use std::io;
use std::io::Write;

#[doc(hidden)]
pub type RBoxErr<T> = Result<T, Box<dyn Error>>;
#[doc(hidden)]
pub type RVoidError<T> = Result<T, ()>;

#[doc(hidden)]
#[derive(Parser, Debug, PartialEq)]
#[command(version, long_about = None, about = "CLI tool for handling Elektron Octatrack DPS-1 data files.")]
#[command(propagate_version = false)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Generates completion files for the specified shell.
#[doc(hidden)]
#[derive(Subcommand, Debug, PartialEq, Clone)]
enum SubCmds {
    /// Example usage:
    /// `ot-tools-cli shell-completion bash > ./ot-tools.bash && . ./ot-tools.bash`
    Bash,
    /// Example usage:
    /// `ot-tools-cli shell-completion powershell > ./ot-tools.ps && . ./ot-tools.ps`
    // #[arg(alias = "ps")]
    Powershell,
}

#[doc(hidden)]
#[derive(Subcommand, Debug, PartialEq)]
enum Commands {
    #[command(subcommand, visible_aliases = &["bin", "b"])]
    BinFiles(bin_files::SubCmds),

    #[command(subcommand, visible_aliases = &["ops"])]
    Operations(operations::SubCmds),

    #[command(subcommand, visible_aliases = &["samples", "s"])]
    SampleFiles(sample_files::SubCmds),

    #[command(subcommand, visible_aliases = &["shell", "sh"])]
    ShellCompletion(SubCmds),

    /// Prints a list of all available commands and a description of what they do
    HelpFull,
}

#[doc(hidden)]
pub fn print_err<E, F>(cb: F)
where
    F: FnOnce() -> Result<(), E>,
    E: Display,
{
    let r = cb();
    if r.is_err() {
        println!("ERROR: {}", r.unwrap_err());
    }
}

#[doc(hidden)]
fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

#[doc(hidden)]
fn cmd_shell_completions(x: SubCmds) {
    let mut cli_data = Cli::command();
    match x {
        SubCmds::Bash => print_completions(Shell::Bash, &mut cli_data),
        SubCmds::Powershell => print_completions(Shell::PowerShell, &mut cli_data),
    }
}

#[doc(hidden)]
fn cmd_help_full() {
    let mut cli_data = Cli::command();

    let mut buf = String::new();
    let mut prefix = String::new();

    /*

    SAMPLES: Some text describing `samples` commands
    ====================================
    samples chain create: some text about chaining
    samples chain create-n: some text about chaining
    sample grid linear: some text about slice grids
    sample grid random: some text about slice grids
    */
    let _ = recursive_walk_subcommands(&mut buf, &mut prefix, &mut cli_data);

    io::stdout().write_all(buf.as_bytes()).unwrap();
    io::stdout().flush().unwrap();
}

#[doc(hidden)]
fn write_command_usage(buffer: &mut String, prefix: &mut String, cmd: &mut Command) {
    /*
    {prefix} command -- Some text describing a specific command
    {prefix} command -- Some text describing a specific command
    {prefix} command -- Some text describing a specific command
    {prefix} command -- Some text describing a specific command
    */

    buffer.push_str(format!("{prefix} {}", cmd.get_name()).as_str());
    if let Some(about) = cmd.get_about() {
        buffer.push_str(format!(" -- {}", about).as_str());
    }
    buffer.push('\n');
}

#[doc(hidden)]
fn write_top_level_header(buffer: &mut String, cmd: &mut Command) {
    /*

    SAMPLES: Some text describing `samples` commands
    ====================================
    */
    buffer.push_str(format!("\n{}", cmd.get_name().to_ascii_uppercase()).as_str());
    if let Some(about) = cmd.get_about() {
        buffer.push_str(format!(": {}\n", about).as_str());
    }
    buffer.push_str("====================================\n");
}

#[doc(hidden)]
fn recursive_walk_subcommands(
    buffer: &mut String,
    prefix: &mut String,
    cmd: &mut Command,
) -> String {
    for sub in cmd.get_subcommands_mut() {
        // some sort of command/subcommand
        if sub.has_subcommands() {
            let mut sub_prefix = prefix.clone();
            if sub_prefix.is_empty() {
                // no existing prefix -- top level command, create a header block
                write_top_level_header(buffer, sub)
            } else {
                // an existing prefix -- is a subcommand so include in list with usage
                sub_prefix.push(' ');
            }
            sub_prefix.push_str(sub.get_name());
            recursive_walk_subcommands(buffer, &mut sub_prefix, sub);
        } else {
            // no subcommands, write usage details
            write_command_usage(buffer, prefix, sub);
        }
    }

    buffer.clone()
}
#[doc(hidden)]
fn main() {
    let mut logger = Builder::new();
    logger.filter_level(LevelFilter::Debug);
    logger.target(Target::Stdout).init();

    match Cli::parse().command {
        Commands::BinFiles(x) => bin_files::subcmd_runner(x),
        Commands::Operations(x) => operations::subcmd_runner(x),
        Commands::SampleFiles(x) => sample_files::subcmd_runner(x),
        Commands::ShellCompletion(x) => cmd_shell_completions(x),
        Commands::HelpFull => cmd_help_full(),
    };
}
