use clap::{Arg, value_parser};
use clap::builder::{IntoResettable, OsStr};
use crate::commands::common::args::common::{FastArg, PicturifyArg};
use crate::commands::common::command::CommandForImage;
use crate::common::filter_group::Group;

struct QuantizationDefaultArgs {
    fast: &'static str,
}

const DEFAULT_ARGS: QuantizationDefaultArgs = QuantizationDefaultArgs {
    fast: "false",
};

pub struct QuantizationLevelsArg;

impl PicturifyArg for QuantizationLevelsArg {
    fn new(default_value: impl IntoResettable<OsStr>) -> Arg {
        Arg::new(Self::id())
            .short('l')
            .long("levels")
            .help("Number of levels")
            .default_value(default_value)
            .value_parser(value_parser!(u8))
    }

    fn id() -> &'static str {
        "levels"
    }
}

pub struct QuantizationCommand;

impl CommandForImage for QuantizationCommand {
    fn get() -> clap::Command {
        Self::get_base()
            .arg(FastArg::new(DEFAULT_ARGS.fast))
            .arg(QuantizationLevelsArg::new(None))
    }

    fn name() -> &'static str {
        "quantization"
    }

    fn group() -> Group {
        Group::Color
    }
}
