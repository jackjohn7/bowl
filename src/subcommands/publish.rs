use bowl_core::config::Config;
use clap::Parser;
use std::{
    fs::{self, DirEntry, File},
    io::Write,
    path::{Path, PathBuf},
};

use crate::{
    files::{file_entries, save_file_locally},
    templates::{bowlfile::BowlFile, files::FileContent},
};

/// Arguments to be passed when running publish command
#[derive(Parser, Debug)]
pub struct PublishArgs {
    /// Where the bowlfile will be placed after it's built
    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(long, action)]
    pub local: bool,
}

pub fn handle_publish(cmd: PublishArgs) -> Result<(), String> {
    // check for bowl.toml file
    let contents = match fs::read_to_string("bowl.toml") {
        Ok(contents) => contents,
        Err(_) => {
            return Err("Couldn't find bowl.toml".into());
        }
    };

    let config: Config =
        toml::from_str(&contents).map_err(|e| format!("Error parsing bowl.toml: {}", e))?;

    if !Path::new(&config.options.readme).exists() {
        return Err(format!(
            "Error: ReadMe file \"{}\" not found\nThe path of this \
                    readme file can be set with the \"readme\" option in bowl.toml",
            config.options.readme
        ));
    }

    let ignore = match &config.options.ignore {
        Some(ignore) => ignore.iter().map(PathBuf::from).collect::<Vec<PathBuf>>(),
        None => Vec::new(),
    };

    let files = fs::read_dir(".")
        .map_err(|e| e.to_string())?
        .map(|x| x.map_err(|e| e.to_string()))
        .collect::<Result<Vec<DirEntry>, String>>()?
        .into_iter()
        .map(file_entries)
        .collect::<Result<Vec<Vec<DirEntry>>, String>>()?
        .into_iter()
        .flatten()
        .map(|x| x.path())
        .filter(|x| !ignore.contains(x))
        .map(FileContent::from_path)
        .collect::<Result<Vec<FileContent>, String>>()?;

    let bf = BowlFile::new(files);

    let bytes = bf.encode();

    if let Some(out) = cmd.output {
        let p = PathBuf::from(&out);
        if let Some(parent) = p.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("Failed to make directory: {}", e))?;
        }
        // is director
        // if user provided a directory, save it as <name>.bowl in that dir
        // if the user provides a full filepath, save it at that path
        let mut file = File::create(out).map_err(|e| format!("Error creating bowlfile: {}", e))?;

        let _ = file
            .write_all(&bytes)
            .map_err(|e| format!("Failed to write bowlfile: {}", e));
    } else if cmd.local {
        save_file_locally(format!("{}.bowl", config.template.name), bytes)?;
    } else {
        // TODO: should be pushed to soup registry (local install only for now)
        todo!("Soup server not built yet")
    }

    println!("Bowl template built successfully!");

    Ok(())
}
