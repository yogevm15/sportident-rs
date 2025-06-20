use crate::protocol::commands::NoResponse;
use crate::protocol::Command;

pub struct Beep;

impl Command for Beep {
    const MAGIC: u8 = 0x06;
    type Response = NoResponse;

    fn encode(&self) -> &[u8] {
        &[]
    }
}
