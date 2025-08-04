use thiserror::Error;

#[derive(Debug, Error)]
pub enum MidiError {
    #[error("Invalid Channel: {0}. It should be from 0 to 15.")]
    InvalidChannel(u8),

    #[error("Invalid Event value: {0}. It should be from 0 to 6.")]
    InvalidEvent(u8),

    #[error("Invalid System Common Event value: {0}.")]
    InvalidSystemCommonEvent(u8),

    #[error("Invalid Real Time Message value: {0}.")]
    InvalidRealTimeMessage(u8),

    #[error("Not a status byte, MSB should be 1")]
    InvalidStatusByte,

    #[error("Not a valid Header")]
    InvalidHeaderByte,

    #[error("Not a valid MidiFormat")]
    InvalidFormat, 

    #[error("Not a valid VQL input: {0}")]
    InvalidVqlInput(u32)
}
