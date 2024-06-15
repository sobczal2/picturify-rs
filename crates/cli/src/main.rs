use std::process::{exit, ExitCode, Termination};
use std::time::Instant;

#[allow(unused_imports)]
use log::{error, info, warn, LevelFilter};
use log::debug;
use simplelog::*;

use crate::commands::common::command::Command;
use crate::commands::common::picturify::PicturifyCommand;
use crate::error::{CliPicturifyError, CliPicturifyResult};
use crate::handlers::common::handler::CommandHandler;
use crate::handlers::common::image::ImageCommandHandler;
use crate::handlers::common::movie::MovieCommandHandler;

mod commands;
mod common;
mod error;
mod handlers;
mod metadata;
mod progress;

fn main() -> ExitCode {
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();
    
    #[cfg(target_os = "windows")]
    warn!("You are using windows, please reconsider your life choices");

    let start = Instant::now();

    let matches = PicturifyCommand::get().get_matches();

    let result = match matches.subcommand() {
        Some(("image", args)) => ImageCommandHandler::handle(&ImageCommandHandler, args.clone()),
        Some(("movie", args)) => MovieCommandHandler::handle(&MovieCommandHandler, args.clone()),
        _ => {
            PicturifyCommand::get().print_help().unwrap();
            Err(CliPicturifyError::MissingCommand)
        }
    };
    
    match result {
        Ok(_) => {
            ExitCode::SUCCESS
        }
        Err(e) => {
            error!("{}", e);
            ExitCode::FAILURE
        }
    }
}
