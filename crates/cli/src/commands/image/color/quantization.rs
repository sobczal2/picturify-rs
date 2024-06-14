use crate::commands::common::arg::{add_input_output_args, ArgType};
use crate::commands::common::command::Command;

pub struct QuantizationCommand;

impl Command for QuantizationCommand {
    fn get() -> clap::Command {
        let cmd = clap::Command::new(Self::name())
            .arg(ArgType::Fast.to_arg())
            .arg(ArgType::Levels.to_arg())
            .about("Run quantization filter on the image");
        add_input_output_args(cmd)
    }

    fn name() -> &'static str {
        "quantization"
    }
}
