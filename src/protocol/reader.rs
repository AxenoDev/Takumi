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

        let byte_len = len * 2;

        if self.pos + byte_len > self.data.len() {
            return Err("string out of bounds");
        }

        let bytes = &self.data[self.pos..self.pos + byte_len];
        self.pos += byte_len;

        let utf16: Vec<u16> = bytes
            .chunks_exact(2)
            .map(|pair| u16::from_be_bytes([pair[0], pair[1]]))
            .collect();

        String::from_utf16(&utf16).map_err(|_| "invalid utf-16")
    }

    pub fn read_u16(&mut self) -> Result<u16, &'static str> {
        let value = u16::from_be_bytes([self.data[self.pos], self.data[self.pos + 1]]);

        self.pos += 2;

        Ok(value)
    }
}
