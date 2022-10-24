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

#[cfg(test)]
mod test {
    use super::*;
    use core::iter::Iterator;

    #[test]
    fn test_byte_to_hext_string() {
        let b0 = Byte::new(0x00);
        assert_eq!(b0.to_hex_string(), String::from("00"));

        let b255 = Byte::new(0xFF);
        assert_eq!(b255.to_hex_string(), String::from("ff"));
    }

    fn range_to_ascii<I: Iterator<Item = u8> + ExactSizeIterator>(r: I) -> (usize, String) {
        let l = r.len();
        let s = r
            .map(|b| Byte::new(b).to_ascii())
            .collect::<Vec<String>>()
            .join("");
        (l, s)
    }

    #[test]
    fn test_byte_to_ascii() {
        let non_printable_1 = 0x00u8..0x20u8;
        let non_printable_2 = 0x7fu8..0xa1u8;
        let printable_1 = 0x20u8..=0x7eu8;
        let printable_2 = 0xa1u8..=0xff;

        let np1 = range_to_ascii(non_printable_1);
        assert_eq!(String::from(".".repeat(np1.0)), np1.1);

        let np2 = range_to_ascii(non_printable_2);
        assert_eq!(String::from(".".repeat(np2.0)), np2.1);

        let p1 = range_to_ascii(printable_1);
        let p1s = String::from(" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~");
        assert_eq!(p1s, p1.1);

        let p2 = range_to_ascii(printable_2);
        let p2s = String::from("¡¢£¤¥¦§¨©ª«¬\u{ad}®¯°±²³´µ¶·¸¹º»¼½¾¿ÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓÔÕÖ×ØÙÚÛÜÝÞßàáâãäåæçèéêëìíîïðñòóôõö÷øùúûüýþÿ");
        assert_eq!(p2s, p2.1);
    }
}
