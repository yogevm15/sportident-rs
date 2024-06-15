#![allow(clippy::pedantic)]

use crate::protocol::{CardBlocks, BLOCK_SIZE};

mod pcard;
mod si10;
mod si8;
mod siac;

impl CardBlocks for Vec<u8> {
    async fn get_block(&mut self, index: u8) -> crate::Result<&[u8; BLOCK_SIZE]> {
        if self.len() < BLOCK_SIZE * (index as usize + 1) {
            panic!("missing block!")
        }
        Ok(
            (&self[BLOCK_SIZE * index as usize..BLOCK_SIZE * (index as usize + 1)])
                .try_into()
                .expect("unreachable"),
        )
    }
}
