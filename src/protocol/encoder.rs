use tokio_util::bytes::{BufMut, BytesMut};
use tokio_util::codec::Encoder;

use crate::protocol;
use crate::protocol::{crc_iter, Command};

pub struct CommandEncoder;

#[derive(thiserror::Error, Debug)]
pub enum EncoderError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Encoded command is too long, maximum length is {0} bytes, but {1} bytes were given")]
    EncodedCommandTooLong(usize, usize),
}

impl<C: Command> Encoder<C> for CommandEncoder {
    type Error = EncoderError;

    fn encode(&mut self, command: C, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let parameters = command.encode();
        let len = u8::try_from(parameters.len())
            .map_err(|_| EncoderError::EncodedCommandTooLong(u8::MAX as usize, parameters.len()))?;
        dst.put_u8(protocol::WAKEUP);
        dst.put_u8(protocol::START);
        dst.put_u8(C::MAGIC);
        dst.put_u8(len);
        dst.put_slice(parameters);
        dst.put_u16(crc_iter(
            [C::MAGIC, len]
                .into_iter()
                .chain(parameters.iter().copied()),
        ));
        dst.put_u8(protocol::END);

        Ok(())
    }
}
