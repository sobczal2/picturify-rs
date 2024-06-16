use clap::{Arg, value_parser};
use clap::builder::{IntoResettable, OsStr};
use crate::commands::common::args::common::{FastArg, PicturifyArg};
use crate::commands::common::command::CommandForImage;
use crate::common::filter_group::Group;

struct KuwaharaBlurDefaultArgs {
    radius: &'static str,
    fast: &'static str,
    sigma: &'static str,
}

const DEFAULT_ARGS: KuwaharaBlurDefaultArgs = KuwaharaBlurDefaultArgs {
    radius: "1",
    fast: "false",
    sigma: "1.0",
};

pub struct KuwaharaRadiusArg;

impl PicturifyArg for KuwaharaRadiusArg {
    fn new(default_value: impl IntoResettable<OsStr>) -> Arg {
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
            .arg(FastArg::new(DEFAULT_ARGS.fast))
            .arg(KuwaharaRadiusArg::new(DEFAULT_ARGS.radius))
    }

    fn name() -> &'static str {
        "kuwahara"
    }

    fn group() -> Group {
        Group::Noise
    }
}
