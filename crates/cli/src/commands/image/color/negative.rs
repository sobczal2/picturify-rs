use crate::commands::common::arg::{add_input_output_args, ArgType};
use crate::commands::common::command::Command;

pub struct NegativeCommand;

impl Command for NegativeCommand {
    fn get() -> clap::Command {
        let cmd = clap::Command::new(Self::name())
            .arg(ArgType::Fast.to_arg())
            .about("Run negative processing-bench pipeline on the image");
        add_input_output_args(cmd)
    }

    fn name() -> &'static str {
        "negative"
    }
}
