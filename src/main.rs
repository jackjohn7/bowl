pub mod args;
pub mod files;
pub mod subcommands;
pub mod templates;

use std::process::exit;

use args::Cli;
use clap::Parser;
use subcommands::{
    check::handle_check, init::handle_use, new::handle_new, publish::handle_publish,
    run::handle_run,
};

fn main() {
    let args = Cli::parse();
    if let Err(e) = match args.command {
        args::Command::Use(args) => handle_use(args),
        args::Command::Run(args) => handle_run(args),
        args::Command::New(args) => handle_new(args),
        args::Command::Check(args) => handle_check(args),
        args::Command::Publish(args) => handle_publish(args),
        args::Command::Save => todo!(),
    } {
        println!("{}", e);
        exit(1);
    }
}
