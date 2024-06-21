use clap::builder::{IntoResettable, OsStr};
use clap::Arg;

use crate::commands::common::args::common::PicturifyArg;
use crate::commands::common::command::Command;
use crate::commands::common::completions::CompletionsCommand;
use crate::commands::common::image::ImageCommand;
use crate::commands::common::movie::MovieCommand;
use crate::common::logging::LogLevelValueParser;
use crate::common::threading::CpuCountValueParser;
use crate::metadata;

struct PicturifyDefaultArgs {
    verbosity: &'static str,
    cpu_count: &'static str,
}

const DEFAULT_ARGS: PicturifyDefaultArgs = PicturifyDefaultArgs {
    verbosity: "info",
    cpu_count: "auto",
};

pub struct PicturifyVerbosityArg;

impl PicturifyArg for PicturifyVerbosityArg {
    fn create(default_value: impl IntoResettable<OsStr>) -> Arg {
        Arg::new(Self::id())
            .long("verbosity")
            .help("Verbosity level")
            .default_value(default_value)
            .value_parser(LogLevelValueParser)
            .global(true)
    }

    fn id() -> &'static str {
        "verbosity"
    }
}

pub struct PicturifyCpuCountArg;

impl PicturifyArg for PicturifyCpuCountArg {
    fn create(default_value: impl IntoResettable<OsStr>) -> Arg {
        Arg::new(Self::id())
            .long("cpu-count")
            .help("Number of CPU cores to use")
            .default_value(default_value)
            .global(true)
            .value_parser(CpuCountValueParser)
    }

    fn id() -> &'static str {
        "cpu-count"
    }
}

pub struct PicturifyCommand;

impl Command for PicturifyCommand {
    fn create() -> clap::Command {
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
            .subcommands(vec![
                ImageCommand::create(),
                MovieCommand::create(),
                CompletionsCommand::create(),
            ])
            .arg(PicturifyVerbosityArg::create(DEFAULT_ARGS.verbosity))
            .arg(PicturifyCpuCountArg::create(DEFAULT_ARGS.cpu_count))
    }
}
