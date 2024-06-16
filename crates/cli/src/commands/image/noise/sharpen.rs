use crate::commands::common::args::common::{FastArg, PicturifyArg};
use crate::commands::common::command::CommandForImage;
use crate::common::filter_group::Group;

struct SharpenDefaultArgs {
    fast: &'static str,
}

const DEFAULT_ARGS: SharpenDefaultArgs = SharpenDefaultArgs { fast: "false" };

pub struct SharpenCommand;

impl CommandForImage for SharpenCommand {
    fn get() -> clap::Command {
        Self::get_base().arg(FastArg::create(DEFAULT_ARGS.fast))
    }

    fn name() -> &'static str {
        "sharpen"
    }

    fn group() -> Group {
        Group::Noise
    }
}
