use clap::Command;

pub struct SepiaCommand;

impl SepiaCommand {
    pub fn get() -> Command {
        Command::new("sepia").about("Run sepia processing pipeline on an fast_image")
    }
}
