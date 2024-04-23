pub use commands::*;
pub use constants::*;
pub use crc::*;
pub use decoder::*;
pub use encoder::*;
pub use responses::*;
pub use readout::*;
mod commands;
mod constants;
mod crc;
mod decoder;
mod encoder;
mod responses;
mod readout;

pub struct Codec {
    waiting: bool,
}

impl Default for Codec {
    fn default() -> Self {
        Self { waiting: true }
    }
}
