use clap::{Arg, Command, value_parser};

pub struct SobelRgbCommand;

impl SobelRgbCommand {
    pub fn get() -> Command {
        Command::new("sobel_rgb")
            .arg(
                Arg::new("use_fast_approximation")
                    .help("Use fast approximation for sobel filter")
                    .default_value("false")
                    .value_parser(value_parser!(bool)),
            )
            .about("Run sobel rgb processing pipeline on an fast_image")
    }
}