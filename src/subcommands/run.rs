use clap::Parser;

/// Arguments to be passed when running run command
#[derive(Parser, Debug)]
pub struct RunArgs {
    #[arg()]
    pub template: String,
    #[arg(value_delimiter=' ', num_args=1..)]
    pub cmd: Vec<String>,
}

/// Run the user's specified command provided by the template
pub fn handle_run(command: RunArgs) {
    println!("{} {}", command.cmd[0], command.cmd[1]);
    todo!()
}
