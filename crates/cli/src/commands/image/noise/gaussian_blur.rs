use crate::commands::common::arg::{add_input_output_args, ArgType};
use crate::commands::common::command::Command;

pub struct GaussianBlurCommand;

impl Command for GaussianBlurCommand {
    fn get() -> clap::Command {
        let cmd = clap::Command::new(Self::name())
            .arg(ArgType::Radius.to_arg())
            .arg(ArgType::Fast.to_arg())
            .arg(ArgType::Sigma.to_arg())
            .about("Run gaussian blur filter on the image");
        add_input_output_args(cmd)
    }

    fn name() -> &'static str {
        "gaussian-blur"
    }
}
