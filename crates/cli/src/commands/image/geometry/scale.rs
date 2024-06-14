use crate::commands::common::arg::ArgType;
use crate::commands::common::command::CommandForImage;
use crate::common::filter_group::Group;

pub struct ScaleCommand;

impl CommandForImage for ScaleCommand {
    fn get() -> clap::Command {
        Self::get_base()
            .arg(ArgType::Size.to_arg())
            .arg(ArgType::ScaleStrategy.to_arg())
    }

    fn name() -> &'static str {
        "scale"
    }

    fn group() -> Group {
        Group::Geometry
    }
}
