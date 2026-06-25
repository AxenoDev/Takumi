use takumi_macros::PacketOut;

use crate::OutgoingPacket;
use crate::error::ProtocolError;
use crate::writer::PacketWriter;

#[derive(Debug, Clone, PartialEq, Eq, PacketOut)]
#[packet(id = 0x00)]
pub struct StatusResponsePacket {
    pub json: String,
}

impl StatusResponsePacket {
    pub fn takumi_default() -> Self {
        Self {
            json: r#"{"version":{"name":"26.2","protocol":776},"players":{"max":100,"online":0},"description":{"text":"Takumi Server"}}"#.into(),
        }
    }
}

impl OutgoingPacket for StatusResponsePacket {
    fn encode_payload(&self, writer: &mut PacketWriter) -> Result<(), ProtocolError> {
        writer.write_string(&self.json);
        Ok(())
    }
}