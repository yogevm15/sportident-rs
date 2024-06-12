use futures::{SinkExt, StreamExt};
use tokio_serial::{ClearBuffer, SerialPort, SerialPortBuilderExt, SerialStream};
use tokio_util::codec::Framed;

use crate::Error;
use crate::error::Result;
use crate::protocol::{Beep, CardOwnerData, CardReadout, Codec, Commands, DecoderError, FromCardBlocks, GetSystemConfiguration, ReadCardData, ReadCardDataResponse, Responses, SetMasterSlave, StationMode, SystemConfiguration};
use crate::protocol::card::CardType;

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
}


impl Reader {
    pub async fn poll_card(&mut self) -> Result<CardReadout> {
        self.poll_card_generic().await
    }

    pub async fn poll_card_with_owner_data(&mut self) -> Result<(CardReadout, CardOwnerData)> {
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
            match receive_command(&mut self.framed_codec).await? {
                Responses::CardInserted(card) => {
                    let card_data = self.read_card_data(card.card_type).await?;
                    self.framed_codec.send(Commands::Beep(Beep)).await?;
                    return Ok(card_data);

                }
                Responses::CardRemoved(_) => return Err(Error::CardRemovedWhileReadingData),
                _ => {}
            }
        }
    }

    async fn read_card_data<T: FromCardBlocks>(&mut self, card_type: CardType) -> Result<T> {
        let mut card_data = vec![];
        match card_type {
            CardType::Si8 | CardType::Si9 | CardType::PunchCard => {
                for i in 0..=1 {
                    self.framed_codec
                        .send(Commands::ReadCardData(ReadCardData::new(i)))
                        .await?;

                    card_data.extend_from_slice(
                        &receive_card_data_response(&mut self.framed_codec).await?,
                    );
                }
            }
            
            CardType::Si10 | CardType::Si11 | CardType::Siac => {
                if T::INCLUDE_OWNER_DATA_BLOCKS {
                    for i in 0..=1 {
                        self.framed_codec
                            .send(Commands::ReadCardData(ReadCardData::new(i)))
                            .await?;

                        card_data.extend_from_slice(
                            &receive_card_data_response(&mut self.framed_codec).await?,
                        );
                    }
                }

                self.framed_codec
                    .send(Commands::ReadCardData(ReadCardData::new(8)))
                    .await?;
                for _ in 0..5 {
                    card_data.extend_from_slice(
                        &receive_card_data_response(&mut self.framed_codec).await?,
                    );
                }
            }
        }

        Ok(T::from_card_blocks(&card_data, card_type)?)
    }
}

pub async fn send_and_receive_command(
    framed: &mut Framed<SerialStream, Codec>,
    cmd: Commands,
) -> Result<Responses> {
    framed.send(cmd).await?;

    receive_command(framed).await
}

pub async fn receive_command(framed: &mut Framed<SerialStream, Codec>) -> Result<Responses> {
    Ok(framed.next().await.ok_or(Error::PortClosed)??)
}

pub async fn receive_card_data_response(
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
