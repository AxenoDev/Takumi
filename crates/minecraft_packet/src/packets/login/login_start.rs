use takumi_macros::PacketIn;
use uuid::Uuid;

use crate::IncomingPacket;

#[derive(Debug, Clone, PacketIn)]
#[packet(id = 0x00)]
pub struct LoginStartPacket {
    pub name: String,
    pub uuid: Uuid,
}

impl IncomingPacket for LoginStartPacket {
    fn decode_payload(
        reader: &mut crate::reader::PacketReader<'_>,
    ) -> Result<Self, crate::ProtocolError>
    where
        Self: Sized,
    {
        let name = reader.read_string()?;
        let uuid = reader.read_uuid()?;

        Ok(Self { name, uuid })
    }
}
