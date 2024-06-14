use crate::commands::common::arg::ArgType;
use crate::commands::common::command::CommandForImage;
use crate::common::filter_group::Group;

pub struct GrayscaleCommand;

impl CommandForImage for GrayscaleCommand {
    fn get() -> clap::Command {
        Self::get_base()
            .arg(ArgType::GrayscaleStrategy.to_arg())
            .arg(ArgType::Fast.to_arg())
    }

    fn name() -> &'static str {
        "grayscale"
    }

    fn group() -> Group {
        Group::Color
    }
}
