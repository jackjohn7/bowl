use clap::Parser;

/// Arguments to be passed when running init command
#[derive(Parser, Debug)]
pub struct InitArgs {
    #[arg()]
    pub template: String,
}

/// create project from boilerplate code provided in
/// the template argument
pub fn handle_init(_command: InitArgs) -> Result<(), String> {
    todo!()
}
