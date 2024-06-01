use clap::Parser;

/// Arguments to be passed when running init command
#[derive(Parser, Debug)]
pub struct UseArgs {
    #[arg()]
    pub template: String,

    /// Specify the path of where the bowlfile should be sourced
    #[arg(long)]
    pub path: Option<String>,
}

/// create project from boilerplate code provided in
/// the template argument
pub fn handle_use(_cmd: UseArgs) -> Result<(), String> {
    todo!()
}
