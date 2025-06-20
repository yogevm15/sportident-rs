use crate::protocol::commands::WithResponse;
use crate::protocol::{Command, DecodableResponse};
use crate::DecoderError;
use std::slice;

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
#[allow(dead_code)]
pub enum SetMasterSlave {
    Master = 0x4d,
    Slave = 0x53,
}

#[derive(Debug)]
pub struct SetMasterSlaveResponse;

impl Command for SetMasterSlave {
    const MAGIC: u8 = 0xf0;
    type Response = WithResponse<SetMasterSlaveResponse>;

    fn encode(&self) -> &[u8] {
        // SAFETY: `SetMasterSlave` is repr u8.
        slice::from_ref(unsafe { &*std::ptr::from_ref::<Self>(self).cast::<u8>() })
    }
}

impl DecodableResponse for SetMasterSlaveResponse {
    const MAGIC: u8 = 0xf0;

    fn decode(_data: &[u8]) -> Result<Self, DecoderError> {
        Ok(Self)
    }
}
