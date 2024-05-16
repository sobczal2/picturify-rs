use crate::commands::common::arg::{add_input_output_args, ArgType};
use crate::commands::common::command::Command;

pub struct MeanCommand;

impl Command for MeanCommand {
    fn get() -> clap::Command {
        let cmd = clap::Command::new(Self::name())
            .arg(
                ArgType::Radius.to_arg(),
            )
            .arg(
                ArgType::UseFastApproximation.to_arg(),
            )
            .about("Run mean processing pipeline on an fast_image");
        add_input_output_args(cmd)
    }

    fn name() -> &'static str {
        "mean"
    }
}
