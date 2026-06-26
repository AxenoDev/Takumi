use crate::IncomingPacket;
use takumi_macros::PacketIn;

#[derive(Debug, Clone, PacketIn)]
#[packet(id = 0x00)]
pub struct ClientInformationPacket {
    pub locale: String,
    pub view_distance: i8,
    pub chat_mode: i32,
    pub chat_colors: bool,
    pub displayed_skin_parts: u8,
    pub main_hand: i32,
    pub enable_text_filtering: bool,
    pub allow_server_listings: bool,
}

impl IncomingPacket for ClientInformationPacket {
    fn decode_payload(
        reader: &mut crate::reader::PacketReader<'_>,
    ) -> Result<Self, crate::ProtocolError> {
        Ok(Self {
            locale: reader.read_string()?,
            view_distance: reader.read_i8()?,
            chat_mode: reader.read_varint()?,
            chat_colors: reader.read_bool()?,
            displayed_skin_parts: reader.read_u8()?,
            main_hand: reader.read_varint()?,
            enable_text_filtering: reader.read_bool()?,
            allow_server_listings: reader.read_bool()?,
        })
    }
}
