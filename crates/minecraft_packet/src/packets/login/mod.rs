use crate::connection::Connection;
use crate::error::ProtocolError;
use crate::packet::RawPacket;

pub async fn handle(_conn: &mut Connection, raw: RawPacket) -> Result<(), ProtocolError> {
    Err(ProtocolError::UnknownPacket { id: raw.id })
}
