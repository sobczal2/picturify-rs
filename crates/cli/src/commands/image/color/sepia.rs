use crate::commands::common::arg::{add_input_output_args, ArgType};
use crate::commands::common::command::Command;

pub struct SepiaCommand;

impl Command for SepiaCommand {
    fn get() -> clap::Command {
        let cmd = clap::Command::new(Self::name())
            .arg(ArgType::Fast.to_arg())
            .about("Run sepia filter on the core");
        add_input_output_args(cmd)
    }

    fn name() -> &'static str {
        "sepia"
    }
}
