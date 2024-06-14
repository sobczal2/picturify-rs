use crate::commands::common::arg::ArgType;
use crate::commands::common::command::CommandForImage;
use crate::common::filter_group::Group;

pub struct SharpenCommand;

impl CommandForImage for SharpenCommand {
    fn get() -> clap::Command {
        Self::get_base().arg(ArgType::Fast.to_arg())
    }

    fn name() -> &'static str {
        "sharpen"
    }

    fn group() -> Group {
        Group::Noise
    }
}
