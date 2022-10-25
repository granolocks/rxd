// non printable
const ASCII_NP_1_L: u8 = 0x00u8;
const ASCII_NP_1_U: u8 = 0x1Fu8;

// printable
const ASCII_P_1_L: u8 = 0x20u8;
const ASCII_P_1_U: u8 = 0x7eu8;

// non printable
const ASCII_NP_2_L: u8 = 0x7fu8;
const ASCII_NP_2_U: u8 = 0xa0u8;

//printable
const ASCII_P_2_L: u8 = 0xa1u8;
const ASCII_P_2_U: u8 = 0xff;

#[derive(Debug, Clone)]
enum AsciiType {
    Printable,
    NonPrintable,
}

impl AsciiType {
    fn categorize(byte: u8) -> AsciiType {
        match byte {
            ASCII_NP_1_L..=ASCII_NP_1_U => AsciiType::NonPrintable,
            ASCII_P_1_L..=ASCII_P_1_U => AsciiType::Printable,
            ASCII_NP_2_L..=ASCII_NP_2_U => AsciiType::NonPrintable,
            ASCII_P_2_L..=ASCII_P_2_U => AsciiType::Printable,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Byte {
    byte: u8,
    ascii_type: AsciiType,
}

impl Byte {
    pub fn new(byte: u8) -> Self {
        Self {
            byte,
            ascii_type: AsciiType::categorize(byte),
        }
    }

    pub fn to_ascii(&self) -> String {
        let ascii_char = match self.ascii_type {
            AsciiType::Printable => self.byte as char,
            AsciiType::NonPrintable => '.',
        };
        String::from(ascii_char)
    }

    pub fn to_colored_hex_string(&self) -> String {
        match self.ascii_type {
            AsciiType::Printable => format!("\x1b[31m{:02x}\x1b[0m", self.byte),
            AsciiType::NonPrintable => format!("{:02x}", self.byte),
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
        let non_printable_1 = ASCII_NP_1_L..ASCII_NP_1_U;
        let non_printable_2 = ASCII_NP_2_L..ASCII_NP_2_U;
        let printable_1 = ASCII_P_1_L..=ASCII_P_1_U;
        let printable_2 = ASCII_P_2_L..=ASCII_P_2_U;

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
