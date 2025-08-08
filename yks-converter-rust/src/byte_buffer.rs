use byteorder::{BigEndian, WriteBytesExt};

#[derive(Debug, Clone)]
pub struct ByteBuffer {
    buf: Vec<u8>,
    read_pos: usize,
}

impl ByteBuffer {
    pub fn new() -> Self {
        ByteBuffer {
            buf: Vec::new(),
            read_pos: 0,
        }
    }


    pub fn put_byte(&mut self, value: u8) -> &mut Self {
        self.buf.push(value);
        self
    }

    pub fn put_bytes(&mut self, other: &ByteBuffer) -> &mut Self {
        self.buf.extend(&other.buf);
        self
    }

    pub fn put_u16(&mut self, value: u16) -> &mut Self {
        let mut temp = Vec::new();
        temp.write_u16::<BigEndian>(value).unwrap();
        self.buf.extend(temp);
        self
    }

    pub fn put_u32(&mut self, value: u32) -> &mut Self {
        let mut temp = Vec::new();
        temp.write_u32::<BigEndian>(value).unwrap();
        self.buf.extend(temp);
        self
    }

    pub fn put_string(&mut self, value: &str) -> &mut Self {
        self.buf.extend(value.bytes());
        self
    }

    pub fn put_bytes_array(&mut self, bytes: &[u8]) -> &mut Self {
        self.buf.extend(bytes);
        self
    }

    pub fn get(&mut self) -> u8 {
        if self.read_pos < self.buf.len() {
            let value = self.buf[self.read_pos];
            self.read_pos += 1;
            value
        } else {
            0
        }
    }

    pub fn get_at(&self, index: usize) -> u8 {
        if index < self.buf.len() {
            self.buf[index]
        } else {
            0
        }
    }

    pub fn size(&self) -> usize {
        self.buf.len()
    }

    pub fn clear(&mut self) {
        self.buf.clear();
        self.read_pos = 0;
    }

    pub fn print_hex(&self) {
        println!("ByteBuffer Length: {}, Hex: ", self.buf.len());
        for (i, byte) in self.buf.iter().enumerate() {
            print!("{:02x} ", byte);
            if (i + 1) % 16 == 0 {
                println!();
            }
        }
        println!();
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.buf
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.buf.clone()
    }
}


impl Default for ByteBuffer {
    fn default() -> Self {
        Self::new()
    }
}