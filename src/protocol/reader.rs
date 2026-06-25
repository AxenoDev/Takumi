pub struct PacketReader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> PacketReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    pub fn read_varint(&mut self) -> Result<i32, &'static str> {
        let mut num_read = 0;
        let mut result = 0i32;

        loop {
            if self.pos >= self.data.len() {
                return Err("VarInt: unexpected end of data");
            }

            let byte = self.data[self.pos];
            self.pos += 1;

            let value = (byte & 0x7F) as i32;
            result |= value << (7 * num_read);
            num_read += 1;

            if num_read > 5 {
                return Err("VarInt too big");
            }

            if (byte & 0x80) == 0 {
                break;
            }
        }

        Ok(result)
    }

    pub fn read_string(&mut self) -> Result<String, &'static str> {
        let len = self.read_varint()? as usize;

        if self.pos + len > self.data.len() {
            return Err("string out of bounds");
        }

        let bytes = &self.data[self.pos..self.pos + len];
        self.pos += len;

        String::from_utf8(bytes.to_vec()).map_err(|_| "invalid utf-8")
    }

    pub fn read_u16(&mut self) -> Result<u16, &'static str> {
        if self.pos + 2 > self.data.len() {
            return Err("u16: unexpected end of data");
        }

        let value = u16::from_be_bytes([self.data[self.pos], self.data[self.pos + 1]]);
        self.pos += 2;

        Ok(value)
    }
}
