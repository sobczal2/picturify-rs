use std::env::args;
use std::process::ExitCode;

use itertools::Itertools;
use log::LevelFilter;
use picturify_core::logging::logger::PicturifyLogger;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

use crate::commands::common::command::Command;
use crate::commands::common::picturify::PicturifyCommand;
use crate::error::handle_clap_error;
use crate::handlers::common::handler::CommandHandler;
use crate::handlers::common::picturify::PicturifyCommandHandler;

mod commands;
mod common;
mod error;
mod handlers;
mod metadata;
mod progress;

const DEFAULT_VERBOSITY: LevelFilter = LevelFilter::Info;

fn main() -> ExitCode {
    setup_logger();

    #[cfg(target_os = "windows")]
    PicturifyLogger::log_war("You are using windows, please reconsider your life choices");

    let handler = PicturifyCommandHandler;

    let matches_result = PicturifyCommand::create().try_get_matches();

    match matches_result {
        Ok(matches) => {
            let result = handler.handle(matches);

            match result {
                Ok(_) => ExitCode::SUCCESS,
                Err(e) => {
                    PicturifyLogger::log_error(e);
                    ExitCode::FAILURE
                }
            }
        }
        Err(e) => handle_clap_error(e),
    }
}

fn setup_logger() {
    let verbosity = args()
        .skip(1)
        .tuple_windows()
        .find(|(arg, _)| arg == "--verbosity" || arg == "-v")
        .map(|(_, value)| value)
        .map(|value| value.parse::<LevelFilter>())
        .unwrap_or(Ok(DEFAULT_VERBOSITY))
        .unwrap_or(DEFAULT_VERBOSITY);

    TermLogger::init(
        verbosity,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .expect("Failed to initialize logger");
}
