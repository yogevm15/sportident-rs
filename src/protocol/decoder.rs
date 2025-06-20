use std::marker::PhantomData;
use tokio_util::bytes::{Buf, BytesMut};
use tokio_util::codec::Decoder;

use crate::protocol;
use crate::protocol::{crc_slice, DecodableResponse};

const IGNORED_DATA_LENGTH: usize = 2;

pub struct ResponseDecoder<T> {
    waiting: bool,
    marker: PhantomData<T>,
}

impl<T> Default for ResponseDecoder<T> {
    fn default() -> Self {
        Self {
            waiting: true,
            marker: PhantomData,
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum DecoderError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Invalid start byte ({0:#04x})")]
    InvalidStartByte(u8),
    #[error("Length should be at least {IGNORED_DATA_LENGTH} bytes, but was {0}")]
    InvalidLength(usize),
    #[error("Invalid end byte ({0:#04x})")]
    InvalidEndByte(u8),
    #[error("Invalid checksum (expected: ({0:#06x}, found: ({1:#06x}))")]
    InvalidChecksum(u16, u16),
    #[error("Invalid response type (expected: {0:#04x}, found: {0:#04x})")]
    InvalidResponseType(u8, u8),
    #[error("Missing data, Length should be at least {0} bytes, but was {1}")]
    MissingData(usize, usize),
    #[error("Invalid card number: {0}")]
    InvalidCardNumber(u32),
    #[error("Not a valid calendar date (year: {0}, month: {1}, day: {2})")]
    InvalidDate(i32, u32, u32),
    #[error("Unknown model id received: {0}")]
    UnknownModelId(u16),
    #[error("Unknown SRR channel received: {0}")]
    UnknownSRRChannel(u8),
    #[error("Unknown station mode received: {0}")]
    UnknownStationMode(u8),
    #[error("Invalid punch time")]
    InvalidPunchTime,
    #[error("Invalid owner data")]
    InvalidOwnerData,
}

impl<T: DecodableResponse> Decoder for ResponseDecoder<T> {
    type Item = T;
    type Error = DecoderError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if self.waiting {
            loop {
                if src.is_empty() {
                    return Ok(None);
                }
                if src[0] == protocol::WAKEUP {
                    src.advance(1);
                    continue;
                }
                break;
            }

            let start = src[0];

            if start != protocol::START {
                return Err(DecoderError::InvalidStartByte(start));
            }
            src.advance(1);
            self.waiting = false;
        }

        if src.len() < 2 {
            return Ok(None);
        }

        let (cmd, length) = (src[0], src[1] as usize);

        if cmd != T::MAGIC {
            return Err(DecoderError::InvalidResponseType(T::MAGIC, cmd));
        }

        if length < IGNORED_DATA_LENGTH {
            self.waiting = true;
            return Err(DecoderError::InvalidLength(length));
        }

        if src.len() < 2 + length + 3 {
            // The response is not yet complete.
            //
            // We reserve more space in the buffer. This is not strictly
            // necessary but is a good idea performance-wise.
            src.reserve(2 + length + 3 - src.len());

            // We inform the Framed that we need more bytes to form the next
            // frame.
            return Ok(None);
        }
        let crc_calc = crc_slice(&src[..2 + length]);

        self.waiting = true;

        src.advance(2 + IGNORED_DATA_LENGTH);

        let data = &src[1..length - IGNORED_DATA_LENGTH];
        let res = T::decode(data);
        src.advance(length - IGNORED_DATA_LENGTH);

        let (crc_recv, end) = (u16::from_be_bytes(src[0..2].try_into().unwrap()), src[2]);

        src.advance(3);

        if end != protocol::END {
            return Err(DecoderError::InvalidEndByte(end));
        }

        if crc_calc != crc_recv {
            return Err(DecoderError::InvalidChecksum(crc_calc, crc_recv));
        }

        Ok(Some(res?))
    }
}
