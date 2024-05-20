use crate::commands::common::arg::{add_input_output_args, ArgType};
use crate::commands::common::command::Command;

pub struct BilateralCommand;

impl Command for BilateralCommand {
    fn get() -> clap::Command {
        let cmd = clap::Command::new(Self::name())
            .arg(ArgType::Radius.to_arg())
            .arg(ArgType::Fast.to_arg())
            .arg(ArgType::SigmaSpatial.to_arg())
            .arg(ArgType::SigmaIntensity.to_arg())
            .about("Run bilateral noise processing pipeline on the image");
        add_input_output_args(cmd)
    }

    fn name() -> &'static str {
        "bilateral"
    }
}
