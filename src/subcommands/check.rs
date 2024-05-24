use std::fs;

use clap::Parser;

use crate::templates::config::Config;

/// Arguments provided to the check command
#[derive(Parser, Debug)]
pub struct CheckArgs {}

/// Checks that a user's configuration is valid
pub fn handle_check(_command: CheckArgs) {
    // check for bowl.toml file
    let contents = fs::read_to_string("bowl.toml").expect("Should have been able to read the file");

    let config: Config = match toml::from_str(&contents) {
        Ok(content) => content,
        Err(err) => {
            println!("Error parsing bowl.toml: {}", err.to_string());
            panic!()
        }
    };

    dbg!(config);

    println!("Check succeeded!");
}
