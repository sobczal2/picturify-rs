use clap::{Arg, value_parser};
use clap::builder::{IntoResettable, OsStr};
use crate::commands::common::args::common::{FastArg, PicturifyArg};
use crate::commands::common::command::CommandForImage;
use crate::common::filter_group::Group;


struct BilateralBlurDefaultArgs {
    radius: &'static str,
    fast: &'static str,
    sigma_spatial: &'static str,
    sigma_intensity: &'static str,
}

const DEFAULT_ARGS: BilateralBlurDefaultArgs = BilateralBlurDefaultArgs {
    radius: "1",
    fast: "false",
    sigma_spatial: "1.0",
    sigma_intensity: "1.0",
};

pub struct BilateralBlurRadiusArg;

impl PicturifyArg for BilateralBlurRadiusArg {
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

pub struct BilateralBlurSpatialSigmaArg;

impl PicturifyArg for BilateralBlurSpatialSigmaArg {
    fn new(default_value: impl IntoResettable<OsStr>) -> Arg {
        Arg::new(Self::id())
            .long("sigma-spatial")
            .alias("ss")
            .help("Sigma spatial value")
            .default_value(default_value)
            .value_parser(value_parser!(f32))
    }

    fn id() -> &'static str {
        "sigma-spatial"
    }
}

pub struct BilateralBlurIntensitySigmaArg;

impl PicturifyArg for BilateralBlurIntensitySigmaArg {
    fn new(default_value: impl IntoResettable<OsStr>) -> Arg {
        Arg::new(Self::id())
            .long("sigma-intensity")
            .alias("si")
            .help("Sigma intensity value")
            .default_value(default_value)
            .value_parser(value_parser!(f32))
    }

    fn id() -> &'static str {
        "sigma-intensity"
    }
}

pub struct BilateralBlurCommand;

impl CommandForImage for BilateralBlurCommand {
    fn get() -> clap::Command {
        Self::get_base()
            .arg(FastArg::new(DEFAULT_ARGS.fast))
            .arg(BilateralBlurRadiusArg::new(DEFAULT_ARGS.radius))
            .arg(BilateralBlurSpatialSigmaArg::new(DEFAULT_ARGS.sigma_spatial))
            .arg(BilateralBlurIntensitySigmaArg::new(DEFAULT_ARGS.sigma_intensity))
    }

    fn name() -> &'static str {
        "bilateral-blur"
    }

    fn group() -> Group {
        Group::Noise
    }
}
