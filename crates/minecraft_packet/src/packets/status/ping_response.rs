use takumi_macros::PacketOut;

use crate::OutgoingPacket;
use crate::error::ProtocolError;
use crate::writer::PacketWriter;

use super::PingRequestPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PacketOut)]
#[packet(id = 0x01)]
pub struct PingResponsePacket {
    pub payload: i64,
}

impl From<PingRequestPacket> for PingResponsePacket {
    fn from(request: PingRequestPacket) -> Self {
        Self {
            payload: request.payload,
        }
    }
}

impl OutgoingPacket for PingResponsePacket {
    fn encode_payload(&self, writer: &mut PacketWriter) -> Result<(), ProtocolError> {
        writer.write_i64(self.payload);
        Ok(())
    }
}