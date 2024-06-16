use clap::{Arg, value_parser};
use clap::builder::{IntoResettable, OsStr};
use picturify_processing::processors::color::grayscale::GrayscaleStrategy;
use crate::commands::common::args::common::{FastArg, PicturifyArg};
use crate::commands::common::command::CommandForImage;
use crate::common::filter_group::Group;

struct GrayscaleDefaultArgs {
    fast: &'static str,
    strategy: &'static str,
}

const DEFAULT_ARGS: GrayscaleDefaultArgs = GrayscaleDefaultArgs {
    fast: "false",
    strategy: "average",
};

pub struct GrayscaleStrategyArg;

impl PicturifyArg for GrayscaleStrategyArg {
    fn new(default_value: impl IntoResettable<OsStr>) -> Arg {
        Arg::new(Self::id())
            .short('s')
            .long("strategy")
            .help("Grayscale strategy (average, lightness, luminosity)")
            .default_value(default_value)
            .value_parser(value_parser!(GrayscaleStrategy))
    }

    fn id() -> &'static str {
        "strategy"
    }
}

pub struct GrayscaleCommand;

impl CommandForImage for GrayscaleCommand {
    fn get() -> clap::Command {
        Self::get_base()
            .arg(FastArg::new(DEFAULT_ARGS.fast))
            .arg(GrayscaleStrategyArg::new(DEFAULT_ARGS.strategy))
    }

    fn name() -> &'static str {
        "grayscale"
    }

    fn group() -> Group {
        Group::Color
    }
}
