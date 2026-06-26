use crate::IncomingPacket;
use takumi_macros::PacketIn;

#[derive(Debug, Clone, PacketIn)]
#[packet(id = 0x03)]
pub struct AcknowledgeFinishConfigurationPacket;

impl IncomingPacket for AcknowledgeFinishConfigurationPacket {
    fn decode_payload(
        _reader: &mut crate::reader::PacketReader<'_>,
    ) -> Result<Self, crate::ProtocolError> {
        Ok(Self)
    }
}
