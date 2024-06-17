use clap::error::ErrorKind;
use std::process::ExitCode;
use thiserror::Error;

use picturify_core::error::PicturifyError;
use picturify_core::{log_error, log_info};
use picturify_core::error::movie::MoviePicturifyError;
use picturify_core::error::pipeline::PipelinePicturifyError;

pub type CliPicturifyResult<T> = Result<T, CliPicturifyError>;

#[derive(Error, Debug)]
pub enum CliPicturifyError {
    #[error("Core library error: {0}")]
    Picturify(#[from] PicturifyError),
    #[error("Movie library error: {0}")]
    MoviePicturify(#[from] MoviePicturifyError),
    #[error("Pipeline picturify error: {0}")]
    PipelinePicturify(#[from] PipelinePicturifyError),
    #[error("Threading error")]
    Threading,
    #[error("Command error: {0}")]
    Command(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

pub trait MapToCliPicturifyResult<T> {
    fn map_to_unknown_error(self) -> CliPicturifyResult<T>;
}

impl<T, E> MapToCliPicturifyResult<T> for Result<T, E>
where
    E: std::fmt::Display,
{
    fn map_to_unknown_error(self) -> CliPicturifyResult<T> {
        self.map_err(|e| CliPicturifyError::Internal(e.to_string()))
    }
}

impl<T> MapToCliPicturifyResult<T> for Option<T> {
    fn map_to_unknown_error(self) -> CliPicturifyResult<T> {
        self.ok_or(CliPicturifyError::Internal("Unknown error".to_string()))
    }
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
