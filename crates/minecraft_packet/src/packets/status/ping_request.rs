use takumi_macros::PacketIn;

use crate::IncomingPacket;
use crate::error::ProtocolError;
use crate::reader::PacketReader;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PacketIn)]
#[packet(id = 0x01)]
pub struct PingRequestPacket {
    pub payload: i64,
}

impl IncomingPacket for PingRequestPacket {
    fn decode_payload(reader: &mut PacketReader<'_>) -> Result<Self, ProtocolError> {
        Ok(Self {
            payload: reader.read_i64()?,
        })
    }
}
