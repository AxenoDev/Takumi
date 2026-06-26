use takumi_macros::PacketIn;

use crate::IncomingPacket;

#[derive(Debug, Clone, PacketIn)]
#[packet(id = 0x02)]
pub struct LoginPluginResponsePacket {
    pub message_id: i32,
    pub data: Option<Vec<u8>>,
}

impl IncomingPacket for LoginPluginResponsePacket {
    fn decode_payload(
        reader: &mut crate::reader::PacketReader<'_>,
    ) -> Result<Self, crate::ProtocolError>
    where
        Self: Sized,
    {
        let message_id = reader.read_varint()?;
        let data = Some(reader.read_remaining_bytes());

        Ok(Self { message_id, data })
    }
}
