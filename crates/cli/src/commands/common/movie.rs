use crate::commands::common::command::{Command, CommandForMovie};
use crate::commands::movie::color::negative::NegativeCommand;
use crate::commands::movie::edge::sobel::SobelCommand;

pub struct MovieCommand;

impl Command for MovieCommand {
    fn create() -> clap::Command {
        clap::Command::new("movie")
            .about("Run processing pipeline on the movie")
            .subcommands(&[NegativeCommand::get(), SobelCommand::get()])
    }
}
