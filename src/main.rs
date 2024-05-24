pub mod args;
pub mod subcommands;
pub mod templates;

use args::Cli;
use clap::Parser;
use subcommands::{check::handle_check, init::handle_init, new::handle_new, run::handle_run};

fn main() {
    let args = Cli::parse();
    match args.command {
        args::Command::Init(args) => handle_init(args),
        args::Command::Run(args) => handle_run(args),
        args::Command::New(args) => handle_new(args),
        args::Command::Check(args) => handle_check(args),
        args::Command::Save => todo!(),
    }
}
