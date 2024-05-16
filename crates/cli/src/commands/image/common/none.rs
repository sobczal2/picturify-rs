use crate::commands::common::arg::add_input_output_args;
use crate::commands::common::command::Command;

pub struct NoneCommand;

impl Command for NoneCommand {
    fn get() -> clap::Command {
        let cmd = clap::Command::new(Self::name())
            .about("Does nothing to the image");
        add_input_output_args(cmd)
    }

    fn name() -> &'static str {
        "none"
    }
}
