use crate::commands::common::args::common::{FastArg, PicturifyArg};
use crate::commands::common::command::CommandForMovie;
use crate::common::filter_group::Group;

struct NegativeDefaultArgs {
    fast: &'static str,
}

const DEFAULT_ARGS: NegativeDefaultArgs = NegativeDefaultArgs {
    fast: "false",
};

pub struct NegativeCommand;

impl CommandForMovie for NegativeCommand {
    fn get() -> clap::Command {
        Self::get_base()
            .arg(FastArg::new(DEFAULT_ARGS.fast))
    }

    fn name() -> &'static str {
        "negative"
    }

    fn group() -> Group {
        Group::Color
    }
}
