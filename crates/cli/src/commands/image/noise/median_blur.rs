use crate::commands::common::args::common::{FastArg, PicturifyArg};
use crate::commands::common::command::CommandForImage;
use crate::common::filter_group::Group;
use clap::builder::{IntoResettable, OsStr};
use clap::{value_parser, Arg};

struct MedianBlurDefaultArgs {
    radius: &'static str,
    fast: &'static str,
}

const DEFAULT_ARGS: MedianBlurDefaultArgs = MedianBlurDefaultArgs {
    radius: "1",
    fast: "false",
};

pub struct MedianBlurRadiusArg;

impl PicturifyArg for MedianBlurRadiusArg {
    fn create(default_value: impl IntoResettable<OsStr>) -> Arg {
        Arg::new(Self::id())
            .short('r')
            .long("radius")
            .help("Kernel radius")
            .default_value(default_value)
            .value_parser(value_parser!(usize))
    }

    fn id() -> &'static str {
        "radius"
    }
}

pub struct MedianBlurCommand;

impl CommandForImage for MedianBlurCommand {
    fn get() -> clap::Command {
        Self::get_base()
            .arg(FastArg::create(DEFAULT_ARGS.fast))
            .arg(MedianBlurRadiusArg::create(DEFAULT_ARGS.radius))
    }

    fn name() -> &'static str {
        "median-blur"
    }

    fn group() -> Group {
        Group::Noise
    }
}
