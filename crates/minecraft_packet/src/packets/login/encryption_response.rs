use takumi_macros::PacketIn;

use crate::IncomingPacket;

#[derive(Debug, Clone, PacketIn)]
#[packet(id = 0x01)]
pub struct EncryptionResponsePacket {
    pub shared_secret: Vec<u8>,
    pub verify_token: Vec<u8>,
}

impl IncomingPacket for EncryptionResponsePacket {
    fn decode_payload(
        reader: &mut crate::reader::PacketReader<'_>,
    ) -> Result<Self, crate::ProtocolError>
    where
        Self: Sized,
    {
        let shared_secret = reader.read_byte_array()?;
        let verify_token = reader.read_byte_array()?;

        Ok(Self {
            shared_secret,
            verify_token,
        })
    }
}
