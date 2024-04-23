use std::ops::{AddAssign, BitAnd, Shr};

use chrono::{NaiveTime, TimeDelta};
use strum_macros::FromRepr;

use crate::protocol::card::CardType;
use crate::protocol::DecoderError;

#[derive(Debug, FromRepr)]
#[repr(u8)]
pub enum DayOfWeek {
    Monday = 0,
    Tuesday = 1,
    Wednesday = 2,
    Thursday = 3,
    Friday = 4,
    Saturday = 5,
    Sunday = 6,
}

#[derive(Debug, FromRepr)]
#[repr(u8)]
pub enum WeekCounter {
    First = 0,
    Second = 1,
    Third = 2,
    Fourth = 3,
}

#[derive(Debug)]
pub struct Punch {
    pub time: NaiveTime,
    pub day_of_week: DayOfWeek,
    pub week_counter: WeekCounter,
    pub code: u16,
}
const TWELVE_HOURS: TimeDelta = {
    let Some(t) = TimeDelta::try_hours(12) else {
        unreachable!()
    };

    t
};

impl Punch {
    fn decode_punch(card_type: &CardType, data: [u8; 4]) -> Result<Option<Self>, DecoderError> {
        match card_type {
            CardType::Si8 | CardType::Si9 | CardType::Si10 | CardType::Si11 | CardType::Siac | CardType::PunchCard => {
                let seconds = u16::from_be_bytes([data[2], data[3]]);
                if seconds == 0xeeee {
                    return Ok(None);
                } else if seconds >= 43200 {
                    return Err(DecoderError::InvalidPunchTime);
                }

                let mut time = NaiveTime::from_num_seconds_from_midnight_opt(u32::from(seconds), 0)
                    .ok_or(DecoderError::InvalidPunchTime)?;
                let punch_time_date = data[0];
                if punch_time_date.bitand(0b0000_0001) == 1 {
                    time.add_assign(TWELVE_HOURS);
                }

                let day_of_week = punch_time_date
                    .bitand(0b0000_1110)
                    .shr(1u8)
                    .wrapping_sub(1u8)
                    % 7u8;
                let week_counter = punch_time_date.bitand(0b0011_0000).shr(4u8);
                let code =
                    u16::from(data[1]) + u16::from(punch_time_date.bitand(0b1100_0000).shr(6u8));

                Ok(Some(Self {
                    time,
                    day_of_week: unsafe { DayOfWeek::from_repr(day_of_week).unwrap_unchecked() },
                    week_counter: unsafe {
                        WeekCounter::from_repr(week_counter).unwrap_unchecked()
                    },
                    code,
                }))
            }
        }
    }
}

#[derive(Debug)]
pub struct CardReadout {
    pub card_number: u32,
    pub card_type: CardType,
    pub start: Option<Punch>,
    pub finish: Option<Punch>,
    pub check: Option<Punch>,
    pub punches: Vec<Punch>,
}

impl CardReadout {
    pub fn decode(data: &[u8], card_type: CardType) -> Result<Self, DecoderError> {
        match card_type {
            CardType::Si8 | CardType::Si9 | CardType::Si10 | CardType::Si11 | CardType::Siac | CardType::PunchCard => {
                if data.len() < 28 {
                    return Err(DecoderError::InvalidReadoutDataLength);
                }
                let punch_count = data[22];
                let punches_start_offset = usize::from(Self::punches_offset(&card_type));
                let punches_end_offset = punches_start_offset + usize::from(punch_count) * 4;
                if data.len() <= punches_end_offset {
                    return Err(DecoderError::InvalidReadoutDataLength);
                }
                let punches = data[punches_start_offset..punches_end_offset]
                    .chunks_exact(4)
                    .filter_map(|punch_data| {
                        Punch::decode_punch(&card_type, unsafe {
                            punch_data.try_into().unwrap_unchecked()
                        })
                        .transpose()
                    })
                    .collect::<Result<_, _>>()?;
                Ok(Self {
                    start: Punch::decode_punch(
                        &card_type,
                        [data[12], data[13], data[14], data[15]],
                    )?,
                    finish: Punch::decode_punch(
                        &card_type,
                        [data[16], data[17], data[18], data[19]],
                    )?,
                    check: Punch::decode_punch(&card_type, [data[8], data[9], data[10], data[11]])?,
                    punches,
                    card_number: u32::from_be_bytes([0, data[25], data[26], data[27]]),
                    card_type,
                })
            }
        }
    }

    const fn punches_offset(card_type: &CardType) -> u8 {
        match card_type {
            CardType::Si8 => 136,
            CardType::Si9 => 56,
            CardType::PunchCard => 176,
            CardType::Si10 | CardType::Si11 | CardType::Siac => 128,
        }
    }
}
