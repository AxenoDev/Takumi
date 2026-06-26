use takumi_macros::PacketIn;

use crate::IncomingPacket;

#[derive(Debug, Clone, PacketIn)]
#[packet(id = 0x04)]
pub struct CookieResponseLoginPacket {
    pub key: String,
    pub payload: Option<Vec<u8>>,
}

impl IncomingPacket for CookieResponseLoginPacket {
    fn decode_payload(
        _reader: &mut crate::reader::PacketReader<'_>,
    ) -> Result<Self, crate::ProtocolError>
    where
        Self: Sized,
    {
        let key = _reader.read_string()?;
        let payload = Some(_reader.read_remaining_bytes());

        Ok(Self { key, payload })
    }
}
