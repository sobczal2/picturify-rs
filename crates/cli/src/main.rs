use std::env::args;
use crate::commands::common::command::Command;
use crate::commands::common::picturify::PicturifyCommand;
use crate::handlers::common::handler::CommandHandler;
use crate::handlers::common::picturify::PicturifyCommandHandler;
use log::{error, LevelFilter};
use std::process::ExitCode;
use itertools::Itertools;
use simplelog::{ColorChoice, Config, TerminalMode, TermLogger};
use crate::error::CliPicturifyError;

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
    warn!("You are using windows, please reconsider your life choices");

    let handler = PicturifyCommandHandler;

    let matches_result = PicturifyCommand::create().try_get_matches();

    match matches_result {
        Ok(matches) => {
            let result = handler.handle(matches);

            match result {
                Ok(_) => ExitCode::SUCCESS,
                Err(e) => {
                    error!("{}", e);
                    ExitCode::FAILURE
                }
            }
        }
        Err(e) => {
            let error: CliPicturifyError = e.into();
            error!("{}", error);
            ExitCode::FAILURE
        }
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