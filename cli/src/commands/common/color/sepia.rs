use clap::Command;

pub struct SepiaCommand;

impl SepiaCommand {
    pub fn new() -> Command {
        Command::new("sepia")
            .about("Run sepia processing pipeline on an image")
    }
}
