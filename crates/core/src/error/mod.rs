pub mod processing;

use thiserror::Error;

pub type PicturifyResult<T> = Result<T, PicturifyError>;

#[derive(Error, Debug)]
pub enum PicturifyError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Image error: {0}")]
    ImageError(#[from] image::ImageError),
    #[error("Invalid fast_image format")]
    InvalidImageFormat,
    #[error("Processing error: {0}")]
    ProcessingError(#[from] processing::ProcessingError),
}
