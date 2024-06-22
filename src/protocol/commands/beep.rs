use crate::protocol::Command;

pub struct Beep;

impl Command for Beep {
    fn command_parameters(&self) -> Vec<u8> {
        vec![]
    }

    fn magic(&self) -> u8 {
        0x06
    }
}
