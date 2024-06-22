use crate::protocol::card_blocks::FromCardBlocks;
use crate::protocol::responses::card::CardType;
use crate::protocol::{CardBlocks, DecoderError, Punch, StartOrFinishPunch, BLOCK_SIZE};

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone)]
pub struct CardReadout {
    pub card_number: u32,
    pub card_type: CardType,
    pub start: Option<StartOrFinishPunch>,
    pub finish: Option<StartOrFinishPunch>,
    pub check: Option<Punch>,
    pub punches: Vec<Punch>,
}

impl FromCardBlocks for CardReadout {
    async fn from_card_blocks(
        blocks: &mut impl CardBlocks,
        card_type: CardType,
    ) -> crate::Result<Self> {
        let logic = |data: &[u8]| {
            let punch_count = data[22];
            let punches_start_offset = usize::from(Self::punches_offset(card_type));
            let punches_end_offset = punches_start_offset + usize::from(punch_count) * 4;
            if data.len() < punches_end_offset {
                return Err(DecoderError::InvalidReadoutDataLength.into());
            }
            let punches = data[punches_start_offset..punches_end_offset]
                .chunks_exact(4)
                .filter_map(|punch_data| {
                    Punch::decode_punch(card_type, unsafe {
                        punch_data.try_into().unwrap_unchecked()
                    })
                    .transpose()
                })
                .collect::<Result<_, _>>()?;
            Ok(Self {
                start: StartOrFinishPunch::decode_punch(
                    card_type,
                    [data[12], data[13], data[14], data[15]],
                )?,
                finish: StartOrFinishPunch::decode_punch(
                    card_type,
                    [data[16], data[17], data[18], data[19]],
                )?,
                check: Punch::decode_punch(card_type, [data[8], data[9], data[10], data[11]])?,
                punches,
                card_number: u32::from_be_bytes([0, data[25], data[26], data[27]]),
                card_type,
            })
        };

        match card_type {
            CardType::Si8 | CardType::Si9 | CardType::PunchCard => {
                let mut data: [u8; BLOCK_SIZE * 2] = [0; BLOCK_SIZE * 2];
                data[0..BLOCK_SIZE].copy_from_slice(blocks.get_block(0).await?);
                data[BLOCK_SIZE..BLOCK_SIZE * 2].copy_from_slice(blocks.get_block(1).await?);
                logic(data.as_slice())
            }
            CardType::Si10 | CardType::Si11 | CardType::Siac => {
                let mut data: [u8; BLOCK_SIZE * 5] = [0; BLOCK_SIZE * 5];

                for i in 0..5u8 {
                    data[i as usize * BLOCK_SIZE..(i as usize + 1) * BLOCK_SIZE]
                        .copy_from_slice(blocks.get_block(i + 3).await?);
                }
                logic(data.as_slice())
            }
        }
    }
}
impl CardReadout {
    const fn punches_offset(card_type: CardType) -> u8 {
        match card_type {
            CardType::Si8 => 136,
            CardType::Si9 => 56,
            CardType::PunchCard => 176,
            CardType::Si10 | CardType::Si11 | CardType::Siac => 128,
        }
    }
}
