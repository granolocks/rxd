use crate::reader::Reader;
use crate::byte::Byte;

const LINE_LENGTH: usize = 16;

#[derive(Debug)]
pub struct Runner {
    bytes: Vec<Byte>,
}

impl Runner {
    pub fn new(file_path: Option<String>) -> Self {
        let mut reader = Reader::new(file_path);
        let bytes = reader.read().unwrap();

        Self {
            bytes,
        }
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

fn bytes_to_hex_string(bytes: &Vec<Byte>) -> String {
    bytes
        .iter()
        .map(|b| b.to_hex_string())
        .collect::<Vec<String>>()
        .chunks(2)
        .map(|b2| b2.join(""))
        .collect::<Vec<String>>()
        .join(" ")
}

fn bytes_to_ascii_string(bytes: &Vec<Byte>) -> String {
    bytes
        .iter()
        .map(|b| b.to_ascii())
        .collect::<Vec<String>>()
        .join("")
}