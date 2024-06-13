use picturify_core::error::PicturifyError;
use picturify_movie::error::MoviePicturifyError;
use thiserror::Error;

pub type CliPicturifyResult<T> = Result<T, CliPicturifyError>;

#[derive(Error, Debug)]
pub enum CliPicturifyError {
    #[error("Picturify error - {0}")]
    PicturifyError(#[from] PicturifyError),
    #[error("Missing argument: {0}")]
    MissingArgument(String),
    #[error("Invalid command: {0}")]
    InvalidCommand(String),
    #[error("Missing command")]
    MissingCommand,
    #[error("Missing subcommand")]
    MissingSubcommand,
    #[error("Movie Picturify error - {0}")]
    MoviePicturifyError(#[from] MoviePicturifyError),
}
