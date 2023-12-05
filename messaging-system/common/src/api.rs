use std::{
    error::Error as StdError,
    fmt::Display,
    fs::File,
    io::{self, Read, Write},
    path::Path,
    str::FromStr,
    thread,
    time::Duration,
};

use crate::error::Error;

use super::error::Result;
use log::{trace, warn};
use serde::{Deserialize, Serialize};

const HEADER_SIZE: usize = 4;
const CHUNK_SIZE: usize = 4096;

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageEnvelope {
    pub from_user: Username,
    pub content: Message,
}

impl MessageEnvelope {
    pub fn new(from_user: &str, content: Message) -> Self {
        let msg_env = Self {
            from_user: Username::from(from_user),
            content,
        };

        trace!("Creating new message envelope: {}", msg_env);
        msg_env
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
        let mut buffer = Vec::with_capacity(size as usize);

        let mut remaining_bytes = size as usize;
        let mut chunk = vec![0; CHUNK_SIZE];

        while remaining_bytes > 0 {
            let bytes_to_read = std::cmp::min(remaining_bytes, CHUNK_SIZE);

            match stream.read(&mut chunk[0..bytes_to_read]) {
                Ok(n) if n > 0 => {
                    buffer.extend_from_slice(&chunk[0..n]);
                    remaining_bytes -= n;
                }
                Ok(_) => {
                    // Stream is closed or no more data is expected
                    break;
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // Handle non-blocking scenario, wait and retry
                    warn!("Handling blocking operation. The tread will be put to sleep for 100ms");
                    thread::sleep(Duration::from_millis(100));
                }
                Err(e) => return Err(Error::new(&format!("{}", e))),
            }
        }

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

impl Display for MessageEnvelope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "From user: {}. Content: {:?}",
            self.from_user, self.content
        )
    }
}

pub type Username = String;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Message {
    File(FileData),
    Image(FileData),
    OtherText(String),
    Login,
    Exit,
}

impl FromStr for Message {
    type Err = Box<dyn StdError>;

    fn from_str(s: &str) -> Result<Self> {
        trace!("Constructing message from provided input: {}", s);
        if s.starts_with(".file") || s.starts_with(".image") {
            let input: Vec<&str> = s.split_whitespace().collect();
            let (kind, path) = input
                .split_first()
                .ok_or(Error::new("The invalid path provided"))?;
            let file_data = FileData::from_file_path(&path.join(" "))?;
            match *kind {
                ".file" => Ok(Self::File(file_data)),
                _ => Ok(Self::Image(file_data)),
            }
        } else if s.starts_with(".quit") {
            Ok(Self::Exit)
        } else {
            Ok(Self::OtherText(s.to_string()))
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct FileData {
    pub bytes: Vec<u8>,
    pub file_name: String,
    pub file_extension: Option<String>,
    pub file_size: usize,
}

impl FileData {
    pub fn from_file_path(file_path: &str) -> Result<Self> {
        trace!("Creating file data from provided file path: {}", file_path);
        let file = File::open(file_path)?;
        let file_name = match Path::new(file_path).file_name() {
            Some(name) => name.to_string_lossy().to_string(),
            None => String::from("Unknown"),
        };

        let file_extension = Self::get_file_extension(file_path);
        let file_size = file.metadata()?.len() as usize;
        let mut buffer = Vec::with_capacity(file_size);

        let bytes_read = file.take(file_size as u64).read_to_end(&mut buffer)?;

        if bytes_read != file_size {
            return Err(Error::new("Failed to read the entire file"));
        }

        Ok(Self {
            bytes: buffer,
            file_name,
            file_extension,
            file_size,
        })
    }

    fn get_file_extension(file_path: &str) -> Option<String> {
        Path::new(file_path)
            .extension()
            .and_then(|ext| ext.to_str().map(String::from))
    }
}
