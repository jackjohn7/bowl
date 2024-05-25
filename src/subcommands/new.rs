use std::{
    fs::File,
    io::prelude::*,
    process::{exit, Command},
};

use clap::Parser;
use inquire::Text;

const MD_TEMPLATE: &str = include_str!("../../templates/bowl.md");

/// Arguments to be passed when running new command
#[derive(Parser, Debug)]
pub struct NewArgs {
    /// The name of the template
    #[arg()]
    pub name: Option<String>,

    /// A user doesn't want to create a git repository.
    /// By default, this is enabled
    #[arg(long, action)]
    pub no_git: bool,
}

/// Create an empty bowl project with a bowl.toml file within
/// and a git repository created unless specified otherwise
/// or user doesn't have git.
pub fn handle_new(cmd: NewArgs) {
    if !cmd.no_git {
        match Command::new("git").arg("init").output() {
            Ok(_) => (),
            Err(_) => {
                println!("Could not initialize git repository");
                exit(1)
            }
        }
    }

    // ask user for project name
    let project_name = match cmd.name {
        Some(name) => name,
        None => Text::new("Name of your template?").prompt().unwrap(),
    };

    // create bowl.toml
    let _f = File::create("bowl.toml");

    // prompt user for information about their project
    let md_file = Text::new("Name of your markdown file?")
        .with_default("bowl.md")
        .with_placeholder("bowl.md")
        .prompt()
        .unwrap();

    // create md file
    let mut md = File::create(md_file).unwrap();
    md.write(
        MD_TEMPLATE
            .replace("{{BOWL_NAME}}", &project_name)
            .as_bytes(),
    )
    .unwrap();
}
