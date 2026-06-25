use crate::protocol::reader::PacketReader;

#[derive(Debug)]
pub struct TransferPacket {
    pub host: String,
    pub port: u16,
}

impl TransferPacket {
    pub fn decode(reader: &mut PacketReader) -> Result<Self, &'static str> {
        let host = reader.read_string()?;
        let port = reader.read_u16()?;

        Ok(Self {
            host,
            port
        })
    }
}