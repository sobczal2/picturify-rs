use picturify_core::thiserror::Error;

pub type MoviePicturifyResult<T> = Result<T, MoviePicturifyError>;
#[derive(Error, Debug)]
pub enum MoviePicturifyError {
    #[error("Ffmpeg not found")]
    FfmpegNotFound,
    #[error("Ffprobe not found")]
    FfprobeNotFound,
    #[error("Failed to execute ffmpeg")]
    FfmpegFailed,
    #[error("Failed to execute ffprobe")]
    FfprobeFailed,
}