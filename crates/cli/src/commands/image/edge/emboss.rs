use crate::commands::common::args::common::{FastArg, PicturifyArg};
use crate::commands::common::command::CommandForImage;
use crate::common::filter_group::Group;

struct EmbossDefaultArgs {
    fast: &'static str,
}

const DEFAULT_ARGS: EmbossDefaultArgs = EmbossDefaultArgs { fast: "false" };

pub struct EmbossCommand;

impl CommandForImage for EmbossCommand {
    fn get() -> clap::Command {
        Self::get_base().arg(FastArg::create(DEFAULT_ARGS.fast))
    }

    fn name() -> &'static str {
        "emboss"
    }

    fn group() -> Group {
        Group::Edge
    }
}
