use clap::builder::{IntoResettable, OsStr};
use clap::Arg;

use crate::commands::common::args::common::PicturifyArg;
use crate::commands::common::command::CommandForImage;
use crate::commands::parsers::crop_border::CropBorderValueParser;
use crate::common::filter_group::Group;

pub struct CropBorderArg;

impl PicturifyArg for CropBorderArg {
    fn create(default_value: impl IntoResettable<OsStr>) -> Arg {
        Arg::new(Self::id())
            .short('b')
            .long("border")
            .help("Border in format <width>x<height>+<left>+<top>")
            .default_value(default_value)
            .value_parser(CropBorderValueParser::new())
    }

    fn id() -> &'static str {
        "border"
    }
}

pub struct CropCommand;

impl CommandForImage for CropCommand {
    fn get() -> clap::Command {
        Self::get_base().arg(CropBorderArg::create(None))
    }

    fn name() -> &'static str {
        "crop"
    }

    fn group() -> Group {
        Group::Geometry
    }
}
