use clap::error::ErrorKind;
use std::process::ExitCode;
use thiserror::Error;

use picturify_core::error::PicturifyError;
use picturify_core::{log_error, log_info};
use picturify_movie::error::MoviePicturifyError;

pub type CliPicturifyResult<T> = Result<T, CliPicturifyError>;

#[derive(Error, Debug)]
pub enum CliPicturifyError {
    #[error("Core library error: {0}")]
    Picturify(#[from] PicturifyError),
    #[error("Movie library error: {0}")]
    MoviePicturify(#[from] MoviePicturifyError),
    #[error("Threading error")]
    Threading,
    #[error("Command error: {0}")]
    Command(String),
}

impl CliPicturifyError {
    pub fn invalid_subcommand() -> Self {
        CliPicturifyError::Command("invalid subcommand".to_string())
    }

    pub fn missing_subcommand() -> Self {
        CliPicturifyError::Command("missing subcommand".to_string())
    }
}

pub fn handle_clap_error(error: clap::Error) -> ExitCode {
    let message = error.render();
    let kind = error.kind();
    match kind {
        ErrorKind::DisplayHelp
        | ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
        | ErrorKind::DisplayVersion => {
            log_info!(message.clone().ansi());
            ExitCode::SUCCESS
        }
        _ => {
            log_error!(message.clone().ansi());
            ExitCode::FAILURE
        }
    }
}
