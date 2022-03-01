pub struct Buffer {
    buf: Vec<u8>,
    pos: usize,
}

impl Buffer {
    /// Instantiates a buffer from a hex-encoded string.
    pub fn from_hex_string(mut s: &str) -> Self {
        let mut buf = Vec::with_capacity(s.len());
        loop {
            buf.push(u8::from_str_radix(&s[0..2], 16).unwrap());
            s = &s[2..];

            // We could discard trailing zeros.
            if s.len() <= 1 {
                break;
            }
        }
        Buffer { buf, pos: 0 }
    }

    /// Read a single byte and advance the buffer by eight bits.
    /// It is a shorthand for a `buffer.read_bits(8)` call.
    pub fn read_byte(&mut self) -> Option<u8> {
        self.read_bits(8)
    }

    /// Reads and returns `n` bits from the buffer, advancing the
    /// position by the same amount of bits. Returns `None` if the
    /// there are not enough bits to return.
    pub fn read_bits(&mut self, n: u8) -> Option<u8> {
        assert!(n <= 8 && n > 0);
        let bytes_offset = self.pos / 8;
        let bits_offset = self.pos % 8;

        // Check the read will not overflow the buffer.
        if self.pos + (n as usize) > self.buf.len() * 8 {
            return None;
        }

        let remaining_bits = (8 - bits_offset) as u8;
        if n <= remaining_bits {
            // We can read only from the current byte.
            let bits = self.buf[bytes_offset] & ((2_u16.pow(remaining_bits as u32) - 1) as u8);
            let bits = bits >> (remaining_bits - n);
            self.pos += n as usize;
            Some(bits)
        } else {
            // We must read from both the current and the next byte.
            let first = self.buf[bytes_offset] & ((2_u16.pow(remaining_bits as u32) - 1) as u8);
            let first = first << (n - remaining_bits);
            let second = self.buf[bytes_offset + 1] >> 8 - (n - remaining_bits);
            self.pos += n as usize;
            Some(first | second)
        }
    }
}

#[cfg(test)]
mod buffer_tests {
    #[test]
    fn buffer_read() {
        let mut buffer = super::Buffer::from_hex_string("af00af");
        assert_eq!(buffer.read_bits(3), Some(0b101));
        assert_eq!(buffer.read_bits(3), Some(0b011));
        assert_eq!(buffer.read_bits(2), Some(0b11));
        assert_eq!(buffer.read_bits(8), Some(0b0));
        assert_eq!(buffer.read_bits(7), Some(0b1010111));
        assert_eq!(buffer.read_bits(2), None);
        assert_eq!(buffer.read_bits(1), Some(0b1));
        assert_eq!(buffer.read_bits(1), None);
    }

    #[test]
    fn buffer_read_bounds() {
        let mut buffer = super::Buffer::from_hex_string("af0056");
        assert_eq!(buffer.read_bits(6), Some(0b101011));
        assert_eq!(buffer.read_bits(3), Some(0b110));
        assert_eq!(buffer.read_bits(7), Some(0b0));
        assert_eq!(buffer.read_bits(3), Some(0b010));
        assert_eq!(buffer.read_bits(5), Some(0b10110));
        assert_eq!(buffer.read_bits(1), None);
    }
}
