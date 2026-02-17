use anyhow::{Ok, bail};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use flate2::{Compression, read::ZlibDecoder, write::ZlibEncoder};
use std::io::{Read, Write};
use tokio_util::codec::{Decoder, Encoder};

use crate::protocol::{
    Packet,
    types::{read_varint, varint_size, write_varint},
};

const MAX_PACKET_SIZE: usize = 2 * 1024 * 1024; // 2 MiB

pub struct MinecraftCodec {
    compression_threshold: i32,
}

impl MinecraftCodec {
    pub fn new() -> Self {
        MinecraftCodec {
            compression_threshold: -1,
        }
    }

    pub fn set_compression_threshold(&mut self, threshold: i32) {
        self.compression_threshold = threshold;
    }

    pub fn compression_threshold(&self) -> i32 {
        self.compression_threshold
    }
}

impl Decoder for MinecraftCodec {
    type Item = Packet;
    type Error = anyhow::Error;

    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let mut peek = src.as_ref();
        let length = match read_varint_peek(peek) {
            Some((len, varint_bytes)) => {
                let total = varint_bytes + len as usize;
                if total > MAX_PACKET_SIZE {
                    bail!(
                        "Packet size {} exceeds maximum of {}",
                        total,
                        MAX_PACKET_SIZE
                    );
                }
                if src.len() < total {
                    return Ok(None);
                }

                // Consume the varint bytes from src
                src.advance(varint_bytes);
                len as usize
            }
            None => return Ok(None),
        };

        let mut payload = src.split_to(length);

        if self.compression_threshold >= 0 {
            // Compressed format: VarInt data_length, then data
            let data_length = read_varint(&mut payload)? as usize;
            if data_length == 0 {
                // Uncompressed packet despite compression being enabled
                let id = read_varint(&mut payload)?;
                return Ok(Some(Packet::new(id, payload.freeze())));
            } else {
                // Decompress the data
                let compressed = payload.to_vec();
                let mut decoder = ZlibDecoder::new(compressed.as_slice());
                let mut decompressed = Vec::with_capacity(data_length);
                decoder.read_to_end(&mut decompressed)?;

                let mut buf = Bytes::from(decompressed);
                let id = read_varint(&mut buf.as_ref().clone())?;
                // Advance the buffer to consume the varint bytes
                let id_size = varint_size(id);
                buf.advance(id_size);
                return Ok(Some(Packet::new(id, buf)));
            }
        }

        // Uncompressed
        let id = read_varint(&mut payload)?;
        Ok(Some(Packet::new(id, payload.freeze())))
    }
}

impl Encoder<Packet> for MinecraftCodec {
    type Error = anyhow::Error;

    fn encode(&mut self, packet: Packet, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let threshold = self.compression_threshold;

        if threshold >= 0 {
            encode_compressed(packet, threshold as usize, dst)
        } else {
            encode_uncompressed(packet, dst)
        }
    }
}

fn encode_uncompressed(packet: Packet, dst: &mut BytesMut) -> anyhow::Result<()> {
    let id_size = varint_size(packet.id);
    let payload_length = id_size + packet.data.len();

    write_varint_to(dst, payload_length as i32);
    write_varint_to(dst, packet.id);
    dst.put_slice(&packet.data[..]);
    Ok(())
}

fn encode_compressed(packet: Packet, threshold: usize, dst: &mut BytesMut) -> anyhow::Result<()> {
    let uncompressed_data = {
        let id_size = varint_size(packet.id);
        let mut buf = BytesMut::with_capacity(id_size + packet.data.len());
        write_varint_to(&mut buf, packet.id);
        buf.put_slice(&packet.data[..]);
        buf.freeze()
    };

    if uncompressed_data.len() < threshold {
        // Send uncompressed: data_length=0 means uncompressed
        let packet_len = varint_size(0) + uncompressed_data.len();
        write_varint_to(dst, packet_len as i32);
        write_varint_to(dst, 0); // data_length = 0 (not compressed)
        dst.put_slice(&uncompressed_data[..]);
    } else {
        // Compress the data
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&uncompressed_data)?;
        let compressed = encoder.finish()?;

        let data_length = uncompressed_data.len();
        let packet_len = varint_size(data_length as i32) + compressed.len();
        write_varint_to(dst, packet_len as i32);
        write_varint_to(dst, data_length as i32);
        dst.put_slice(&compressed);
    }

    Ok(())
}

fn write_varint_to(dst: &mut BytesMut, value: i32) {
    write_varint(dst, value);
}

fn read_varint_peek(buf: &[u8]) -> Option<(i32, usize)> {
    let mut result = 0i32;
    let mut shift = 0u32;
    let mut bytes_read = 0;
    loop {
        if buf.is_empty() {
            return None;
        }
        let byte = buf[bytes_read];
        result |= ((byte & 0x7F) as i32) << shift;
        bytes_read += 1;
        if byte & 0x80 == 0 {
            return Some((result, bytes_read));
        }
        shift += 7;
        if shift >= 35 {
            return None;
        }
    }
}
