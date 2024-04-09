use clap::{Arg, Command};
use crate::commands::common::color::negative::NegativeCommand;

use crate::commands::common::color::sepia::SepiaCommand;
use crate::commands::common::common::none::NoneCommand;

pub struct ImageCommand;

impl ImageCommand {
    pub fn new() -> Command {
        Command::new("image")
            .about("Run image processing pipeline on an image")
            .args(&[
                Arg::new("input")
                    .help("Input image file")
                    .index(1)
                    .required(true),
                Arg::new("output")
                    .help("Output image file")
                    .index(2)
                    .required(true),
            ])
            .subcommands([
                SepiaCommand::new(),
                NegativeCommand::new(),
                NoneCommand::new()
            ])
    }
}
