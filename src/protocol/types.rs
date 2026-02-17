use anyhow::bail;
use bytes::{Buf, BufMut};

pub fn read_varint(buf: &mut impl Buf) -> anyhow::Result<i32> {
    let mut result = 0i32;
    let mut shift = 0u32;
    loop {
        if !buf.has_remaining() {
            bail!("Unexpected end of buffer while reading VarInt");
        }
        let byte = buf.get_u8();
        result |= ((byte & 0x7F) as i32) << shift;
        if byte & 0x80 == 0 {
            break;
        }
        shift += 7;
        if shift >= 35 {
            bail!("VarInt is too big");
        }
    }
    Ok(result)
}

pub fn write_varint(buf: &mut impl BufMut, mut value: i32) {
    loop {
        let mut byte = (value & 0x7F) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        buf.put_u8(byte);
        if value == 0 {
            break;
        }
    }
}

pub fn varint_size(mut value: i32) -> usize {
    let mut size = 0;
    loop {
        size += 1;
        value >>= 7;
        if value == 0 {
            break;
        }
    }
    size
}

pub fn read_varlong(buf: &mut impl Buf) -> anyhow::Result<i64> {
    let mut result = 0i64;
    let mut shift = 0u32;
    loop {
        if !buf.has_remaining() {
            bail!("Unexpected end of buffer while reading VarLong");
        }
        let byte = buf.get_u8();
        result |= ((byte & 0x7F) as i64) << shift;
        if byte & 0x80 == 0 {
            break;
        }
        shift += 7;
        if shift >= 70 {
            bail!("VarLong is too big");
        }
    }
    Ok(result)
}

pub fn write_varlong(buf: &mut impl BufMut, mut value: i64) {
    loop {
        let mut byte = (value & 0x7F) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        buf.put_u8(byte);
        if value == 0 {
            break;
        }
    }
}

pub fn read_string(buf: &mut impl Buf) -> anyhow::Result<String> {
    let length = read_varint(buf)? as usize;
    if buf.remaining() < length {
        bail!("Unexpected end of buffer while reading string");
    }
    let bytes = buf.copy_to_bytes(length);
    Ok(String::from_utf8(bytes.to_vec())?)
}

pub fn write_string(buf: &mut impl BufMut, value: &str) {
    let bytes = value.as_bytes();
    write_varint(buf, bytes.len() as i32);
    buf.put_slice(bytes);
}

pub fn read_byte_array(buf: &mut impl Buf) -> anyhow::Result<Vec<u8>> {
    let length = read_varint(buf)? as usize;
    if buf.remaining() < length {
        bail!("Unexpected end of buffer while reading byte array");
    }
    let mut bytes = vec![0u8; length];
    buf.copy_to_slice(&mut bytes);
    Ok(bytes)
}

pub fn write_byte_array(buf: &mut impl BufMut, value: &[u8]) {
    write_varint(buf, value.len() as i32);
    buf.put_slice(value);
}

pub fn read_uuid(buf: &mut impl Buf) -> anyhow::Result<uuid::Uuid> {
    if buf.remaining() < 16 {
        bail!("Unexpected end of buffer while reading UUID");
    }
    let mut bytes = [0u8; 16];
    buf.copy_to_slice(&mut bytes);
    Ok(uuid::Uuid::from_bytes(bytes))
}

pub fn write_uuid(buf: &mut impl BufMut, value: &uuid::Uuid) {
    buf.put_slice(value.as_bytes());
}

pub fn read_bool(buf: &mut impl Buf) -> anyhow::Result<bool> {
    if !buf.has_remaining() {
        bail!("Unexpected end of buffer while reading bool");
    }
    Ok(buf.get_u8() != 0)
}

pub fn write_bool(buf: &mut impl BufMut, value: bool) {
    buf.put_u8(if value { 1 } else { 0 });
}

pub fn read_remaining(buf: &mut impl Buf) -> Vec<u8> {
    let mut bytes = vec![0u8; buf.remaining()];
    buf.copy_to_slice(&mut bytes);
    bytes
}
