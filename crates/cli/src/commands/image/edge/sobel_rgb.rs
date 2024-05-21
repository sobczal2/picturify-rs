use crate::commands::common::arg::{add_input_output_args, ArgType};
use crate::commands::common::command::Command;

pub struct SobelRgbCommand;

impl Command for SobelRgbCommand {
    fn get() -> clap::Command {
        let cmd = clap::Command::new(Self::name())
            .arg(ArgType::Fast.to_arg())
            .about("Run sobel rgb processing-bench pipeline on an image");
        add_input_output_args(cmd)
    }

    fn name() -> &'static str {
        "sobel-rgb"
    }
}
