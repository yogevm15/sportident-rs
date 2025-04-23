use crate::protocol::{DecodableResponse, DecoderError};
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub enum CardType {
    Si8,
    Si9,
    Si10,
    Si11,
    Siac,
    PunchCard,
}

impl CardType {
    const fn new(number: u32) -> Result<Self, DecoderError> {
        Ok(match number {
            2_000_000..=2_999_999 => Self::Si8,
            1_000_000..=1_999_999 => Self::Si9,
            7_000_000..=7_999_999 => Self::Si10,
            9_000_000..=9_999_999 => Self::Si11,
            8_000_000..=8_999_999 => Self::Siac,
            4_000_000..=4_999_999 => Self::PunchCard,
            _ => return Err(DecoderError::InvalidCardNumber(number)),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone)]
pub struct Card {
    pub card_type: CardType,
    pub number: u32,
}

impl Card {
    pub fn new(number: u32) -> Result<Self, DecoderError> {
        let card_type = CardType::new(number)?;

        Ok(Self { card_type, number })
    }
}

pub struct CardInserted(pub Card);

impl Deref for CardInserted {
    type Target = Card;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DecodableResponse for CardInserted {
    const MAGIC: u8 = 0xe8;

    fn decode(data: &[u8]) -> Result<Self, DecoderError> {
        const CARD_INSERTED_LENGTH: usize = 3;
        if data.len() != CARD_INSERTED_LENGTH {
            return Err(DecoderError::MissingData(CARD_INSERTED_LENGTH, data.len()));
        }
        Ok(Self(Card::new(u32::from_be_bytes([
            0, data[0], data[1], data[2],
        ]))?))
    }
}

#[derive(Debug)]
pub struct CardRemoved;
impl DecodableResponse for CardRemoved {
    const MAGIC: u8 = 0xe7;

    fn decode(_data: &[u8]) -> Result<Self, DecoderError> {
        Ok(Self)
    }
}
