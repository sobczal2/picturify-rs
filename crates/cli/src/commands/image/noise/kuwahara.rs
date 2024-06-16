use crate::commands::common::args::common::{FastArg, PicturifyArg};
use crate::commands::common::command::CommandForImage;
use crate::common::filter_group::Group;
use clap::builder::{IntoResettable, OsStr};
use clap::{value_parser, Arg};

struct KuwaharaBlurDefaultArgs {
    radius: &'static str,
    fast: &'static str,
}

const DEFAULT_ARGS: KuwaharaBlurDefaultArgs = KuwaharaBlurDefaultArgs {
    radius: "1",
    fast: "false",
};

pub struct KuwaharaRadiusArg;

impl PicturifyArg for KuwaharaRadiusArg {
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

pub struct KuwaharaCommand;

impl CommandForImage for KuwaharaCommand {
    fn get() -> clap::Command {
        Self::get_base()
            .arg(FastArg::create(DEFAULT_ARGS.fast))
            .arg(KuwaharaRadiusArg::create(DEFAULT_ARGS.radius))
    }

    fn name() -> &'static str {
        "kuwahara"
    }

    fn group() -> Group {
        Group::Noise
    }
}
