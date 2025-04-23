use serialport::{SerialPortType, UsbPortInfo};
use tokio_serial::{ClearBuffer, SerialPort, SerialPortBuilderExt, SerialStream};

use crate::error::Result;
use crate::protocol::responses::card::CardInserted;
use crate::protocol::responses::card_punch::CardPunch;
use crate::protocol::{
    Beep, CardBlocks, CardOwnerData, CardReadout, FromCardBlocks, GetSystemConfiguration,
    ReadCardData, SetMasterSlave, SportIdentPort, StationMode, SystemConfiguration, BLOCK_SIZE,
};
use crate::Error;

const HIGH_SPEED_BAUD_RATE: u32 = 38400;
const LOW_SPEED_BAUD_RATE: u32 = 4800;

pub struct Reader {
    stream: SerialStream,
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

        // set master slave.
        if port.send_command(SetMasterSlave::Master).await.is_err() {
            // try low speed baud rate
            port.set_baud_rate(LOW_SPEED_BAUD_RATE)?;
            port.send_command(SetMasterSlave::Master).await?;
        }

        let system_configuration = port.send_command(GetSystemConfiguration).await?;
        Ok(Self {
            stream: port,
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
        self.stream.send_command(Beep).await
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

        let card: CardInserted = self.stream.receive_command().await?;
        let card_data =
            T::from_card_blocks(&mut ReaderBlocks::new(&mut self.stream), card.card_type).await?;
        Ok(card_data)
    }

    pub async fn wait_for_punch(&mut self) -> Result<CardPunch> {
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

        self.stream.receive_command().await
    }
}

struct ReaderBlocks<'a> {
    stream: &'a mut SerialStream,
    cache: [Option<Vec<u8>>; 8],
}

impl<'a> ReaderBlocks<'a> {
    const fn new(stream: &'a mut SerialStream) -> Self {
        Self {
            stream,
            cache: [const { None }; 8],
        }
    }
}

impl CardBlocks for ReaderBlocks<'_> {
    async fn get_block(&mut self, index: u8) -> Result<&[u8; BLOCK_SIZE]> {
        let block = &mut self.cache[index as usize];
        if block.is_none() {
            let res = self.stream.send_command(ReadCardData::new(index)).await?;
            *block = Some(res.into());
        }
        Ok(block.as_deref().unwrap().try_into().unwrap())
    }
}
