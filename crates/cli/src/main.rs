use crate::commands::common::command::Command;
use crate::commands::common::picturify::PicturifyCommand;
use crate::error::CliPicturifyError;
use crate::handlers::common::handler::CommandHandler;
use crate::handlers::common::image::ImageCommandHandler;
use crate::handlers::common::movie::MovieCommandHandler;
#[allow(unused_imports)]
use log::{error, info, warn, LevelFilter};
use simplelog::*;
use std::time::Instant;

mod commands;
mod error;
mod handlers;
mod metadata;
mod progress;

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
        Some(("image", args)) => ImageCommandHandler::handle(args.clone()),
        Some(("movie", args)) => MovieCommandHandler::handle(args.clone()),
        _ => {
            PicturifyCommand::get().print_help().unwrap();
            Err(CliPicturifyError::MissingCommand)
        }
    };

    if let Err(e) = result {
        error!("{}", e);

        std::process::exit(1);
    }

    let duration = start.elapsed();
    match duration.as_secs() {
        0 => info!(
            "Damn, that was fast! Execution time: {}ms",
            duration.as_millis()
        ),
        _ => info!("Execution time: {}s", duration.as_secs()),
    }
}

#[cfg(not(target_os = "windows"))]
fn welcome() {
    info!("Welcome to Picturify CLI");
}

#[cfg(target_os = "windows")]
fn welcome() {
    warn!("Welcome to Picturify CLI. You are using Windows, please reconsider your life choices.");
}
