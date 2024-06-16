use crate::commands::common::args::common::PicturifyArg;
use crate::commands::common::command::Command;
use crate::commands::common::image::ImageCommand;
use crate::commands::common::movie::MovieCommand;
use crate::common::logging::LogLevel;
use crate::metadata;
use clap::builder::{IntoResettable, OsStr, PossibleValue, TypedValueParser};
use clap::error::{Error, ErrorKind};
use clap::Arg;

struct PicturifyDefaultArgs {
    verbosity: &'static str,
}

const DEFAULT_ARGS: PicturifyDefaultArgs = PicturifyDefaultArgs { verbosity: "info" };

pub struct PicturifyVerbosityArg;

impl PicturifyArg for PicturifyVerbosityArg {
    fn create(default_value: impl IntoResettable<OsStr>) -> Arg {
        Arg::new(Self::id())
            .short('v')
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

#[derive(Debug, Clone, Copy)]
struct LogLevelValueParser;

impl TypedValueParser for LogLevelValueParser {
    type Value = LogLevel;

    fn parse_ref(
        &self,
        _cmd: &clap::Command,
        _arg: Option<&Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, Error> {
        match value.to_str() {
            Some("off") => Ok(LogLevel::Off),
            Some("error") => Ok(LogLevel::Error),
            Some("warn") => Ok(LogLevel::Warn),
            Some("info") => Ok(LogLevel::Info),
            Some("debug") => Ok(LogLevel::Debug),
            Some("trace") => Ok(LogLevel::Trace),
            _ => Err(Error::raw(
                ErrorKind::InvalidValue,
                "Invalid size, expected format: <width>x<height>\n",
            )),
        }
    }

    fn possible_values(&self) -> Option<Box<dyn Iterator<Item = PossibleValue> + '_>> {
        Some(Box::new(
            ["off", "error", "warn", "info", "debug", "trace"]
                .iter()
                .map(PossibleValue::new),
        ))
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
            .subcommands(vec![ImageCommand::create(), MovieCommand::create()])
            .arg(PicturifyVerbosityArg::create(DEFAULT_ARGS.verbosity))
    }
}
