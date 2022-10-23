use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::Read;

const LINE_LENGTH: u8 = 16;

#[derive(Debug)]
#[non_exhaustive]
pub enum RxdError {
    CantOpenFile,
    CantReadFile,
    UnhandledReader,
}

pub enum Reader {
    FileHandle(String),
    StdIo,
}

impl Reader {
    pub fn read(&mut self) -> Result<Vec<u8>, RxdError> {
        match self {
            Reader::FileHandle(path) => {
                if let Ok(mut f) = File::open(&path) {
                    let mut buffer = Vec::new();
                    match f.read_to_end(&mut buffer) {
                        Ok(_) => Ok(buffer),
                        Err(_) => Err(RxdError::CantReadFile),
                    }
                } else {
                    Err(RxdError::CantOpenFile)
                }
            }
            _ => Err(RxdError::UnhandledReader),
        }
    }
}

// mod formatter {}
// mod writer {}

#[derive(Debug)]
pub struct Runner {
    bytes: Vec<Byte>,
}

impl Runner {
    pub fn new(mut reader: Reader) -> Self {
        Self {
            bytes: reader
                .read()
                .unwrap()
                .iter()
                .map(|byte| Byte::new(*byte))
                .collect(),
        }
    }

    pub fn print_lines(&self) {
        let lines = self
            .bytes
            .chunks(LINE_LENGTH as usize)
            .map(|b| Vec::from(b))
            .collect::<Vec<Vec<Byte>>>();
        for (i, line) in lines.iter().enumerate() {
            println!(
                "{:0>8} | {: <21} | {}",
                i,
                line.iter()
                    .map(|b| b.to_hex_string())
                    .collect::<Vec<String>>()
                    .chunks(2)
                    .map(|b2| b2.join(""))
                    .collect::<Vec<String>>()
                    .join(" "),
                line.iter()
                    .map(|b| b.to_ascii())
                    .collect::<Vec<String>>()
                    .join("")
            )
        }
    }

    pub fn test_hex(&self) {
        let hex_lines = self
            .bytes
            .chunks(LINE_LENGTH as usize)
            .map(|chunk| {
                chunk
                    .iter()
                    .map(|b| b.to_hex_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>();

        println!("as hex: {:?}", &hex_lines);
    }

    pub fn test_ascii(&self) {
        let ascii_lines = self
            .bytes
            .chunks(LINE_LENGTH as usize)
            .map(|chunk| {
                chunk
                    .iter()
                    .map(|b| String::from(b.to_ascii()))
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>();

        println!("as hex: {:?}", &ascii_lines);
    }
}

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
