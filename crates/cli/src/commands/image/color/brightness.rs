use crate::commands::common::arg::{add_input_output_args, ArgType};
use crate::commands::common::command::Command;

pub struct BrightnessCommand;

impl Command for BrightnessCommand {
    fn get() -> clap::Command {
        let cmd = clap::Command::new(Self::name())
            .arg(ArgType::Factor.to_arg())
            .about("Run brightness processing-bench pipeline on the image");
        add_input_output_args(cmd)
    }

    fn name() -> &'static str {
        "brightness"
    }
}
