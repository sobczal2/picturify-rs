use crate::commands::common::args::common::{FastArg, PicturifyArg};
use crate::commands::common::command::CommandForImage;
use crate::common::filter_group::Group;
use clap::builder::{IntoResettable, OsStr};
use clap::{value_parser, Arg};

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

pub struct BilateralBlurSpatialSigmaArg;

impl PicturifyArg for BilateralBlurSpatialSigmaArg {
    fn create(default_value: impl IntoResettable<OsStr>) -> Arg {
        Arg::new(Self::id())
            .long("spatial-sigma")
            .alias("ss")
            .help("Spatial sigma value")
            .default_value(default_value)
            .value_parser(value_parser!(f32))
    }

    fn id() -> &'static str {
        "spatial-sigma"
    }
}

pub struct BilateralBlurIntensitySigmaArg;

impl PicturifyArg for BilateralBlurIntensitySigmaArg {
    fn create(default_value: impl IntoResettable<OsStr>) -> Arg {
        Arg::new(Self::id())
            .long("intensity-sigma")
            .alias("is")
            .help("Sigma intensity value")
            .default_value(default_value)
            .value_parser(value_parser!(f32))
    }

    fn id() -> &'static str {
        "intensity-sigma"
    }
}

pub struct BilateralBlurCommand;

impl CommandForImage for BilateralBlurCommand {
    fn get() -> clap::Command {
        Self::get_base()
            .arg(FastArg::create(DEFAULT_ARGS.fast))
            .arg(BilateralBlurRadiusArg::create(DEFAULT_ARGS.radius))
            .arg(BilateralBlurSpatialSigmaArg::create(
                DEFAULT_ARGS.sigma_spatial,
            ))
            .arg(BilateralBlurIntensitySigmaArg::create(
                DEFAULT_ARGS.sigma_intensity,
            ))
    }

    fn name() -> &'static str {
        "bilateral-blur"
    }

    fn group() -> Group {
        Group::Noise
    }
}
