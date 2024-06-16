use clap::{Arg, ArgAction, value_parser};
use clap::builder::{IntoResettable, OsStr};
use crate::commands::common::args::common::{FastArg, PicturifyArg};
use crate::commands::common::command::CommandForMovie;
use crate::common::filter_group::Group;

struct SobelDefaultArgs {
    fast: &'static str,
    rgb: &'static str,
}

const DEFAULT_ARGS: SobelDefaultArgs = SobelDefaultArgs {
    fast: "false",
    rgb: "false",
};

pub struct SobelRgbArg;

impl PicturifyArg for SobelRgbArg {
    fn new(default_value: impl IntoResettable<OsStr>) -> Arg {
        Arg::new(Self::id())
            .long("rgb")
            .help("Use RGB channels")
            .action(ArgAction::SetTrue)
            .default_value(default_value)
            .value_parser(value_parser!(f32))
    }

    fn id() -> &'static str {
        "rgb"
    }
}

pub struct SobelCommand;

impl CommandForMovie for SobelCommand {
    fn get() -> clap::Command {
        Self::get_base()
            .arg(FastArg::new(DEFAULT_ARGS.fast))
            .arg(SobelRgbArg::new(DEFAULT_ARGS.rgb))
    }

    fn name() -> &'static str {
        "sobel"
    }

    fn group() -> Group {
        Group::Edge
    }
}
