mod ping_request;
mod ping_response;
mod status_request;
mod status_response;

pub use ping_request::PingRequestPacket;
pub use ping_response::PingResponsePacket;
pub use status_request::StatusRequestPacket;
pub use status_response::StatusResponsePacket;

use crate::connection::Connection;
use crate::error::ProtocolError;
use crate::packet::{PacketMeta, RawPacket};

pub async fn handle(conn: &mut Connection, raw: RawPacket) -> Result<bool, ProtocolError> {
    match raw.id {
        StatusRequestPacket::ID => {
            let _request: StatusRequestPacket = raw.decode()?;
            conn.send(&StatusResponsePacket::takumi_default()).await?;
            Ok(false)
        }

        PingRequestPacket::ID => {
            let ping: PingRequestPacket = raw.decode()?;
            conn.send(&PingResponsePacket::from(ping)).await?;
            Ok(true)
        }

        id => Err(ProtocolError::UnknownPacket { id }),
    }
}
