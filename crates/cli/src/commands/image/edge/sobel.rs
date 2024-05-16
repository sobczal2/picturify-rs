use clap::{Arg, Command, value_parser};

pub struct SobelCommand;

impl SobelCommand {
    pub fn get() -> Command {
        Command::new("sobel")
            .arg(
                Arg::new("use_fast_approximation")
                    .help("Use fast approximation for sobel filter")
                    .default_value("false")
                    .value_parser(value_parser!(bool)),
            )
            .about("Run sobel processing pipeline on an fast_image")
    }
}