use crate::commands::common::arg::{add_input_output_args, ArgType};
use crate::commands::common::command::Command;

pub struct MedianBlurCommand;

impl Command for MedianBlurCommand {
    fn get() -> clap::Command {
        let cmd = clap::Command::new(Self::name())
            .arg(ArgType::Radius.to_arg())
            .arg(ArgType::Fast.to_arg())
            .about("Run median blur filter on the core");
        add_input_output_args(cmd)
    }

    fn name() -> &'static str {
        "median-blur"
    }
}
