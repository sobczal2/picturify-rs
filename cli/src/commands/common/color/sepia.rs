use clap::{Arg, Command};

pub struct SepiaCommand;

impl SepiaCommand {
    pub fn new() -> Command {
        Command::new("sepia")
            .args(vec![
                Arg::new("input").index(1).required(true),
                Arg::new("output").index(2).required(true),
            ])
            .about("Run sepia processing pipeline on an image")
    }
}
