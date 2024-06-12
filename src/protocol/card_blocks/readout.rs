use crate::protocol::card::CardType;
use crate::protocol::card_blocks::punch::{Punch, StartOrFinishPunch};
use crate::protocol::card_blocks::FromCardBlocks;
use crate::protocol::DecoderError;

#[derive(Debug)]
pub struct CardReadout {
    pub card_number: u32,
    pub card_type: CardType,
    pub start: Option<StartOrFinishPunch>,
    pub finish: Option<StartOrFinishPunch>,
    pub check: Option<Punch>,
    pub punches: Vec<Punch>,
}

impl FromCardBlocks for CardReadout {
    const INCLUDE_OWNER_DATA_BLOCKS: bool = false;

    fn from_card_blocks(data: &[u8], card_type: CardType) -> Result<Self, DecoderError> {
        match card_type {
            CardType::Si8
            | CardType::Si9
            | CardType::Si10
            | CardType::Si11
            | CardType::Siac
            | CardType::PunchCard => {
                if data.len() < 28 {
                    return Err(DecoderError::InvalidReadoutDataLength);
                }
                let punch_count = data[22];
                let punches_start_offset = usize::from(Self::punches_offset(card_type));
                let punches_end_offset = punches_start_offset + usize::from(punch_count) * 4;
                if data.len() < punches_end_offset {
                    return Err(DecoderError::InvalidReadoutDataLength);
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
