use crate::commands::common::command::CommandForImage;
use crate::common::filter_group::Group;

pub struct PassthroughCommand;

impl CommandForImage for PassthroughCommand {
    fn get() -> clap::Command {
        Self::get_base()
    }

    fn name() -> &'static str {
        "passthrough"
    }

    fn group() -> Group {
        Group::Common
    }
}
