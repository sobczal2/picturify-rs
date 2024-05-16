use clap::Command;

pub struct NoneCommand;

impl NoneCommand {
    pub fn get() -> Command {
        Command::new("none").about("Perform no processing on an fast_image")
    }
}
