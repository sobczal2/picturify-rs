use crate::commands::common::args::common::{FastArg, PicturifyArg};
use crate::commands::common::command::CommandForImage;
use crate::common::filter_group::Group;
use clap::builder::{IntoResettable, OsStr};
use clap::{value_parser, Arg};

struct GaussianBlurDefaultArgs {
    fast: &'static str,
    radius: &'static str,
    sigma: &'static str,
}

const DEFAULT_ARGS: GaussianBlurDefaultArgs = GaussianBlurDefaultArgs {
    radius: "1",
    fast: "false",
    sigma: "1.0",
};

pub struct GaussianBlurRadiusArg;

impl PicturifyArg for GaussianBlurRadiusArg {
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

pub struct GaussianBlurSigmaArg;

impl PicturifyArg for GaussianBlurSigmaArg {
    fn create(default_value: impl IntoResettable<OsStr>) -> Arg {
        Arg::new(Self::id())
            .short('s')
            .long("sigma")
            .help("Sigma value")
            .default_value(default_value)
            .value_parser(value_parser!(f32))
    }

    fn id() -> &'static str {
        "sigma"
    }
}

pub struct GaussianBlurCommand;

impl CommandForImage for GaussianBlurCommand {
    fn get() -> clap::Command {
        Self::get_base()
            .arg(FastArg::create(DEFAULT_ARGS.fast))
            .arg(GaussianBlurRadiusArg::create(DEFAULT_ARGS.radius))
            .arg(GaussianBlurSigmaArg::create(DEFAULT_ARGS.sigma))
    }

    fn name() -> &'static str {
        "gaussian-blur"
    }

    fn group() -> Group {
        Group::Noise
    }
}
