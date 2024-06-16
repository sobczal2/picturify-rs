use crate::commands::common::args::common::{FastArg, PicturifyArg};
use crate::commands::common::command::CommandForImage;
use crate::common::filter_group::Group;
use clap::builder::{IntoResettable, OsStr};
use clap::{value_parser, Arg, ArgAction};

struct PrewittDefaultArgs {
    fast: &'static str,
    rgb: &'static str,
}

const DEFAULT_ARGS: PrewittDefaultArgs = PrewittDefaultArgs {
    fast: "false",
    rgb: "false",
};

pub struct PrewittRgbArg;

impl PicturifyArg for PrewittRgbArg {
    fn create(default_value: impl IntoResettable<OsStr>) -> Arg {
        Arg::new(Self::id())
            .long("rgb")
            .help("Use RGB channels")
            .action(ArgAction::SetTrue)
            .default_value(default_value)
            .value_parser(value_parser!(bool))
    }

    fn id() -> &'static str {
        "rgb"
    }
}

pub struct PrewittCommand;

impl CommandForImage for PrewittCommand {
    fn get() -> clap::Command {
        Self::get_base()
            .arg(FastArg::create(DEFAULT_ARGS.fast))
            .arg(PrewittRgbArg::create(DEFAULT_ARGS.rgb))
    }

    fn name() -> &'static str {
        "prewitt"
    }

    fn group() -> Group {
        Group::Edge
    }
}
