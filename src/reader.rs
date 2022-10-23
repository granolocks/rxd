use std::fs::File;
// use std::io;
// use std::io::prelude::*;
use std::io::Read;
use crate::rxd_error::RxdError;
use crate::byte::Byte;

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
