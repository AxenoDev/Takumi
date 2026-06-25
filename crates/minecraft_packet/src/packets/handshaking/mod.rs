mod handshake;

pub use handshake::{HandshakePacket, Intent};

use crate::error::ProtocolError;
use crate::packet::RawPacket;

pub fn handle(raw: RawPacket) -> Result<Intent, ProtocolError> {
    let packet: HandshakePacket = raw.decode()?;
    Ok(packet.intent)
}
