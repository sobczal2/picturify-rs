use clap::Command;

use crate::commands::common::color::sepia::SepiaCommand;
use crate::commands::common::common::none::NoneCommand;

pub struct ImageCommand;

impl ImageCommand {
    pub fn new() -> Command {
        Command::new("image")
            .about("Run image processing pipeline on an image")
            .subcommands([
                SepiaCommand::new(),
                NoneCommand::new(),
            ])
    }
}
