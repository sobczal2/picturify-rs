use thiserror::Error;

pub type ProcessingResult<T> = Result<T, ProcessingError>;

#[derive(Debug, Error)]
pub enum ProcessingError {
    #[error("Channel selection not supported")]
    ChannelSelectionNotSupported,
    #[error("Invalid channel selector")]
    InvalidChannelSelector,
    #[error("Invalid angle")]
    InvalidAngle,
    #[error("Invalid kernel")]
    InvalidKernel,
}
