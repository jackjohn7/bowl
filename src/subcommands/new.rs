use std::{fs::File, io::prelude::*, process::Command};

use clap::Parser;
use inquire::Text;

use bowl_core::config::Config;

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
pub fn handle_new(cmd: NewArgs) -> Result<(), String> {
    if !cmd.no_git && Command::new("git").arg("init").output().is_err() {
        return Err(
            "Could not initialize git repository. HINT: use --no-git flag to disable".into(),
        );
    }

    // ask user for project name
    let project_name = match cmd.name {
        Some(name) => name,
        None => Text::new("Name of your template?").prompt().unwrap(),
    };

    // create bowl.toml
    let mut f =
        File::create("bowl.toml").map_err(|e| format!("Failed to create bowl.toml: {}", e))?;
    let serialized = toml::to_string(&Config::new_default(project_name.clone()))
        .map_err(|e| format!("Failed to serialize bowl.toml file: {}", e))?;

    f.write(serialized.as_bytes())
        .map_err(|e| format!("Failed to write bowl.toml file: {}", e))?;

    // prompt user for information about their project
    let md_file = Text::new("Name of your markdown file?")
        .with_default("bowl.md")
        .with_placeholder("bowl.md")
        .prompt()
        .map_err(|e| e.to_string())?;

    // create md file
    let mut md = File::create(md_file).map_err(|e| e.to_string())?;
    md.write(
        MD_TEMPLATE
            .replace("{{BOWL_NAME}}", &project_name)
            .as_bytes(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
