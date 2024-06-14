use crate::commands::common::arg::ArgType;
use crate::commands::common::command::CommandForImage;
use crate::common::filter_group::Group;

pub struct SobelCommand;

impl CommandForImage for SobelCommand {
    fn get() -> clap::Command {
        Self::get_base()
            .arg(ArgType::Fast.to_arg())
            .arg(ArgType::Rgb.to_arg())
    }

    fn name() -> &'static str {
        "sobel"
    }

    fn group() -> Group {
        Group::Edge
    }
}
