use crate::protocol::{Command, DecoderError, Response};
use std::ops::{Deref, DerefMut};

pub struct ReadCardData {
    block: u8,
}

impl ReadCardData {
    pub const fn new(block: u8) -> Self {
        Self { block }
    }
}

pub const BLOCK_SIZE: usize = 128;

pub struct ReadCardDataResponse([u8; BLOCK_SIZE]);

impl Deref for ReadCardDataResponse {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ReadCardDataResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Command for ReadCardData {
    fn command_parameters(&self) -> Vec<u8> {
        self.block.to_be_bytes().to_vec()
    }

    fn magic(&self) -> u8 {
        0xef
    }
}

impl Response for ReadCardDataResponse {
    fn decode(data: &[u8]) -> Result<Self, DecoderError> {
        if data.is_empty() {
            return Err(DecoderError::InvalidBlockSize(BLOCK_SIZE, 0));
        }
        let data = &data[1..];
        Ok(Self(data.try_into().map_err(|_| {
            DecoderError::InvalidBlockSize(BLOCK_SIZE, data.len())
        })?))
    }
}
