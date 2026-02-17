use bytes::{Bytes, BytesMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Handshaking,
    Status,
    Login,
    Play,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketDirection {
    // Client -> Proxy (serverbound from the client's perspective)
    Clientbound,
    // Proxy -> Server (serverbound from the server's perspective)
    Serverbound,
}

#[derive(Debug, Clone)]
pub struct Packet {
    pub id: i32,
    pub data: Bytes,
}

impl Packet {
    pub fn new(id: i32, data: Bytes) -> Self {
        Packet { id, data }
    }

    pub fn build(id: i32, builder: impl FnOnce(&mut BytesMut)) -> Self {
        let mut data = BytesMut::new();
        builder(&mut data);
        Self::new(id, data.freeze())
    }
}
