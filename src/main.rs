pub mod args;
pub mod files;
pub mod subcommands;
pub mod templates;

use std::process::exit;

use args::Cli;
use clap::Parser;
use subcommands::{check::handle_check, init::handle_init, new::handle_new, run::handle_run};

fn main() {
    let args = Cli::parse();
    if let Err(e) = match args.command {
        args::Command::Init(args) => handle_init(args),
        args::Command::Run(args) => handle_run(args),
        args::Command::New(args) => handle_new(args),
        args::Command::Check(args) => handle_check(args),
        args::Command::Save => todo!(),
    } {
        println!("{}", e.to_string());
        exit(1);
    }
}
