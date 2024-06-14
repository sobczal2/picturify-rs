use crate::commands::common::arg::ArgType;
use crate::commands::common::command::CommandForImage;
use crate::common::filter_group::Group;

pub struct BrightnessCommand;

impl CommandForImage for BrightnessCommand {
    fn get() -> clap::Command {
        Self::get_base().arg(ArgType::Factor.to_arg())
    }

    fn name() -> &'static str {
        "brightness"
    }

    fn group() -> Group {
        Group::Color
    }
}
