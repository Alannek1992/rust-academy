use std::{
    error::Error as StdError,
    io::{Read, Write},
    str::FromStr,
};

use crate::{error::Error, util};

use super::error::Result;
use serde::{Deserialize, Serialize};

const HEADER_SIZE: usize = 4;

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageEnvelope {
    pub from_user: Username,
    pub content: Message,
}

impl MessageEnvelope {
    pub fn new(from_user: &str, content: Message) -> Self {
        Self {
            from_user: Username::from(from_user),
            content,
        }
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Self> {
        match bincode::deserialize(bytes) {
            Ok(msg) => Ok(msg),
            Err(e) => Err(Error::new(&format!("The deserialization failed: {}", e))),
        }
    }

    pub fn read_frame<S: Read + Write>(stream: &mut S) -> Result<Vec<u8>> {
        let mut header_bytes = [0; HEADER_SIZE];
        stream.read_exact(&mut header_bytes)?;

        let size = u32::from_le_bytes(header_bytes);
        let mut buffer = vec![0; size as usize];
        stream.read_exact(&mut buffer)?;

        Ok(buffer)
    }

    pub fn serialize(&self) -> Result<Vec<u8>> {
        match bincode::serialize(self) {
            Ok(content_bytes) => {
                let mut result = vec![0; HEADER_SIZE];
                let size = content_bytes.len() as u32;
                result.copy_from_slice(&size.to_le_bytes());
                result.extend(content_bytes);
                Ok(result)
            }
            Err(e) => Err(Error::new(&format!("The serialization failed: {}", e))),
        }
    }
}

pub type Username = String;

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    File(Vec<u8>),
    Image(Vec<u8>),
    OtherText(String),
    Login,
    Exit,
}

impl FromStr for Message {
    type Err = Box<dyn StdError>;

    fn from_str(s: &str) -> Result<Self> {
        if s.starts_with(".file") || s.starts_with(".image") {
            let input: Vec<&str> = s.split_whitespace().collect();
            let (kind, path) = input
                .split_first()
                .ok_or(Error::new("The invalid path provided"))?;
            let binary = util::read_from_file_path(&path.join(" "))?;
            match *kind {
                ".file" => Ok(Self::File(binary)),
                _ => Ok(Self::Image(binary)),
            }
        } else if s.starts_with(".quit") {
            Ok(Self::Exit)
        } else {
            Ok(Self::OtherText(s.to_string()))
        }
    }
}
