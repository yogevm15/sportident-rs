pub use card_blocks::*;
pub use commands::*;
pub use constants::*;
pub use crc::*;
pub use decoder::*;
pub use encoder::*;
pub use punch::*;
mod card_blocks;
mod commands;
mod constants;
mod crc;
mod decoder;
mod encoder;
mod punch;
pub mod responses;

pub struct Codec {
    waiting: bool,
}

impl Default for Codec {
    fn default() -> Self {
        Self { waiting: true }
    }
}
