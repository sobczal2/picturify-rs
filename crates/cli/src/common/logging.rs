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
