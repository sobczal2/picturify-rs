use crate::commands::color::negative::NegativeCommand;
use crate::commands::color::sepia::SepiaCommand;
use crate::commands::common::none::NoneCommand;
use crate::commands::noise::kuwahara::KuwaharaCommand;
use crate::commands::noise::median::MedianCommand;
use clap::{Arg, Command};

pub struct ImageCommand;

impl ImageCommand {
    pub fn get() -> Command {
        Command::new("image")
            .about("Run image processing pipeline on an fast_image")
            .args(&[
                Arg::new("input")
                    .help("Input fast_image file")
                    .index(1)
                    .required(true),
                Arg::new("output")
                    .help("Output fast_image file")
                    .index(2)
                    .required(true),
            ])
            .subcommands([
                SepiaCommand::get(),
                NegativeCommand::get(),
                NoneCommand::get(),
                KuwaharaCommand::get(),
                MedianCommand::get(),
            ])
    }
}
