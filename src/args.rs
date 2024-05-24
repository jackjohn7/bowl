use clap::Parser;

use crate::subcommands::{init::InitArgs, new::NewArgs, run::RunArgs, check::CheckArgs};

/// All subcommands available to the user
#[derive(Parser, Debug)]
pub enum Command {
    /// Create a new project from template
    Init(InitArgs),
    /// Execute subcommand from template
    Run(RunArgs),
    /// Create a new bowl template
    New(NewArgs),
    /// Checks that your bowl template is valid
    Check(CheckArgs),
    /// Saves a template locally without publishing it
    Save,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Individual subcommand
    #[command(subcommand)]
    pub command: Command,

    /// Specifies authentication token with soup server
    #[arg(long, global = true)]
    pub token: Option<String>,
}
