pub use owner_data::*;
pub use readout::*;
use std::future::Future;

use crate::protocol::responses::card::CardType;
use crate::protocol::BLOCK_SIZE;
use crate::Result;

pub trait FromCardBlocks: Sized + Send {
    fn from_card_blocks(
        blocks: &mut impl CardBlocks,
        card_type: CardType,
    ) -> impl Future<Output = Result<Self>> + Send;
}

pub trait CardBlocks: Sized + Send {
    fn get_block(&mut self, index: u8) -> impl Future<Output = Result<&[u8; BLOCK_SIZE]>> + Send;
}

impl<U: FromCardBlocks, T: FromCardBlocks> FromCardBlocks for (U, T) {
    async fn from_card_blocks(blocks: &mut impl CardBlocks, card_type: CardType) -> Result<Self> {
        let u = U::from_card_blocks(blocks, card_type).await?;
        let t = T::from_card_blocks(blocks, card_type).await?;
        Ok((u, t))
    }
}

pub mod owner_data;
pub mod readout;

#[cfg(test)]
mod tests;
