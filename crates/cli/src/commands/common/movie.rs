use crate::commands::common::command::Command;
use crate::commands::movie::color::negative::NegativeCommand;
use crate::commands::movie::edge::sobel::SobelCommand;

pub struct MovieCommand;

impl Command for MovieCommand {
    fn get() -> clap::Command {
        clap::Command::new(Self::name())
            .about("Run movie processing-bench pipeline on a movie")
            .subcommands(&[
                NegativeCommand::get(),
                SobelCommand::get(),
            ])
    }

    fn name() -> &'static str {
        "movie"
    }
}
