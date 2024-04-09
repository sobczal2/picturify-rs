use clap::{Arg, Command};

pub struct NoneCommand;

impl NoneCommand {
    pub fn new() -> Command {
        Command::new("none")
            .about("Perform no processing on an image")
    }
}
