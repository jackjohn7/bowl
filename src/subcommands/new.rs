use clap::Parser;

/// Arguments to be passed when running new command
#[derive(Parser, Debug)]
pub struct NewArgs {
    #[arg()]
    pub name: String,

    #[arg(long, action)]
    pub no_git: bool,
}

/// Create an empty bowl project with a bowl.toml file within
/// and a git repository created unless specified otherwise
/// or user doesn't have git.
pub fn handle_new(_command: NewArgs) {
    todo!()
}
