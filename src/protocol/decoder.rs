use tokio_util::bytes::{Buf, BytesMut};
use tokio_util::codec::Decoder;

use crate::protocol;
use crate::protocol::card::{Card, CardRemoved};
use crate::protocol::{
    crc, Codec, ReadCardDataResponse, Response, SetMasterSlaveResponse, SystemConfiguration,
};

const IGNORED_DATA_LENGTH: usize = 2;

#[derive(thiserror::Error, Debug)]
pub enum DecoderError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Invalid command sent")]
    InvalidCommandSent,
    #[error("Invalid start byte ({0:#04x})")]
    InvalidStartByte(u8),
    #[error("Length should be at least {IGNORED_DATA_LENGTH} bytes, but was {0}")]
    InvalidLength(usize),
    #[error("Invalid end byte ({0:#04x})")]
    InvalidEndByte(u8),
    #[error("Invalid checksum (expected: ({0:#06x}, found: ({1:#06x}))")]
    InvalidChecksum(u16, u16),
    #[error("Invalid command ({0:#04x})")]
    InvalidCommand(u8),
    #[error("Received invalid system configuration, should be exactly {0} bytes but {1} bytes were given")]
    InvalidSystemConfiguration(usize, usize),
    #[error("Not a valid calendar date (year: {0}, month: {1}, day: {2})")]
    InvalidDate(i32, u32, u32),
    #[error("Unknown model id received: {0}")]
    UnknownModelId(u16),
    #[error("Unknown SRR channel received: {0}")]
    UnknownSRRChannel(u8),
    #[error("Unknown station mode received: {0}")]
    UnknownStationMode(u8),
    #[error("Received invalid card inserted response, should be exactly {0} bytes but {1} bytes were given")]
    InvalidCardInsertedLength(usize, usize),
    #[error("Invalid card number: {0}")]
    InvalidCardNumber(u32),
    #[error("Received invalid block size, should be exactly {0} bytes but {1} bytes were given")]
    InvalidBlockSize(usize, usize),
    #[error("Received invalid readout data length")]
    InvalidReadoutDataLength,
    #[error("Invalid punch time")]
    InvalidPunchTime,
    #[error("Invalid owner data")]
    InvalidOwnerData,
}

pub enum Responses {
    SystemConfiguration(SystemConfiguration),
    SetMasterSlaveResponse(SetMasterSlaveResponse),
    CardInserted(Card),
    CardRemoved(CardRemoved),
    CardData(ReadCardDataResponse),
}

impl TryFrom<(u8, Vec<u8>)> for Responses {
    type Error = DecoderError;

    fn try_from((cmd, data): (u8, Vec<u8>)) -> Result<Self, Self::Error> {
        Ok(match cmd {
            0x83 => Self::SystemConfiguration(SystemConfiguration::decode(&data)?),
            0xf0 => Self::SetMasterSlaveResponse(SetMasterSlaveResponse::decode(&data)?),
            0xe7 => Self::CardRemoved(CardRemoved::decode(&data)?),
            0xe8 => Self::CardInserted(Card::decode(&data)?),
            0xef => Self::CardData(ReadCardDataResponse::decode(&data)?),
            _ => return Err(DecoderError::InvalidCommand(cmd)),
        })
    }
}

impl Decoder for Codec {
    type Item = Responses;
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
            if start == protocol::NOT_ACK {
                return Err(DecoderError::InvalidCommandSent);
            }

            if start != protocol::START {
                return Err(DecoderError::InvalidStartByte(start));
            }
            src.advance(1);
            self.waiting = false;
        }

        let mut cmd_and_length = [0; 2];
        cmd_and_length.copy_from_slice(&src[..2]);

        let (cmd, length) = (cmd_and_length[0], cmd_and_length[1] as usize);

        if length < IGNORED_DATA_LENGTH {
            return Err(DecoderError::InvalidLength(length));
        }

        if src.len() < 2 + length + 3 {
            // The response is not yet complete.
            //
            // We reserve more space in the buffer. This is not strictly
            // necessary, but is a good idea performance-wise.
            src.reserve(2 + length + 3 - src.len());

            // We inform the Framed that we need more bytes to form the next
            // frame.
            return Ok(None);
        }
        src.advance(2);

        let mut ignored = [0; IGNORED_DATA_LENGTH];

        ignored.copy_from_slice(&src[..IGNORED_DATA_LENGTH]);
        src.advance(IGNORED_DATA_LENGTH);

        let data = src[..length - IGNORED_DATA_LENGTH].to_vec();
        src.advance(length - IGNORED_DATA_LENGTH);

        let mut crc_and_end = [0; 3];

        crc_and_end.copy_from_slice(&src[..3]);
        src.advance(3);

        self.waiting = true;

        let (crc_recv, end) = (
            u16::from_be_bytes(crc_and_end[0..2].try_into().unwrap()),
            crc_and_end[2],
        );

        if end != protocol::END {
            return Err(DecoderError::InvalidEndByte(end));
        }

        let mut check = Vec::with_capacity(2 + length);
        check.extend_from_slice(cmd_and_length.as_slice());
        check.extend_from_slice(ignored.as_slice());
        check.extend_from_slice(data.as_slice());
        let crc_calc = crc(check.as_slice());
        if crc_calc != crc_recv {
            return Err(DecoderError::InvalidChecksum(crc_calc, crc_recv));
        }

        Ok(Some((cmd, data).try_into()?))
    }
}
