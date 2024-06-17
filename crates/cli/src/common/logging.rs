use clap::Arg;
use clap::builder::{PossibleValue, TypedValueParser};
use clap::error::{Error, ErrorKind};
use log::LevelFilter;
use picturify_core::log_info;

#[derive(Debug, Clone)]
pub enum LogLevel {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl From<LogLevel> for LevelFilter {
    fn from(log_level: LogLevel) -> LevelFilter {
        match log_level {
            LogLevel::Off => LevelFilter::Off,
            LogLevel::Error => LevelFilter::Error,
            LogLevel::Warn => LevelFilter::Warn,
            LogLevel::Info => LevelFilter::Info,
            LogLevel::Debug => LevelFilter::Debug,
            LogLevel::Trace => LevelFilter::Trace,
        }
    }
}

pub fn log_help(cmd: &mut clap::Command) {
    let help = cmd.render_help();
    log_info!(help.ansi());
}

#[derive(Debug, Clone, Copy)]
pub struct LogLevelValueParser;

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

    fn possible_values(&self) -> Option<Box<dyn Iterator<Item=PossibleValue> + '_>> {
        Some(Box::new(
            ["off", "error", "warn", "info", "debug", "trace"]
                .iter()
                .map(PossibleValue::new),
        ))
    }
}
