use crate::protocol::{Command, DecoderError, Response};

pub struct Beep;

pub struct BeepResponse;

impl Command for Beep {
    fn command_parameters(&self) -> Vec<u8> {
        vec![]
    }

    fn magic(&self) -> u8 {
        0x06
    }
}

impl Response for BeepResponse {
    fn decode(_data: &[u8]) -> Result<Self, DecoderError> {
        Ok(Self)
    }
}
