use crate::commands::common::args::common::PicturifyArg;
use crate::commands::common::command::CommandForImage;
use crate::common::filter_group::Group;
use clap::builder::{IntoResettable, OsStr};
use clap::{value_parser, Arg};

pub struct BrightnessFactorArg;

impl PicturifyArg for BrightnessFactorArg {
    fn create(default_value: impl IntoResettable<OsStr>) -> Arg {
        Arg::new(Self::id())
            .short('f')
            .long("factor")
            .help("Brightness factor")
            .default_value(default_value)
            .value_parser(value_parser!(f32))
    }

    fn id() -> &'static str {
        "factor"
    }
}

pub struct BrightnessCommand;

impl CommandForImage for BrightnessCommand {
    fn get() -> clap::Command {
        Self::get_base().arg(BrightnessFactorArg::create(None))
    }

    fn name() -> &'static str {
        "brightness"
    }

    fn group() -> Group {
        Group::Color
    }
}
