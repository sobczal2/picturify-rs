use crate::commands::common::arg::ArgType;
use crate::commands::common::command::CommandForImage;
use crate::common::filter_group::Group;

pub struct CropCommand;

impl CommandForImage for CropCommand {
    fn get() -> clap::Command {
        Self::get_base().arg(ArgType::Border.to_arg())
    }

    fn name() -> &'static str {
        "crop"
    }

    fn group() -> Group {
        Group::Geometry
    }
}
