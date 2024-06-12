use crate::protocol::card::CardType;
use crate::protocol::DecoderError;
pub use readout::*;
pub use owner_data::*;



pub trait FromCardBlocks: Sized + Send{
    const INCLUDE_OWNER_DATA_BLOCKS: bool;
    fn from_card_blocks(data: &[u8], card_type: CardType) -> Result<Self, DecoderError>;
}

pub mod readout;
pub mod owner_data;
mod punch;
