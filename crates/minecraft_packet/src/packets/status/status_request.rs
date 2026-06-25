use takumi_macros::PacketIn;

use crate::error::ProtocolError;
use crate::reader::PacketReader;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, PacketIn)]
#[packet(id = 0x00)]
pub struct StatusRequestPacket;

impl crate::packet::IncomingPacket for StatusRequestPacket {
    fn decode_payload(_reader: &mut PacketReader<'_>) -> Result<Self, ProtocolError> {
        Ok(Self)
    }
}
