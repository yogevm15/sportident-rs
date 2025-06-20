use crate::protocol::responses::card::Card;
use crate::protocol::{DecodableResponse, DecoderError, SubSecondPunch};

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone)]
pub struct CardPunch {
    pub punch: SubSecondPunch,
    pub card: Card,
}

impl DecodableResponse for CardPunch {
    const MAGIC: u8 = 0xd3;

    fn decode(data: &[u8]) -> Result<Self, DecoderError> {
        const CARD_PUNCH_LENGTH: usize = 7;

        if data.len() != CARD_PUNCH_LENGTH {
            return Err(DecoderError::MissingData(CARD_PUNCH_LENGTH, data.len()));
        }
        let card = Card::new(u32::from_be_bytes([0, data[0], data[1], data[2]]))?;
        let punch =
            SubSecondPunch::decode_punch(card.card_type, [data[3], data[6], data[4], data[5]])?
                .unwrap();
        Ok(Self { punch, card })
    }
}
