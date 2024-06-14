use crate::commands::common::arg::{add_input_output_args, ArgType};
use crate::commands::common::command::Command;

pub struct RotateCommand;

impl Command for RotateCommand {
    fn get() -> clap::Command {
        let cmd = clap::Command::new(Self::name())
            .arg(ArgType::Angle.to_arg())
            .about("Rotate the image clockwise");
        add_input_output_args(cmd)
    }

    fn name() -> &'static str {
        "rotate"
    }
}
