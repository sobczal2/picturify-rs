use clap::Command;

pub struct MovieCommand;

impl MovieCommand {
    pub fn new() -> Command {
        Command::new("movie").about("Run movie processing pipeline on a movie")
    }
}
