use crate::commands::common::args::common::{FastArg, PicturifyArg};
use crate::commands::common::command::CommandForImage;
use crate::common::filter_group::Group;

struct NegativeDefaultArgs {
    fast: &'static str,
}

const DEFAULT_ARGS: NegativeDefaultArgs = NegativeDefaultArgs { fast: "false" };

pub struct NegativeCommand;

impl CommandForImage for NegativeCommand {
    fn get() -> clap::Command {
        Self::get_base().arg(FastArg::create(DEFAULT_ARGS.fast))
    }

    fn name() -> &'static str {
        "negative"
    }

    fn group() -> Group {
        Group::Color
    }
}
