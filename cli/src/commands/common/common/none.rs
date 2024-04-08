use clap::{Arg, Command};

pub struct NoneCommand;

impl NoneCommand {
    pub fn new() -> Command {
        Command::new("none")
            .args(vec![
                Arg::new("input").index(1).required(true),
                Arg::new("output").index(2).required(true),
            ])
            .about("Perform no processing on an image")
    }
}
