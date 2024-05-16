use std::time::Instant;
use crate::commands::common::picturify::PicturifyCommand;
use crate::handlers::common::image::ImageCommandHandler;
use log::{error, info, LevelFilter};
use simplelog::*;
use crate::commands::common::command::Command;
use crate::error::CliPicturifyError;

mod commands;
mod handlers;
mod metadata;
mod progress;
mod error;

fn main() {
    #[cfg(debug_assertions)]
    TermLogger::init(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
        .unwrap();

    #[cfg(not(debug_assertions))]
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
        .unwrap();

    welcome();
    let start = Instant::now();

    let matches = PicturifyCommand::get().get_matches();

    let result = match matches.subcommand() {
        Some(("image", arg)) => {
            ImageCommandHandler::handle(arg.clone())
        }
        Some(("movie", _)) => {
            info!("Movie command not implemented yet");
            Err(CliPicturifyError::InvalidCommand("movie".to_string()))
        }
        _ => {
            error!("No command specified");
            Err(CliPicturifyError::MissingCommand)
        }
    };
    
    if let Err(e) = result {
        error!("{}", e);
    }
    
    let duration = start.elapsed();
    match duration.as_secs() {
        0 => info!("Damn, that was fast! Execution time: {}ms", duration.as_millis()),
        _ => info!("Execution time: {}s", duration.as_secs()),
    }
}

#[cfg(not(target_os = "windows"))]
fn welcome() {
    info!("Welcome to Picturify CLI");
}

#[cfg(target_os = "windows")]
fn welcome() {
    info!("Welcome to Picturify CLI. Consider switching to a Unix-based system for sanity reasons");
}
