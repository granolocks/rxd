use crate::byte::Byte;
use crate::reader::Reader;

const LINE_LENGTH: usize = 16;

#[derive(Debug)]
pub struct Runner {
    bytes: Vec<Byte>,
}

impl Runner {
    pub fn new(file_path: Option<String>) -> Self {
        let mut reader = Reader::new(file_path);
        let bytes = reader.read().unwrap();

        Self { bytes }
    }

    pub fn print_lines(&self) {
        let lines = self
            .bytes
            .chunks(LINE_LENGTH)
            .map(|b| Vec::from(b))
            .collect::<Vec<Vec<Byte>>>();

        for (i, line) in lines.iter().enumerate() {
            println!(
                "{:0>8} | {: <39} | {}",
                i,
                bytes_to_hex_string(&line),
                bytes_to_ascii_string(&line)
            )
        }
    }
}

pub fn bytes_to_hex_string(bytes: &Vec<Byte>) -> String {
    bytes
        .iter()
        .map(|b| b.to_hex_string())
        .collect::<Vec<String>>()
        .chunks(2)
        .map(|b2| b2.join(""))
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn bytes_to_ascii_string(bytes: &Vec<Byte>) -> String {
    bytes
        .iter()
        .map(|b| b.to_ascii())
        .collect::<Vec<String>>()
        .join("")
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_bytes_to_ascii_string() {
        let ascii_range = (0u8..255u8)
            .map(|b| Byte::new(b).to_ascii())
            .collect::<Vec<String>>()
            .join((""));
        assert_eq!(ascii_range, "................................ !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~..................................¡¢£¤¥¦§¨©ª«¬\u{ad}®¯°±²³´µ¶·¸¹º»¼½¾¿ÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓÔÕÖ×ØÙÚÛÜÝÞßàáâãäåæçèéêëìíîïðñòóôõö÷øùúûüýþ")
    }

    #[test]
    fn test_bytes_to_hex_string() {
        let v_empty: Vec<Byte> = vec![];
        assert_eq!(String::from(""), bytes_to_hex_string(&v_empty));

        let v0 = vec![Byte::new(0)];
        assert_eq!(String::from("00"), bytes_to_hex_string(&v0));

        let v1 = vec![Byte::new(0), Byte::new(255)];
        assert_eq!(String::from("00ff"), bytes_to_hex_string(&v1));

        let v2 = vec![
            Byte::new(0xde),
            Byte::new(0xad),
            Byte::new(0xbe),
            Byte::new(0xef),
        ];
        assert_eq!(String::from("dead beef"), bytes_to_hex_string(&v2));
    }
}
