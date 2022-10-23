use std::fs::File;
// use std::io;
// use std::io::prelude::*;
use std::io::Read;

const LINE_LENGTH: usize = 16;

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
    pub fn new(file_path: Option<String>) -> Self {
        match file_path {
            Some(path) => Reader::FileHandle(path),
            None => Reader::StdIo,
        }
    }

    pub fn read(&mut self) -> Result<Vec<Byte>, RxdError> {
        match self {
            Reader::FileHandle(path) => {
                if let Ok(mut f) = File::open(&path) {
                    let mut buffer = Vec::new();
                    match f.read_to_end(&mut buffer) {
                        Ok(_) => Ok(buffer.iter().map(|byte| Byte::new(*byte)).collect()),
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
