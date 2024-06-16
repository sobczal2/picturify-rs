use clap::Arg;
use clap::builder::{IntoResettable, OsStr};

use crate::commands::common::args::common::PicturifyArg;
use crate::commands::common::command::CommandForImage;
use crate::commands::parsers::angle::AngleValueParser;
use crate::common::filter_group::Group;

struct RotateDefaultArgs {
    angle: &'static str,
}

const DEFAULT_ARGS: RotateDefaultArgs = RotateDefaultArgs {
    angle: "90deg",
};

pub struct RotateAngleArg;

impl PicturifyArg for RotateAngleArg {
    fn new(default_value: impl IntoResettable<OsStr>) -> Arg {
        Arg::new(Self::id())
            .short('a')
            .long("angle")
            .help("Rotation angle (e.g. 90deg, 2rad)")
            .default_value(default_value)
            .value_parser(AngleValueParser::new())
    }

    fn id() -> &'static str {
        "angle"
    }
}

pub struct RotateCommand;

impl CommandForImage for RotateCommand {
    fn get() -> clap::Command {
        Self::get_base()
            .arg(RotateAngleArg::new(DEFAULT_ARGS.angle))
    }

    fn name() -> &'static str {
        "rotate"
    }

    fn group() -> Group {
        Group::Geometry
    }
}
