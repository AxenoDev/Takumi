use anyhow::{Ok, bail};

use crate::protocol::State;
use crate::protocol::packet::Packet;
use crate::protocol::types::{read_string, read_varint};

pub struct Handshake {
    pub protocol_version: i32,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: i32,
}

pub struct HandshakeHandler;

impl HandshakeHandler {
    pub fn handle(packet: &Packet) -> anyhow::Result<State> {
        if packet.id != 0x00 {
            bail!(
                "Expected Handshake packet with ID 0x00, got {:#04x}",
                packet.id
            );
        }

        let mut data = packet.data.clone();
        let hs = Self::parse(&mut data)?;

        match hs.next_state {
            1 => Ok(State::Status),
            2 => Ok(State::Login),
            _ => bail!("Invalid next state in handshake: {}", hs.next_state),
        }
    }

    fn parse(data: &mut bytes::Bytes) -> anyhow::Result<Handshake> {
        use bytes::Buf;
        let protocol_version = read_varint(data)?;
        let server_address = read_string(data)?;
        if data.remaining() < 2 {
            bail!("Not enough data for server port");
        }
        let server_port = data.get_u16();
        let next_state = read_varint(data)?;

        Ok(Handshake {
            protocol_version,
            server_address,
            server_port,
            next_state,
        })
    }
}
