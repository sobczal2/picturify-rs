use thiserror::Error;
use crate::error::processing::ProcessingPicturifyError;

pub type PipelinePicturifyResult<T> = Result<T, PipelinePicturifyError>;

#[derive(Debug, Error)]
pub enum PipelinePicturifyError {
    #[error("Processing error: {0}")]
    ProcessingError(#[from] ProcessingPicturifyError),
}
