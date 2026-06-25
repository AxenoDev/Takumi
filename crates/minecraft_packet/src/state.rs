use crate::packets::handshaking::Intent;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Handshaking,
    Status,
    Login,
    Transfer,
}

impl From<Intent> for ConnectionState {
    fn from(intent: Intent) -> Self {
        match intent {
            Intent::Status => Self::Status,
            Intent::Login => Self::Login,
            Intent::Transfer => Self::Transfer,
        }
    }
}
