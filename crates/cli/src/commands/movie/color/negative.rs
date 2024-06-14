use crate::commands::common::arg::ArgType;
use crate::commands::common::command::CommandForMovie;
use crate::common::filter_group::Group;

pub struct NegativeCommand;

impl CommandForMovie for NegativeCommand {
    fn get() -> clap::Command {
        Self::get_base().arg(ArgType::Fast.to_arg())
    }

    fn name() -> &'static str {
        "negative"
    }

    fn group() -> Group {
        Group::Color
    }
}
