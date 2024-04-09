use clap::Command;

pub struct NegativeCommand;

impl NegativeCommand {
    pub fn new() -> Command {
        Command::new("negative").about("Run negative processing pipeline on an image")
    }
}
