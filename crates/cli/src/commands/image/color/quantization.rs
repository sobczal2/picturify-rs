use crate::commands::common::args::common::{FastArg, PicturifyArg};
use crate::commands::common::command::CommandForImage;
use crate::common::filter_group::Group;
use clap::builder::{IntoResettable, OsStr};
use clap::{value_parser, Arg};

struct QuantizationDefaultArgs {
    fast: &'static str,
}

const DEFAULT_ARGS: QuantizationDefaultArgs = QuantizationDefaultArgs { fast: "false" };

pub struct QuantizationLevelsArg;

impl PicturifyArg for QuantizationLevelsArg {
    fn create(default_value: impl IntoResettable<OsStr>) -> Arg {
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
            .arg(FastArg::create(DEFAULT_ARGS.fast))
            .arg(QuantizationLevelsArg::create(None))
    }

    fn name() -> &'static str {
        "quantization"
    }

    fn group() -> Group {
        Group::Color
    }
}
