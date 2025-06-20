use crate::protocol::commands::WithResponse;
use crate::protocol::{Command, DecodableResponse, DecoderError};
use std::slice;

pub const BLOCK_SIZE: usize = 128;

pub struct ReadCardData {
    block: u8,
}

impl ReadCardData {
    pub const fn new(block: u8) -> Self {
        Self { block }
    }
}

#[derive(Debug)]
pub struct ReadCardDataResponse(Vec<u8>);

impl From<ReadCardDataResponse> for Vec<u8> {
    fn from(value: ReadCardDataResponse) -> Self {
        value.0
    }
}

impl Command for ReadCardData {
    const MAGIC: u8 = 0xef;
    type Response = WithResponse<ReadCardDataResponse>;

    fn encode(&self) -> &[u8] {
        slice::from_ref(&self.block)
    }
}

impl DecodableResponse for ReadCardDataResponse {
    const MAGIC: u8 = 0xef;

    fn decode(data: &[u8]) -> Result<Self, DecoderError> {
        if data.len() != BLOCK_SIZE {
            return Err(DecoderError::MissingData(BLOCK_SIZE, data.len()));
        }
        Ok(Self(data.to_vec()))
    }
}
