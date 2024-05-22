use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProcessingError {
    #[error("Channel selection not supported")]
    ChannelSelectionNotSupported,
    #[error("Invalid channel selector")]
    InvalidChannelSelector,
    #[error("Invalid angle")]
    InvalidAngle,
}
