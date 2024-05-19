use crate::commands::common::arg::add_input_output_args;
use crate::commands::common::command::Command;

pub struct NegativeCommand;

impl Command for NegativeCommand {
    fn get() -> clap::Command {
        let cmd =
            clap::Command::new(Self::name()).about("Run negative processing pipeline on the movie");
        add_input_output_args(cmd)
    }

    fn name() -> &'static str {
        "negative"
    }
}
