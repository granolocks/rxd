
#[derive(Debug, Clone, Copy)]
pub struct Byte {
    byte: u8,
}

impl Byte {
    pub fn new(byte: u8) -> Self {
        Self { byte }
    }

    pub fn to_ascii(&self) -> String {
        // printable ascii range...
        if (self.byte >= 0x20 && self.byte <= 0x7E) || self.byte >= 0xA1 {
            String::from(self.byte as char)
        } else {
            String::from('.')
        }
    }

    pub fn to_hex_string(&self) -> String {
        format!("{:02x}", self.byte)
    }
}