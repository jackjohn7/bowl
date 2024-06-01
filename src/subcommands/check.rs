use std::{
    fs::{self, DirEntry},
    path::Path,
};

use clap::Parser;

use crate::{
    files::file_entries,
    templates::{bowlfile::BowlFile, config::Config, files::FileContent},
};

/// Arguments provided to the check command
#[derive(Parser, Debug)]
pub struct CheckArgs {}

/// Checks that a user's configuration is valid
pub fn handle_check(_command: CheckArgs) -> Result<(), String> {
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
        .filter(|x| match config.options.ignore.clone() {
            Some(ignore) => !ignore.contains(&x.to_str().unwrap().to_owned()),
            None => true,
        })
        .map(FileContent::from_path)
        .collect::<Result<Vec<FileContent>, String>>()?;

    let bf = BowlFile::new(dbg!(files));

    let _ = bf.encode();

    dbg!(config);

    println!("Check succeeded!");
    Ok(())
}
