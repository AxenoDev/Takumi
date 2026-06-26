use takumi_macros::PacketIn;

use crate::IncomingPacket;

#[derive(Debug, Clone, PacketIn)]
#[packet(id = 0x03)]
pub struct LoginAcknowledgedPacket;

impl IncomingPacket for LoginAcknowledgedPacket {
    fn decode_payload(
        _reader: &mut crate::reader::PacketReader<'_>,
    ) -> Result<Self, crate::ProtocolError>
    where
        Self: Sized,
    {
        Ok(Self)
    }
}
