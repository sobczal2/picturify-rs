use clap::error::ErrorKind;
use thiserror::Error;

use picturify_core::error::PicturifyError;
use picturify_movie::error::MoviePicturifyError;

pub type CliPicturifyResult<T> = Result<T, CliPicturifyError>;

#[derive(Error, Debug)]
pub enum CliPicturifyError {
    #[error("Cli {0}")]
    Picturify(#[from] PicturifyError),
    #[error("Cli {0}")]
    MoviePicturify(#[from] MoviePicturifyError),
    #[error("Threading error")]
    Threading,
    #[error("Cli {0}")]
    Clap(#[from] clap::Error),
}

impl CliPicturifyError {
    pub fn invalid_subcommand() -> Self {
        CliPicturifyError::Clap(clap::Error::raw(ErrorKind::InvalidSubcommand, "invalid subcommand"))
    }
    
    pub fn missing_subcommand() -> Self {
        CliPicturifyError::Clap(clap::Error::raw(ErrorKind::MissingSubcommand, "missing subcommand"))
    }
}