use thiserror::Error;

pub type ProcessingPicturifyResult<T> = Result<T, ProcessingPicturifyError>;

#[derive(Debug, Error)]
pub enum ProcessingPicturifyError {
    #[error("Channel selection not supported")]
    ChannelSelectionNotSupported,
    #[error("Invalid channel selector")]
    InvalidChannelSelector,
    #[error("Invalid angle")]
    InvalidAngle,
    #[error("Invalid kernel")]
    InvalidKernel,
}
