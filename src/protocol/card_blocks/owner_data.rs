use std::str::from_utf8;

use crate::protocol::responses::card::CardType;
use crate::protocol::{CardBlocks, DecoderError, FromCardBlocks, BLOCK_SIZE};

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone)]
pub struct CardOwnerData {
    pub first_name: String,
    pub last_name: String,
    pub gender: Option<String>,
    pub birthday: Option<String>,
    pub club: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub city: Option<String>,
    pub street: Option<String>,
    pub zip: Option<String>,
    pub country: Option<String>,
}

impl CardOwnerData {
    fn parse_parts(amount: usize, data: &[u8]) -> Result<Vec<String>, DecoderError> {
        let parts = data
            .splitn(amount + 1, |c| *c == b';')
            .take(amount)
            .map(|part| from_utf8(part).map(ToString::to_string))
            .collect::<Result<Vec<String>, _>>()
            .map_err(|_| DecoderError::InvalidOwnerData)?;

        if parts.len() != amount {
            return Err(DecoderError::InvalidOwnerData);
        }

        Ok(parts)
    }
}

impl FromCardBlocks for CardOwnerData {
    async fn from_card_blocks(
        blocks: &mut impl CardBlocks,
        card_type: CardType,
    ) -> crate::Result<Self> {
        let mut data: [u8; BLOCK_SIZE * 2] = [0; BLOCK_SIZE * 2];
        data[0..BLOCK_SIZE].copy_from_slice(blocks.get_block(0).await?);
        data[BLOCK_SIZE..BLOCK_SIZE * 2].copy_from_slice(blocks.get_block(1).await?);

        let parts = match card_type {
            CardType::PunchCard | CardType::Si10 | CardType::Si11 | CardType::Siac => {
                let owner_data_slice = &data[32..160];

                Self::parse_parts(11, owner_data_slice)
            }

            CardType::Si9 | CardType::Si8 => {
                let owner_data_slice = &data[32..if card_type == CardType::Si9 { 56 } else { 136 }];
                Self::parse_parts(2, owner_data_slice)
            }
        }?;

        let first_name = parts[0].to_string();
        let last_name = parts[1].to_string();
        let gender = parts
            .get(2)
            .filter(|s| !s.is_empty())
            .map(ToString::to_string);
        let birthday = parts
            .get(3)
            .filter(|s| !s.is_empty())
            .map(ToString::to_string);
        let club = parts
            .get(4)
            .filter(|s| !s.is_empty())
            .map(ToString::to_string);
        let email = parts
            .get(5)
            .filter(|s| !s.is_empty())
            .map(ToString::to_string);
        let phone = parts
            .get(6)
            .filter(|s| !s.is_empty())
            .map(ToString::to_string);
        let city = parts
            .get(7)
            .filter(|s| !s.is_empty())
            .map(ToString::to_string);
        let street = parts
            .get(8)
            .filter(|s| !s.is_empty())
            .map(ToString::to_string);
        let zip = parts
            .get(9)
            .filter(|s| !s.is_empty())
            .map(ToString::to_string);
        let country = parts
            .get(10)
            .filter(|s| !s.is_empty())
            .map(ToString::to_string);

        Ok(Self {
            first_name,
            last_name,
            gender,
            birthday,
            club,
            email,
            phone,
            city,
            street,
            zip,
            country,
        })
    }
}
