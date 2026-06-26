use crate::error::ProtocolError;

pub struct PacketReader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> PacketReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    pub fn remaining(&self) -> usize {
        self.data.len().saturating_sub(self.pos)
    }

    pub fn read_varint(&mut self) -> Result<i32, ProtocolError> {
        let mut num_read = 0;
        let mut result = 0i32;

        loop {
            if self.pos >= self.data.len() {
                return Err(ProtocolError::UnexpectedEof);
            }

            let byte = self.data[self.pos];
            self.pos += 1;

            let value = (byte & 0x7F) as i32;
            result |= value << (7 * num_read);
            num_read += 1;

            if num_read > 5 {
                return Err(ProtocolError::VarIntTooBig);
            }

            if (byte & 0x80) == 0 {
                break;
            }
        }

        Ok(result)
    }

    pub fn read_string(&mut self) -> Result<String, ProtocolError> {
        let len = self.read_varint()? as usize;

        if self.pos + len > self.data.len() {
            return Err(ProtocolError::UnexpectedEof);
        }

        let bytes = &self.data[self.pos..self.pos + len];
        self.pos += len;

        String::from_utf8(bytes.to_vec()).map_err(|_| ProtocolError::InvalidUtf8)
    }

    pub fn read_u16(&mut self) -> Result<u16, ProtocolError> {
        if self.pos + 2 > self.data.len() {
            return Err(ProtocolError::UnexpectedEof);
        }

        let value = u16::from_be_bytes([self.data[self.pos], self.data[self.pos + 1]]);
        self.pos += 2;

        Ok(value)
    }

    pub fn read_i64(&mut self) -> Result<i64, ProtocolError> {
        if self.pos + 8 > self.data.len() {
            return Err(ProtocolError::UnexpectedEof);
        }

        let value = i64::from_be_bytes([
            self.data[self.pos],
            self.data[self.pos + 1],
            self.data[self.pos + 2],
            self.data[self.pos + 3],
            self.data[self.pos + 4],
            self.data[self.pos + 5],
            self.data[self.pos + 6],
            self.data[self.pos + 7],
        ]);
        self.pos += 8;

        Ok(value)
    }

    pub fn read_uuid(&mut self) -> Result<uuid::Uuid, ProtocolError> {
        if self.pos + 16 > self.data.len() {
            return Err(ProtocolError::UnexpectedEof);
        }

        let bytes = &self.data[self.pos..self.pos + 16];
        self.pos += 16;

        Ok(uuid::Uuid::from_slice(bytes).map_err(|_| ProtocolError::InvalidUuid)?)
    }
}
