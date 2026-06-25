use crate::protocol::reader::PacketReader;

#[derive(Debug)]
pub enum Intent {
    Status = 1,
    Login = 2,
    Transfer = 3
}

#[derive(Debug)]
pub struct HandshakePacket {
    pub protocol_version: i32,
    pub server_address: String,
    pub server_port: u16,
    pub intent: Intent,
}

impl HandshakePacket {
    pub fn decode(reader: &mut PacketReader) -> Result<Self, &'static str> {
        let protocol_version = reader.read_varint()?;

        let server_address = reader.read_string()?;

        let server_port = reader.read_u16()?;

        let intent = match reader.read_varint()? {
            1 => Intent::Status,
            2 => Intent::Login,
            3 => Intent::Transfer,
            _ => return Err("unknown intent"),
        };

        Ok(Self {
            protocol_version,
            server_address,
            server_port,
            intent,
        })
    }
}