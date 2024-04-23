use std::ops::BitAnd;

use bitflags::bitflags;
use chrono::{Duration, NaiveDate};
use strum_macros::FromRepr;

use crate::protocol::commands::Response;
use crate::protocol::decoder::DecoderError;
use crate::protocol::Command;

const SYSTEM_CONFIGURATION_LENGTH: usize = 0x81;

#[allow(dead_code)]
pub struct SystemConfiguration {
    pub serial_number: u32,
    pub srr_configuration: SRRConfiguration,
    pub firmware: [u8; 3],
    pub build_date: NaiveDate,
    pub model: Model,
    pub mem_kilobytes: u8,
    pub battery_date: NaiveDate,
    pub battery_capacity_milliampere_hour: u16,
    pub backup_pointer_high: u16,
    pub backup_pointer_low: u16,
    pub si6_card_blocks: SI6CardBlocks,
    pub srr_channel: SRRChannel,
    pub used_battery_capacity_percentage: f64,
    pub memory_overflow: bool,
    pub battery_voltage: f64,
    pub station_program: StationProgram,
    pub mode: StationMode,
    pub station_code: u16,
    pub punch_feedback: PunchFeedback,
    pub protocol_configuration: ProtocolConfiguration,
    pub wakeup_date: NaiveDate,
    pub active_duration: Duration,
}

bitflags! {
    pub struct SI6CardBlocks: u8 {
        const FIRST =   0b0000_0001;
        const SECOND =  0b0000_0010;
        const THIRD =   0b0000_0100;
        const FOURTH =  0b0000_1000;
        const FIFTH =   0b0001_0000;
        const SIXTH =   0b0010_0000;
        const SEVENTH = 0b0100_0000;
        const EIGHTH =  0b1000_0000;
    }
}

bitflags! {
    pub struct PunchFeedback: u8 {
        const OPTICAL = 0b0000_0001;
        const AUDIBLE = 0b0000_0100;
    }
}

bitflags! {
    pub struct SRRConfiguration: u8 {
        const OPTICAL = 0b0000_0001;
        const AUDIBLE = 0b0000_0100;
    }
}

bitflags! {
    pub struct ProtocolConfiguration: u8 {
        const EXTENDED_PROTOCOL =   0b0000_0001;
        const AUTO_SEND_OUT =       0b0000_0010;
        const HANDSHAKE =           0b0000_0100;
        const PASSWORD_ACCESS =     0b0001_0000;
        const READ_OUT =            0b0010_0000;
    }
}

#[derive(FromRepr)]
#[repr(u16)]
pub enum Model {
    SRRDongle = 0x6F21,
    BSF3 = 0x8003,
    BSF4 = 0x8004,
    BSM4RS232 = 0x8084,
    BSM6RS232 = 0x8086,
    BSF5 = 0x8115,
    BSF7V1 = 0x8117,
    BSF8V1 = 0x8118,
    BSF6 = 0x8146,
    BSF7Master = 0x8187,
    BSF8Master = 0x8188,
    BSF7V2 = 0x8197,
    BSF8V2 = 0x8198,
    BSM7RS232 = 0x9197,
    BSM8SRR = 0x9198,
    BS7S = 0x9597,
    BS11BL = 0x9D9A,
    BS7P = 0xB197,
    BS7GSM = 0xB897,
    BS11BS = 0xCD9B,
}

#[derive(FromRepr)]
#[repr(u8)]
pub enum SRRChannel {
    Red = 0x00,
    Blue = 0x01,
}

pub enum StationProgram {
    Competition,
    Training,
}

#[derive(FromRepr, PartialEq, Eq)]
#[repr(u8)]
pub enum StationMode {
    SIACSpecial = 0x01,
    Control = 0x02,
    Start = 0x03,
    Finish = 0x04,
    Readout = 0x05,
    ClearOld = 0x06,
    Clear = 0x07,
    Check = 0x0A,
    PrintOut = 0x0B,
    StartTrigger = 0x0C,
    FinishTrigger = 0x0D,
    BeaconControl = 0x12,
    BeaconStart = 0x13,
    BeaconFinish = 0x14,
    BeaconReadout = 0x15,
}

#[derive(Copy, Clone, Debug)]
pub struct GetSystemConfiguration;

impl Command for GetSystemConfiguration {
    fn command_parameters(&self) -> Vec<u8> {
        vec![0x00, 0x80]
    }
    fn magic(&self) -> u8 {
        0x83
    }
}
impl Response for SystemConfiguration {
    fn decode(data: &[u8]) -> Result<Self, DecoderError> {
        if data.len() != SYSTEM_CONFIGURATION_LENGTH {
            return Err(DecoderError::InvalidSystemConfiguration(
                SYSTEM_CONFIGURATION_LENGTH,
                data.len(),
            ));
        }

        // Ignoring the first byte.
        let data = &data[1..];

        let model_id = u16::from_be_bytes([data[11], data[12]]);

        Ok(Self {
            serial_number: u32::from_le_bytes([data[0], data[1], data[2], data[3]]),
            srr_configuration: SRRConfiguration::from_bits_retain(data[4]),
            firmware: [data[5], data[6], data[7]],
            build_date: naive_date_from_data(data[8], data[9], data[10])?,
            model: Model::from_repr(model_id).ok_or(DecoderError::UnknownModelId(model_id))?,
            mem_kilobytes: data[13],
            battery_date: naive_date_from_data(data[21], data[22], data[23])?,
            battery_capacity_milliampere_hour: u16::from_be_bytes([data[25], data[26]]),
            backup_pointer_high: u16::from_be_bytes([data[28], data[29]]),
            backup_pointer_low: u16::from_be_bytes([data[33], data[34]]),
            si6_card_blocks: SI6CardBlocks::from_bits_retain(data[51]),
            srr_channel: SRRChannel::from_repr(data[52])
                .ok_or(DecoderError::UnknownSRRChannel(data[52]))?,
            used_battery_capacity_percentage: f64::from(
                data[53].wrapping_shl(16) + data[54].wrapping_shl(8) + data[55],
            ) * 2.778e-5,
            memory_overflow: data[61] != 0,
            battery_voltage: f64::from(u16::from_be_bytes([data[80], data[81]]))
                * (5f64 / 65536f64),
            station_program: if data[112].bitand(0b0010_0000) == 0 {
                StationProgram::Competition
            } else {
                StationProgram::Training
            },
            mode: StationMode::from_repr(data[113])
                .ok_or(DecoderError::UnknownStationMode(data[113]))?,
            station_code: u16::from_be_bytes([data[114], data[115].bitand(0b1100_0000)]),
            punch_feedback: PunchFeedback::from_bits_retain(data[115]),
            protocol_configuration: ProtocolConfiguration::from_bits_retain(data[116]),
            wakeup_date: naive_date_from_data(data[117], data[118], data[119])?,
            active_duration: unsafe {
                Duration::try_minutes(i64::from(data[126])).unwrap_unchecked()
            },
        })
    }
}

fn naive_date_from_data(year: u8, month: u8, day: u8) -> Result<NaiveDate, DecoderError> {
    let (year, month, day) = (2000 + i32::from(year), u32::from(month), u32::from(day));

    NaiveDate::from_ymd_opt(year, month, day).ok_or(DecoderError::InvalidDate(year, month, day))
}

impl ProtocolConfiguration {
    pub const fn is_extended_protocol(&self) -> bool {
        self.contains(Self::EXTENDED_PROTOCOL)
    }
}
