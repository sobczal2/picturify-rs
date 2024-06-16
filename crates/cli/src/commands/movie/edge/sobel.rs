use crate::commands::common::args::common::{FastArg, PicturifyArg};
use crate::commands::common::command::CommandForMovie;
use crate::common::filter_group::Group;
use clap::builder::{IntoResettable, OsStr};
use clap::{value_parser, Arg, ArgAction};

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
    fn create(default_value: impl IntoResettable<OsStr>) -> Arg {
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
            .arg(FastArg::create(DEFAULT_ARGS.fast))
            .arg(SobelRgbArg::create(DEFAULT_ARGS.rgb))
    }

    fn name() -> &'static str {
        "sobel"
    }

    fn group() -> Group {
        Group::Edge
    }
}
