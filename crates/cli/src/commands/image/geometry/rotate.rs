use crate::commands::common::arg::ArgType;
use crate::commands::common::command::CommandForImage;
use crate::common::filter_group::Group;

pub struct RotateCommand;

impl CommandForImage for RotateCommand {
    fn get() -> clap::Command {
        Self::get_base()
            .arg(ArgType::Angle.to_arg())
    }

    fn name() -> &'static str {
        "rotate"
    }

    fn group() -> Group {
        Group::Geometry
    }
}
