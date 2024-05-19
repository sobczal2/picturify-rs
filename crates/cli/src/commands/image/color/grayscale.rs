use crate::commands::common::arg::{add_input_output_args, ArgType};
use crate::commands::common::command::Command;

pub struct GrayscaleCommand;

impl Command for GrayscaleCommand {
    fn get() -> clap::Command {
        let cmd = clap::Command::new(Self::name())
            .arg(ArgType::GrayscaleStrategy.to_arg())
            .arg(ArgType::Fast.to_arg())
            .about("Run grayscale processing pipeline on the image");
        add_input_output_args(cmd)
    }

    fn name() -> &'static str {
        "grayscale"
    }
}
