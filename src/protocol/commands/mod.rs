pub use beep::*;
pub use read_card_data::*;
pub use set_master_slave::*;
pub use system_configuration::*;

use crate::protocol::decoder::DecoderError;
use crate::protocol::EncoderError;

mod beep;
mod read_card_data;
mod set_master_slave;
mod system_configuration;

#[enum_dispatch::enum_dispatch]
pub trait Command {
    fn command_parameters(&self) -> Vec<u8>;
    fn magic(&self) -> u8;

    fn encode(&self) -> Result<Vec<u8>, EncoderError> {
        let mut bytes = vec![self.magic()];
        let parameters = self.command_parameters();
        bytes.push(u8::try_from(parameters.len()).map_err(|_| {
            EncoderError::CommandParametersTooLong(u8::MAX as usize, parameters.len())
        })?);
        bytes.extend(parameters);
        Ok(bytes)
    }
}

pub trait Response: Sized {
    fn decode(data: &[u8]) -> Result<Self, DecoderError>;
}
