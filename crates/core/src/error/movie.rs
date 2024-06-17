use crate::error::pipeline::PipelinePicturifyError;
use thiserror::Error;

pub type MoviePicturifyResult<T> = Result<T, MoviePicturifyError>;
#[derive(Debug, Error)]
pub enum MoviePicturifyError {
    #[error("Ffmpeg not found")]
    FfmpegNotFound,
    #[error("Ffprobe not found")]
    FfprobeNotFound,
    #[error("Failed to execute ffmpeg")]
    FfmpegFailed,
    #[error("Failed to execute ffprobe")]
    FfprobeFailed,
    #[error("Pipeline error: {0}")]
    PipelineError(#[from] PipelinePicturifyError),
}
