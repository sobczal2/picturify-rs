use clap::Command;

pub struct NegativeCommand;

impl NegativeCommand {
    pub fn get() -> Command {
        Command::new("negative").about("Run negative processing pipeline on an fast_image")
    }
}
