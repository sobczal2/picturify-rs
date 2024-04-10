use clap::{Arg, Command, value_parser};

pub struct MedianCommand;

impl MedianCommand {
    pub fn new() -> Command {
        Command::new("median")
            .arg(
                Arg::new("radius")
                    .help("Radius of the median filter")
                    .default_value("3")
                    .value_parser(value_parser!(usize)),
            )
            .about("Run median processing pipeline on an fast_image")
    }
}