use crate::protocol::{DecoderError, EncoderError};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Serial port error: {0}")]
    SerialPortError(#[from] tokio_serial::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Decoder error: {0}")]
    DecoderError(#[from] DecoderError),
    #[error("Encoder error: {0}")]
    EncoderError(#[from] EncoderError),
    #[error("Invalid response received")]
    InvalidResponseReceived,
    #[error("Input buffer is not empty ({0} bytes to read)")]
    InputIsNotEmpty(u32),
    #[error("Received invalid command (expected: {0:#04x}, found: {1:#04x})")]
    ReceivedInvalidCommand(u8, u8),
    #[error(
        "This feature only supports stations in \"Extended Protocol\" mode. Switch mode first"
    )]
    NotExtendedProtocolMode,
    #[error("Station must be in 'Read SI cards' operating mode! Change operating mode first.")]
    NotReadoutMode,
    #[error("Port closed")]
    PortClosed,
    #[error("Card removed while reading data")]
    CardRemovedWhileReadingData,
    #[error("This feature only supports stations in \"Auto Send\" mode. Switch mode first")]
    NotAutoSendMode,
    #[error("No reader detected")]
    NoReaderDetected,
}

pub type Result<T> = std::result::Result<T, Error>;
