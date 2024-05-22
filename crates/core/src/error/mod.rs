pub mod processing;

use thiserror::Error;

pub type PicturifyResult<T> = Result<T, PicturifyError>;

#[derive(Error, Debug)]
pub enum PicturifyError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Image error: {0}")]
    ImageError(#[from] image::ImageError),
    #[error("Invalid image format")]
    InvalidImageFormat,
    #[error("Processing error: {0}")]
    ProcessingError(#[from] processing::ProcessingError),
    #[error("Parse error: {0}")]
    ParseError(String),
}
