use clap::{Arg, Command, value_parser};

pub struct KuwaharaCommand;

impl KuwaharaCommand {
    pub fn new() -> Command {
        Command::new("kuwahara")
            .arg(
                Arg::new("radius")
                    .help("Radius of the kuwahara filter")
                    .default_value("3")
                    .value_parser(value_parser!(usize)),
            )
            .about("Run kuwahara processing pipeline on an fast_image")
    }
}