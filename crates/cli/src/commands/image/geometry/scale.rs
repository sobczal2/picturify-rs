use clap::Arg;
use clap::builder::{IntoResettable, OsStr};
use crate::commands::common::args::common::PicturifyArg;
use crate::commands::common::command::CommandForImage;
use crate::commands::parsers::scale_strategy::ScaleStrategyValueParser;
use crate::commands::parsers::size::SizeValueParser;
use crate::common::filter_group::Group;

struct ScaleDefaultArgs {
    strategy: &'static str,
}

const DEFAULT_ARGS: ScaleDefaultArgs = ScaleDefaultArgs {
    strategy: "nearest-neighbor",
};

pub struct ScaleSizeArg;

impl PicturifyArg for ScaleSizeArg {
    fn new(default_value: impl IntoResettable<OsStr>) -> Arg {
        Arg::new(Self::id())
            .long("size")
            .help("Size to scale to (e.g. 800x600)")
            .default_value(default_value)
            .value_parser(SizeValueParser::new())
    }

    fn id() -> &'static str {
        "size"
    }
}

pub struct ScaleStrategyArg;

impl PicturifyArg for ScaleStrategyArg {
    fn new(default_value: impl IntoResettable<OsStr>) -> Arg {
        Arg::new(Self::id())
            .long("strategy")
            .help("Scaling strategy (nearest-neighbor(nn), bilinear(bl))")
            .default_value(default_value)
            .value_parser(ScaleStrategyValueParser::new())
    }

    fn id() -> &'static str {
        "strategy"
    }
}

pub struct ScaleCommand;

impl CommandForImage for ScaleCommand {
    fn get() -> clap::Command {
        Self::get_base()
            .arg(ScaleSizeArg::new(None))
            .arg(ScaleStrategyArg::new(None))
    }

    fn name() -> &'static str {
        "scale"
    }

    fn group() -> Group {
        Group::Geometry
    }
}
