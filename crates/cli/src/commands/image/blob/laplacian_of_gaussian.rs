use crate::commands::common::args::common::{FastArg, PicturifyArg};
use crate::commands::common::command::CommandForImage;
use crate::common::filter_group::Group;
use clap::builder::{IntoResettable, OsStr};
use clap::{value_parser, Arg};

struct BilateralBlurDefaultArgs {
    radius: &'static str,
    fast: &'static str,
    sigma: &'static str,
}

const DEFAULT_ARGS: BilateralBlurDefaultArgs = BilateralBlurDefaultArgs {
    radius: "1",
    fast: "false",
    sigma: "1.0",
};

pub struct LaplacianOfGaussianRadiusArg;

impl PicturifyArg for LaplacianOfGaussianRadiusArg {
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

pub struct LaplacianOfGaussianSigmaArg;

impl PicturifyArg for LaplacianOfGaussianSigmaArg {
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

pub struct LaplacianOfGaussianCommand;

impl CommandForImage for LaplacianOfGaussianCommand {
    fn get() -> clap::Command {
        Self::get_base()
            .arg(FastArg::create(DEFAULT_ARGS.fast))
            .arg(LaplacianOfGaussianRadiusArg::create(DEFAULT_ARGS.radius))
            .arg(LaplacianOfGaussianSigmaArg::create(DEFAULT_ARGS.sigma))
    }

    fn name() -> &'static str {
        "laplacian-of-gaussian"
    }

    fn group() -> Group {
        Group::Blob
    }
}
