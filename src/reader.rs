use crate::byte::Byte;
use crate::rxd_error::RxdError;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read};

#[derive(Debug)]
#[non_exhaustive]
pub enum Reader {
    FileHandle(String),
    StdIn,
}

impl Reader {
    pub fn new(file_path: Option<String>) -> Self {
        match file_path {
            Some(path) => Reader::FileHandle(path),
            None => Reader::StdIn,
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

            Reader::StdIn => {
                let mut buf_reader = BufReader::new(io::stdin());
                let mut buffer = Vec::new();
                match buf_reader.read_to_end(&mut buffer) {
                    Ok(_) => Ok(buffer.iter().map(|byte| Byte::new(*byte)).collect()),
                    Err(_) => Err(RxdError::CantReadStdIn),
                }
            }
        }
    }
}
