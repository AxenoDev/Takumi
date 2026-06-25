use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::error::ProtocolError;
use crate::packet::{OutgoingPacket, RawPacket};
use crate::writer::PacketWriter;

pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }

    pub async fn receive(&mut self) -> Result<RawPacket, ProtocolError> {
        let length = self.read_varint().await? as usize;
        let mut data = vec![0u8; length];
        self.stream.read_exact(&mut data).await?;

        let mut pos = 0;
        let packet_id = read_varint_from_slice(&data, &mut pos)?;
        let payload = data.split_off(pos);

        Ok(RawPacket {
            id: packet_id,
            payload,
        })
    }

    pub async fn send<P: OutgoingPacket>(&mut self, packet: &P) -> Result<(), ProtocolError> {
        let payload = packet.encode()?;

        let mut frame = PacketWriter::new();
        frame.write_varint(payload.len() as i32);
        frame.extend(payload);

        self.stream.write_all(&frame.into_inner()).await?;
        Ok(())
    }

    async fn read_varint(&mut self) -> Result<i32, ProtocolError> {
        let mut result = 0i32;
        let mut num_read = 0;

        loop {
            let mut byte = [0u8; 1];
            self.stream.read_exact(&mut byte).await?;

            let b = byte[0];
            result |= ((b & 0x7F) as i32) << (7 * num_read);
            num_read += 1;

            if num_read > 5 {
                return Err(ProtocolError::VarIntTooBig);
            }

            if b & 0x80 == 0 {
                break;
            }
        }

        Ok(result)
    }
}

fn read_varint_from_slice(data: &[u8], pos: &mut usize) -> Result<i32, ProtocolError> {
    let mut num_read = 0;
    let mut result = 0i32;

    loop {
        if *pos >= data.len() {
            return Err(ProtocolError::UnexpectedEof);
        }

        let byte = data[*pos];
        *pos += 1;

        result |= ((byte & 0x7F) as i32) << (7 * num_read);
        num_read += 1;

        if num_read > 5 {
            return Err(ProtocolError::VarIntTooBig);
        }

        if byte & 0x80 == 0 {
            break;
        }
    }

    Ok(result)
}
