use clap::Parser;

use crate::subcommands::{
    check::CheckArgs, new::NewArgs, publish::PublishArgs, run::RunArgs, use_cmd::UseArgs,
};

/// All subcommands available to the user
#[derive(Parser, Debug)]
pub enum Command {
    /// Create a new project from template
    Use(UseArgs),
    /// Execute subcommand from template
    Run(RunArgs),
    /// Create a new bowl template
    New(NewArgs),
    /// Checks that your bowl template is valid
    Check(CheckArgs),
    /// Checks that your bowl template is valid and publishes it
    Publish(PublishArgs),
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
