use std::collections::hash_map::Entry;
use std::collections::HashMap;

use futures::{SinkExt, StreamExt};
use serialport::{SerialPortType, UsbPortInfo};
use tokio_serial::{ClearBuffer, SerialPort, SerialPortBuilderExt, SerialStream};
use tokio_util::codec::Framed;

use crate::error::Result;
use crate::protocol::responses::card::CardType;
use crate::protocol::responses::card_punch::CardPunch;
use crate::protocol::{
    Beep, CardBlocks, CardOwnerData, CardReadout, Codec, Commands, DecoderError, FromCardBlocks,
    GetSystemConfiguration, ReadCardData, ReadCardDataResponse, Responses, SetMasterSlave,
    StationMode, SystemConfiguration, BLOCK_SIZE,
};
use crate::Error;

const HIGH_SPEED_BAUD_RATE: u32 = 38400;
const LOW_SPEED_BAUD_RATE: u32 = 4800;

pub struct Reader {
    framed_codec: Framed<SerialStream, Codec>,
    system_configuration: SystemConfiguration,
}

impl Reader {
    /// Connect to the specified serial port.
    pub async fn connect<'a>(path: impl Into<std::borrow::Cow<'a, str>> + Send) -> Result<Self> {
        let mut port = tokio_serial::new(path, HIGH_SPEED_BAUD_RATE).open_native_async()?;

        #[cfg(unix)]
        port.set_exclusive(true)?;

        // flush port io.
        port.clear(ClearBuffer::All)?;

        let mut framed_codec = Framed::new(port, Codec::default());
        // set master slave.
        if send_and_receive_command(
            &mut framed_codec,
            Commands::SetMasterSlave(SetMasterSlave::Master),
        )
        .await
        .is_err()
        {
            // try low speed baud rate
            framed_codec.get_mut().set_baud_rate(LOW_SPEED_BAUD_RATE)?;
            send_and_receive_command(
                &mut framed_codec,
                Commands::SetMasterSlave(SetMasterSlave::Master),
            )
            .await?;
        }

        let system_configuration = send_and_receive_command(
            &mut framed_codec,
            Commands::GetSystemConfiguration(GetSystemConfiguration),
        )
        .await?;
        let Responses::SystemConfiguration(system_configuration) = system_configuration else {
            return Err(Error::InvalidResponseReceived);
        };
        Ok(Self {
            framed_codec,
            system_configuration,
        })
    }

    pub async fn auto_connect() -> Result<Self> {
        const SPORTIDENT_VENDOR_ID: u16 = 4292;
        const SPORTIDENT_READER_PRODUCT_ID: u16 = 32778;

        for port in serialport::available_ports()? {
            if let SerialPortType::UsbPort(UsbPortInfo {
                vid: SPORTIDENT_VENDOR_ID,
                pid: SPORTIDENT_READER_PRODUCT_ID,
                ..
            }) = port.port_type
            {
                if let Ok(reader) = Self::connect(port.port_name).await {
                    return Ok(reader);
                }
            }
        }

        Err(Error::NoReaderDetected)
    }
}

impl Reader {
    pub async fn beep_until_card_removed(&mut self) -> Result<()> {
        Ok(self.framed_codec.send(Commands::Beep(Beep)).await?)
    }
    pub async fn poll_card(&mut self) -> Result<CardReadout> {
        self.poll_card_generic().await
    }

    pub async fn poll_card_with_owner_data(&mut self) -> Result<(CardReadout, CardOwnerData)> {
        self.poll_card_generic().await
    }
    pub async fn poll_owner_data(&mut self) -> Result<CardOwnerData> {
        self.poll_card_generic().await
    }

    async fn poll_card_generic<T: FromCardBlocks>(&mut self) -> Result<T> {
        if !self
            .system_configuration
            .protocol_configuration
            .is_extended_protocol()
        {
            return Err(Error::NotExtendedProtocolMode);
        }
        if self.system_configuration.mode != StationMode::Readout {
            return Err(Error::NotReadoutMode);
        }

        loop {
            if let Responses::CardInserted(card) = receive_command(&mut self.framed_codec).await? {
                let card_data = self.read_card_data(card.card_type).await?;
                return Ok(card_data);
            }
        }
    }
    pub async fn poll_punch(&mut self) -> Result<CardPunch> {
        if !self
            .system_configuration
            .protocol_configuration
            .is_extended_protocol()
        {
            return Err(Error::NotExtendedProtocolMode);
        }

        if !self
            .system_configuration
            .protocol_configuration
            .is_auto_send()
        {
            return Err(Error::NotAutoSendMode);
        }

        match receive_command(&mut self.framed_codec).await? {
            Responses::CardPunch(punch) => Ok(punch),
            _ => Err(Error::InvalidResponseReceived),
        }
    }

    async fn read_card_data<T: FromCardBlocks>(&mut self, card_type: CardType) -> Result<T> {
        T::from_card_blocks(&mut ReaderBlocks::new(&mut self.framed_codec), card_type).await
    }
}

async fn send_and_receive_command(
    framed: &mut Framed<SerialStream, Codec>,
    cmd: Commands,
) -> Result<Responses> {
    framed.send(cmd).await?;

    receive_command(framed).await
}

async fn receive_command(framed: &mut Framed<SerialStream, Codec>) -> Result<Responses> {
    Ok(framed.next().await.ok_or(Error::PortClosed)??)
}

async fn receive_card_data_response(
    framed: &mut Framed<SerialStream, Codec>,
) -> Result<ReadCardDataResponse> {
    let response = receive_command(framed).await.map_err(|e| {
        if matches!(e, Error::DecoderError(DecoderError::InvalidCommandSent)) {
            Error::CardRemovedWhileReadingData
        } else {
            e
        }
    })?;

    match response {
        Responses::CardRemoved(_) => Err(Error::CardRemovedWhileReadingData),
        Responses::CardData(card_data) => Ok(card_data),
        _ => Err(Error::InvalidResponseReceived),
    }
}

struct ReaderBlocks<'a> {
    framed_codec: &'a mut Framed<SerialStream, Codec>,
    cache: HashMap<usize, [u8; BLOCK_SIZE]>,
}

impl<'a> ReaderBlocks<'a> {
    fn new(framed_codec: &'a mut Framed<SerialStream, Codec>) -> Self {
        Self {
            framed_codec,
            cache: HashMap::default(),
        }
    }
}

impl<'a> CardBlocks for ReaderBlocks<'a> {
    async fn get_block<'b>(&'b mut self, index: u8) -> Result<&'b [u8; BLOCK_SIZE]> {
        if let Entry::Vacant(e) = self.cache.entry(index as usize) {
            self.framed_codec
                .send(Commands::ReadCardData(ReadCardData::new(index)))
                .await?;

            let block = receive_card_data_response(self.framed_codec).await?;
            e.insert(block.0);
        }

        Ok(self.cache.get(&(index as usize)).unwrap())
    }
}
