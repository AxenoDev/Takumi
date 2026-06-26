use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProtocolError {
    UnexpectedEof,
    VarIntTooBig,
    InvalidUtf8,
    InvalidUuid,
    InvalidIntent(i32),
    UnknownPacket { id: i32 },
    Io(String),
}

impl fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedEof => write!(f, "unexpected end of data"),
            Self::VarIntTooBig => write!(f, "VarInt exceeds maximum size"),
            Self::InvalidUtf8 => write!(f, "invalid UTF-8 string"),
            Self::InvalidUuid => write!(f, "invalid UUID"),
            Self::InvalidIntent(v) => write!(f, "unknown handshake intent: {v}"),
            Self::UnknownPacket { id } => write!(f, "unknown packet id: 0x{id:02X}"),
            Self::Io(msg) => write!(f, "io error: {msg}"),
        }
    }
}

impl std::error::Error for ProtocolError {}

impl From<std::io::Error> for ProtocolError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err.to_string())
    }
}
