use std::process::ExitCode;

#[allow(unused_imports)]
use log::{error, info, warn, LevelFilter};
use simplelog::*;

use crate::commands::common::command::Command;
use crate::commands::common::picturify::PicturifyCommand;
use crate::error::CliPicturifyError;
use crate::handlers::common::handler::CommandHandler;
use crate::handlers::common::image::ImageCommandHandler;
use crate::handlers::common::movie::MovieCommandHandler;
use crate::handlers::common::picturify::PicturifyCommandHandler;

mod commands;
mod common;
mod error;
mod handlers;
mod metadata;
mod progress;

fn main() -> ExitCode {
    #[cfg(target_os = "windows")]
    warn!("You are using windows, please reconsider your life choices");

    let handler = PicturifyCommandHandler;
    
    let result = handler.handle(PicturifyCommand::get().get_matches());

    match result {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            error!("{}", e);
            ExitCode::FAILURE
        }
    }
}
