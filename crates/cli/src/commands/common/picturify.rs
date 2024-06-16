use clap::{Arg, value_parser};
use clap::builder::{IntoResettable, OsStr};
use log::LevelFilter;
use crate::commands::common::args::common::{PicturifyArg};
use crate::commands::common::command::Command;
use crate::commands::common::image::ImageCommand;
use crate::commands::common::movie::MovieCommand;
use crate::metadata;

struct PicturifyDefaultArgs {
    verbosity: &'static str,
}

const DEFAULT_ARGS: PicturifyDefaultArgs = PicturifyDefaultArgs {
    verbosity: "info",
};

pub struct PicturifyVerbosityArg;

impl PicturifyArg for PicturifyVerbosityArg {
    fn new(default_value: impl IntoResettable<OsStr>) -> Arg {
        Arg::new(Self::id())
            .short('v')
            .long("verbosity")
            .help("Verbosity level")
            .default_value(default_value)
            .value_parser(value_parser!(LevelFilter))
            .global(true)
    }

    fn id() -> &'static str {
        "verbosity"
    }
}

pub struct PicturifyCommand;

impl Command for PicturifyCommand {
    fn get() -> clap::Command {
        clap::Command::new(metadata::NAME)
            .version(metadata::VERSION)
            .author(metadata::AUTHORS)
            .about(format!(
                "{} - a CLI tool for core processing using the Picturify library\n\n\
                \t{} v{}\n\
                \t{} v{}\n\
                \t{} v{}\n\
                \t{} v{}\n",
                metadata::NAME,
                metadata::NAME,
                metadata::VERSION,
                picturify_core::metadata::NAME,
                picturify_core::metadata::VERSION,
                picturify_processing::metadata::NAME,
                picturify_processing::metadata::VERSION,
                picturify_movie::metadata::NAME,
                picturify_movie::metadata::VERSION,
            ))
            .subcommands(vec![ImageCommand::get(), MovieCommand::get()])
            .arg(PicturifyVerbosityArg::new(DEFAULT_ARGS.verbosity))
    }
}
