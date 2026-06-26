use crate::IncomingPacket;
use takumi_macros::PacketIn;

#[derive(Debug, Clone, PacketIn)]
#[packet(id = 0x02)]
pub struct PluginMessagePacket {
    pub channel: String,
    pub data: Vec<u8>,
}

impl IncomingPacket for PluginMessagePacket {
    fn decode_payload(
        reader: &mut crate::reader::PacketReader<'_>,
    ) -> Result<Self, crate::ProtocolError> {
        Ok(Self {
            channel: reader.read_string()?,
            data: reader.read_remaining_bytes(),
        })
    }
}
