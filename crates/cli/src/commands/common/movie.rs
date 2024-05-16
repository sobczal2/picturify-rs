use crate::commands::common::command::Command;

pub struct MovieCommand;

impl Command for MovieCommand {
    fn get() -> clap::Command {
        clap::Command::new(Self::name())
            .about("Run movie processing pipeline on a movie")
            .subcommands(&[])
    }

    fn name() -> &'static str {
        "movie"
    }
}