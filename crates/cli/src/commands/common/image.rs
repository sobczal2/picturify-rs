use crate::commands::common::none::NoneCommand;
use clap::{Arg, Command};
use crate::commands::image::color::negative::NegativeCommand;
use crate::commands::image::color::sepia::SepiaCommand;
use crate::commands::image::edge::sobel::SobelCommand;
use crate::commands::image::edge::sobel_rgb::SobelRgbCommand;
use crate::commands::image::noise::kuwahara::KuwaharaCommand;
use crate::commands::image::noise::median::MedianCommand;

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
                NoneCommand::get(),
                // color
                SepiaCommand::get(),
                NegativeCommand::get(),
                // noise
                KuwaharaCommand::get(),
                MedianCommand::get(),
                // edge
                SobelCommand::get(),
                SobelRgbCommand::get(),
            ])
    }
}
