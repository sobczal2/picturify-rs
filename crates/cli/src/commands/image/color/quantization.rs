use crate::commands::common::arg::ArgType;
use crate::commands::common::command::CommandForImage;
use crate::common::filter_group::Group;

pub struct QuantizationCommand;

impl CommandForImage for QuantizationCommand {
    fn get() -> clap::Command {
        Self::get_base()
            .arg(ArgType::Fast.to_arg())
            .arg(ArgType::Levels.to_arg())
    }

    fn name() -> &'static str {
        "quantization"
    }

    fn group() -> Group {
        Group::Color
    }
}
