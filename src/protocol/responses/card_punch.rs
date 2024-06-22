use crate::protocol::responses::card::Card;
use crate::protocol::{DecoderError, Response, SubSecondPunch};

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone)]
pub struct CardPunch {
    pub punch: SubSecondPunch,
    pub card: Card,
}

impl Response for CardPunch {
    fn decode(data: &[u8]) -> Result<Self, DecoderError> {
        let card = Card::decode(&data[0..4])?;
        let punch =
            SubSecondPunch::decode_punch(card.card_type, [data[4], data[7], data[5], data[6]])?
                .unwrap();
        Ok(Self { punch, card })
    }
}
