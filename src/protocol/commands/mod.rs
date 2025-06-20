use crate::protocol::{CommandEncoder, ResponseDecoder};
use crate::DecoderError;
pub use beep::*;
use futures::{SinkExt, StreamExt};
pub use read_card_data::*;
pub use set_master_slave::*;
use std::marker::PhantomData;
pub use system_configuration::*;
use tokio_serial::SerialStream;
use tokio_util::codec::{FramedRead, FramedWrite};

mod beep;
mod read_card_data;
mod set_master_slave;
mod system_configuration;

pub trait Command {
    const MAGIC: u8;
    type Response: Response;
    fn encode(&self) -> &[u8];
}

pub trait Response {
    type Output;

    async fn receive(port: &mut impl SportIdentPort) -> crate::Result<Self::Output>;
}

pub trait DecodableResponse: Sized {
    const MAGIC: u8;

    fn decode(data: &[u8]) -> Result<Self, DecoderError>;
}

pub struct NoResponse;

pub struct WithResponse<R: DecodableResponse> {
    marker: PhantomData<R>,
}

impl Response for NoResponse {
    type Output = ();
    async fn receive(_port: &mut impl SportIdentPort) -> crate::Result<Self::Output> {
        Ok(())
    }
}

impl<R: DecodableResponse> Response for WithResponse<R> {
    type Output = R;

    async fn receive(port: &mut impl SportIdentPort) -> crate::Result<Self::Output> {
        port.receive_command().await
    }
}

pub trait SportIdentPort {
    async fn send_command<C: Command>(
        &mut self,
        cmd: C,
    ) -> crate::Result<<C::Response as Response>::Output>;
    async fn receive_command<R: DecodableResponse>(&mut self) -> crate::Result<R>;
}

impl SportIdentPort for SerialStream {
    async fn send_command<C: Command>(
        &mut self,
        cmd: C,
    ) -> crate::Result<<C::Response as Response>::Output> {
        let mut self_ = self;
        FramedWrite::new(&mut self_, CommandEncoder)
            .send(cmd)
            .await?;
        C::Response::receive(self_).await
    }
    async fn receive_command<R: DecodableResponse>(&mut self) -> crate::Result<R> {
        Ok(FramedRead::new(self, ResponseDecoder::default())
            .next()
            .await
            .ok_or(crate::Error::PortClosed)??)
    }
}
