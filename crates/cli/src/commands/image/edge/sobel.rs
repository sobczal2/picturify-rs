use crate::commands::common::arg::{add_input_output_args, ArgType};
use crate::commands::common::command::Command;

pub struct SobelCommand;

impl Command for SobelCommand {
    fn get() -> clap::Command {
        let cmd = clap::Command::new(Self::name())
            .arg(ArgType::Fast.to_arg())
            .arg(ArgType::Rgb.to_arg())
            .about("Run sobel processing-bench pipeline on the image");
        add_input_output_args(cmd)
    }

    fn name() -> &'static str {
        "sobel"
    }
}
