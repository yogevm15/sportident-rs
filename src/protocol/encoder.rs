use tokio_util::bytes::BytesMut;
use tokio_util::codec::Encoder;

use crate::protocol;
use crate::protocol::{
    crc, Beep, Codec, Command, GetSystemConfiguration, ReadCardData, SetMasterSlave,
};

#[derive(thiserror::Error, Debug)]
pub enum EncoderError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error(
        "Command parameters are too long, maximum length is {0} bytes, but {1} bytes were given"
    )]
    CommandParametersTooLong(usize, usize),
}

#[enum_dispatch::enum_dispatch(Command)]
pub enum Commands {
    SetMasterSlave(SetMasterSlave),
    GetSystemConfiguration(GetSystemConfiguration),
    Beep(Beep),
    ReadCardData(ReadCardData),
}
impl Encoder<Commands> for Codec {
    type Error = EncoderError;

    fn encode(&mut self, command: Commands, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let command_bytes = command.encode()?;

        dst.extend_from_slice(&[protocol::WAKEUP, protocol::START]);
        dst.extend_from_slice(command_bytes.as_slice());
        dst.extend_from_slice(crc(command_bytes.as_slice()).to_be_bytes().as_slice());
        dst.extend_from_slice(&[protocol::END]);

        Ok(())
    }
}
