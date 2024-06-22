use crate::protocol::responses::card::CardType;
use crate::protocol::DecoderError;
use chrono::{NaiveTime, TimeDelta};
use std::ops::{AddAssign, BitAnd, Shl, Shr};
use strum_macros::FromRepr;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, FromRepr)]
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

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, FromRepr)]
#[repr(u8)]
pub enum WeekCounter {
    First = 0,
    Second = 1,
    Third = 2,
    Fourth = 3,
}

struct BasePunch {
    time: NaiveTime,
    day_of_week: DayOfWeek,
    week_counter: WeekCounter,
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Punch {
    pub time: NaiveTime,
    pub day_of_week: DayOfWeek,
    pub week_counter: WeekCounter,
    pub code: u16,
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct SubSecondPunch {
    pub time: NaiveTime,
    pub day_of_week: DayOfWeek,
    pub week_counter: WeekCounter,
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum StartOrFinishPunch {
    Normal(Punch),
    SubSecond(SubSecondPunch),
}

const TWELVE_HOURS: TimeDelta = {
    let Some(t) = TimeDelta::try_hours(12) else {
        unreachable!()
    };

    t
};

impl BasePunch {
    fn decode_punch(card_type: CardType, data: [u8; 4]) -> Result<Option<Self>, DecoderError> {
        match card_type {
            CardType::Si8
            | CardType::Si9
            | CardType::Si10
            | CardType::Si11
            | CardType::Siac
            | CardType::PunchCard => {
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

                Ok(Some(Self {
                    time,
                    day_of_week: unsafe { DayOfWeek::from_repr(day_of_week).unwrap_unchecked() },
                    week_counter: unsafe {
                        WeekCounter::from_repr(week_counter).unwrap_unchecked()
                    },
                }))
            }
        }
    }
}

impl Punch {
    pub(crate) fn decode_punch(
        card_type: CardType,
        data: [u8; 4],
    ) -> Result<Option<Self>, DecoderError> {
        match card_type {
            CardType::Si8
            | CardType::Si9
            | CardType::Si10
            | CardType::Si11
            | CardType::Siac
            | CardType::PunchCard => {
                let Some(BasePunch {
                    time,
                    day_of_week,
                    week_counter,
                }) = BasePunch::decode_punch(card_type, data)?
                else {
                    return Ok(None);
                };

                let code = u16::from(data[1]) + u16::from(data[0].bitand(0b1100_0000)).shl(2u8);

                Ok(Some(Self {
                    time,
                    day_of_week,
                    week_counter,
                    code,
                }))
            }
        }
    }
}

impl SubSecondPunch {
    pub(crate) fn decode_punch(
        card_type: CardType,
        data: [u8; 4],
    ) -> Result<Option<Self>, DecoderError> {
        match card_type {
            CardType::Si8
            | CardType::Si9
            | CardType::Si10
            | CardType::Si11
            | CardType::Siac
            | CardType::PunchCard => {
                let Some(BasePunch {
                    mut time,
                    day_of_week,
                    week_counter,
                }) = BasePunch::decode_punch(card_type, data)?
                else {
                    return Ok(None);
                };

                time.add_assign(TimeDelta::milliseconds(i64::from(data[1]) * 1000 / 255));

                Ok(Some(Self {
                    time,
                    day_of_week,
                    week_counter,
                }))
            }
        }
    }
}

impl StartOrFinishPunch {
    pub(crate) fn decode_punch(
        card_type: CardType,
        data: [u8; 4],
    ) -> Result<Option<Self>, DecoderError> {
        match card_type {
            CardType::Si8
            | CardType::Si9
            | CardType::Si10
            | CardType::Si11
            | CardType::Siac
            | CardType::PunchCard => Ok(if data[0].bitand(0b1000_0000) == 0 {
                Punch::decode_punch(card_type, data)?.map(StartOrFinishPunch::Normal)
            } else {
                SubSecondPunch::decode_punch(card_type, data)?.map(StartOrFinishPunch::SubSecond)
            }),
        }
    }
}
