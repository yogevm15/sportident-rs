use std::str::from_utf8;
use crate::protocol::card::CardType;
use crate::protocol::{CardReadout, DecoderError, FromCardBlocks, BLOCK_SIZE};

#[derive(Debug)]
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

impl FromCardBlocks for CardOwnerData {
    const INCLUDE_OWNER_DATA_BLOCKS: bool = true;

    fn from_card_blocks(data: &[u8], card_type: CardType) -> Result<Self, DecoderError> {
        if data.len() < BLOCK_SIZE*2 {
            return Err(DecoderError::InvalidReadoutDataLength);
        }

        match card_type {
            CardType::PunchCard | CardType::Si10 | CardType::Si11 | CardType::Siac => {
                let owner_data_slice = &data[32..160];
                let eleventh_semicolon_pos = owner_data_slice
                    .iter()
                    .enumerate()
                    .filter_map(|(i, &b)| if b == b';' { Some(i) } else { None })
                    .nth(10)
                    .ok_or(DecoderError::InvalidOwnerData)?;
                
                let Ok(owner_data_str) = from_utf8(&owner_data_slice[..eleventh_semicolon_pos]) else { return Err(DecoderError::InvalidOwnerData) };

                let parts: Vec<_> = owner_data_str.split(';').collect();

                if parts.len() != 11 {
                    return Err(DecoderError::InvalidOwnerData);
                }

                let first_name = parts[0].to_string();
                let last_name = parts[1].to_string();
                let gender = parts[2].to_string();
                let birthday = parts[3].to_string();
                let club = parts[4].to_string();
                let email = parts[5].to_string();
                let phone = parts[6].to_string();
                let city = parts[7].to_string();
                let street = parts[8].to_string();
                let zip = parts[9].to_string();
                let country = parts[10].to_string();

                Ok(Self {
                    first_name,
                    last_name,
                    gender: Some(gender),
                    birthday: Some(birthday),
                    club: Some(club),
                    email: Some(email),
                    phone: Some(phone),
                    city: Some(city),
                    street: Some(street),
                    zip: Some(zip),
                    country: Some(country),
                })
            }

            CardType::Si9 | CardType::Si8 => {
                let owner_data_slice = &data[32..if card_type == CardType::Si9 { 56 } else { 136 }];
                let second_semicolon_pos = owner_data_slice
                    .iter()
                    .enumerate()
                    .filter_map(|(i, &b)| if b == b';' { Some(i) } else { None })
                    .nth(1)
                    .ok_or(DecoderError::InvalidOwnerData)?;

                let Ok(owner_data_str) = from_utf8(&owner_data_slice[..second_semicolon_pos]) else { return Err(DecoderError::InvalidOwnerData) };

                let parts: Vec<_> = owner_data_str.split(';').collect();

                if parts.len() != 2 {
                    return Err(DecoderError::InvalidOwnerData);
                }

                let first_name = parts[0].to_string();
                let last_name = parts[1].to_string();

                Ok(Self {
                    first_name,
                    last_name,
                    gender: None,
                    birthday: None,
                    club: None,
                    email: None,
                    phone: None,
                    city: None,
                    street: None,
                    zip: None,
                    country: None,
                })
            }
        }
    }
}

impl FromCardBlocks for (CardReadout, CardOwnerData) {
    const INCLUDE_OWNER_DATA_BLOCKS: bool =
        CardReadout::INCLUDE_OWNER_DATA_BLOCKS || CardOwnerData::INCLUDE_OWNER_DATA_BLOCKS;

    fn from_card_blocks(data: &[u8], card_type: CardType) -> Result<Self, DecoderError> {

        if match card_type {
            CardType::Si8 | CardType::Si9 | CardType::PunchCard => data.len() < BLOCK_SIZE * 2,
            CardType::Si10 | CardType::Si11 | CardType::Siac => data.len() < BLOCK_SIZE * 7,
        } {
            return Err(DecoderError::InvalidReadoutDataLength);
        }

        let card_readout_slice = match card_type {
            CardType::Si10 | CardType::Si11 | CardType::Siac => {
                &data[BLOCK_SIZE * 2..BLOCK_SIZE * 7]
            }
            CardType::Si8 | CardType::Si9 | CardType::PunchCard => &data[0..BLOCK_SIZE * 2],
        };
        let card_readout = CardReadout::from_card_blocks(card_readout_slice, card_type)?;

        let owner_data = CardOwnerData::from_card_blocks(&data[0..BLOCK_SIZE*2], card_type)?;

        Ok((card_readout, owner_data))
    }
}
