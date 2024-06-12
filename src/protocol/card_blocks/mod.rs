use crate::protocol::card::CardType;
use crate::protocol::DecoderError;
pub use owner_data::*;
pub use readout::*;

pub trait FromCardBlocks: Sized + Send {
    const INCLUDE_OWNER_DATA_BLOCKS: bool;
    fn from_card_blocks(data: &[u8], card_type: CardType) -> Result<Self, DecoderError>;
}

pub mod owner_data;
mod punch;
pub mod readout;
