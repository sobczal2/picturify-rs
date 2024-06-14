use crate::commands::common::arg::ArgType;
use crate::commands::common::command::CommandForImage;
use crate::common::filter_group::Group;

pub struct SepiaCommand;

impl CommandForImage for SepiaCommand {
    fn get() -> clap::Command {
        Self::get_base()
            .arg(ArgType::Fast.to_arg())
    }

    fn name() -> &'static str {
        "sepia"
    }

    fn group() -> Group {
        Group::Color
    }
}
