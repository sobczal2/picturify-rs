use clap::Command;

use crate::commands::common::color::sepia::SepiaCommand;

pub struct ImageCommand;

impl ImageCommand {
    pub fn new() -> Command {
        Command::new("image")
            .about("Run image processing pipeline on an image")
            .subcommands([SepiaCommand::new()])
    }
}
