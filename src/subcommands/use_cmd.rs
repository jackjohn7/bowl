use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use clap::Parser;

use crate::{files::get_file_locally, templates::bowlfile::BowlFile};

/// Arguments to be passed when running init command
#[derive(Parser, Debug)]
pub struct UseArgs {
    #[arg()]
    pub template: String,

    /// Specify the path of where the bowlfile should be sourced
    #[arg(long)]
    pub path: Option<String>,

    /// Specify that the bowlfile should be found in the local store
    #[arg(long, action)]
    pub local: bool,
}

/// create project from boilerplate code provided in
/// the template argument
pub fn handle_use(cmd: UseArgs) -> Result<(), String> {
    let raw = if cmd.local {
        get_file_locally(cmd.template)?
    } else {
        todo!("Soup registry not built yet")
    };

    let bf = BowlFile::decode(raw)?;

    let config = bf.get_config()?;

    for file in bf
        .files
        .iter()
        .filter(|f| f.file_path != config.options.readme)
        .filter(|f| f.file_path != "./bowl.toml")
    {
        let p = Path::new(&file.file_path);
        if let Some(parent) = p.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("Failed to make directory: {}", e))?;
        }
        File::create(p)
            .map_err(|e| format!("Error creating file: {}", e))?
            .write_all(&file.content)
            .map_err(|e| format!("Failed to write file: {}", e))?;
    }

    Ok(())
}
