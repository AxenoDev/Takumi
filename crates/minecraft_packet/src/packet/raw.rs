use crate::error::ProtocolError;
use crate::packet::IncomingPacket;
use crate::reader::PacketReader;

#[derive(Debug, Clone)]
pub struct RawPacket {
    pub id: i32,
    pub payload: Vec<u8>,
}

impl RawPacket {
    pub fn decode<P: IncomingPacket>(&self) -> Result<P, ProtocolError> {
        if self.id != P::ID {
            return Err(ProtocolError::UnknownPacket { id: self.id });
        }

        let mut reader = PacketReader::new(&self.payload);
        P::decode_payload(&mut reader)
    }
}
