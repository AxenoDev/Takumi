use crate::{OutgoingPacket, error::ProtocolError, writer::PacketWriter};
use takumi_macros::PacketOut;

#[derive(Debug, Clone, PacketOut)]
#[packet(id = 0x03)]
pub struct FinishConfigurationPacket;

impl OutgoingPacket for FinishConfigurationPacket {
    fn encode_payload(&self, _writer: &mut PacketWriter) -> Result<(), ProtocolError> {
        Ok(())
    }
}
