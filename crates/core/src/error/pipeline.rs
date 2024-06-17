use crate::error::processing::ProcessingPicturifyError;
use thiserror::Error;

pub type PipelinePicturifyResult<T> = Result<T, PipelinePicturifyError>;

#[derive(Debug, Error)]
pub enum PipelinePicturifyError {
    #[error("Processing error: {0}")]
    ProcessingError(#[from] ProcessingPicturifyError),
}
