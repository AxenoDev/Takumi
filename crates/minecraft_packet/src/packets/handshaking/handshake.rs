use takumi_macros::PacketIn;

use crate::{IncomingPacket, error::ProtocolError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Intent {
    Status,
    Login,
    Transfer,
}

impl Intent {
    pub fn from_varint(value: i32) -> Result<Self, ProtocolError> {
        match value {
            1 => Ok(Self::Status),
            2 => Ok(Self::Login),
            3 => Ok(Self::Transfer),
            _ => Err(ProtocolError::InvalidIntent(value)),
        }
    }

    pub const fn as_varint(self) -> i32 {
        match self {
            Self::Status => 1,
            Self::Login => 2,
            Self::Transfer => 3,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PacketIn)]
#[packet(id = 0x00)]
pub struct HandshakePacket {
    pub protocol_version: i32,
    pub server_address: String,
    pub server_port: u16,
    pub intent: Intent,
}

impl HandshakePacket {
    pub fn status(
        protocol_version: i32,
        server_address: impl Into<String>,
        server_port: u16,
    ) -> Self {
        Self {
            protocol_version,
            server_address: server_address.into(),
            server_port,
            intent: Intent::Status,
        }
    }

    pub fn login(
        protocol_version: i32,
        server_address: impl Into<String>,
        server_port: u16,
    ) -> Self {
        Self {
            protocol_version,
            server_address: server_address.into(),
            server_port,
            intent: Intent::Login,
        }
    }
}

impl IncomingPacket for HandshakePacket {
    fn decode_payload(
        reader: &mut crate::reader::PacketReader<'_>,
    ) -> Result<Self, crate::error::ProtocolError> {
        Ok(Self {
            protocol_version: reader.read_varint()?,
            server_address: reader.read_string()?,
            server_port: reader.read_u16()?,
            intent: Intent::from_varint(reader.read_varint()?)?,
        })
    }
}