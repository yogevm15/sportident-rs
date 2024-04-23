use crate::protocol::commands::Response;
use crate::protocol::decoder::DecoderError;
use crate::protocol::Command;

#[derive(Copy, Clone, Debug)]
#[allow(dead_code)]
pub enum SetMasterSlave {
    Master = 0x4d,
    Slave = 0x53,
}

pub struct SetMasterSlaveResponse;

impl Command for SetMasterSlave {
    fn command_parameters(&self) -> Vec<u8> {
        vec![*self as u8]
    }

    fn magic(&self) -> u8 {
        0xf0
    }
}

impl Response for SetMasterSlaveResponse {
    fn decode(_data: &[u8]) -> Result<Self, DecoderError> {
        Ok(Self)
    }
}
