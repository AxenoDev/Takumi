use takumi_macros::PacketIn;

use crate::IncomingPacket;

#[derive(Debug, Clone, PacketIn)]
#[packet(id = 0x02)]
pub struct LoginPluginResponsePacket {
    pub message_id: i32,
    pub is_present: bool,
    pub data: Vec<u8>,
}

impl IncomingPacket for LoginPluginResponsePacket {
    fn decode_payload(
        reader: &mut crate::reader::PacketReader<'_>,
    ) -> Result<Self, crate::ProtocolError>
    where
        Self: Sized,
    {
        let message_id = reader.read_varint()?;
        let is_present = reader.read_bool()?;
        let data = if is_present {
            Some(reader.read_remaining_bytes())
        } else {
            None
        };

        Ok(Self {
            message_id,
            is_present,
            data: data.unwrap_or_default(),
        })
    }
}
