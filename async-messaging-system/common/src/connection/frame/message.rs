use std::path::Path;

use anyhow::anyhow;
use log::trace;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::{fs::File, io::AsyncReadExt};

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    File(FileData),
    Image(FileData),
    Text(String),
}

#[derive(Error, Debug)]
pub enum MessageCreationError {
    #[error("Failed to construct message. Invalid string provided: {provided_str:?}")]
    InvalidStringProvided {
        provided_str: String,
        #[source]
        underlying_error: anyhow::Error,
    },
    #[error("Failed to construct file data from file path: {file_path:?}")]
    CannotCreateFileData {
        file_path: String,
        #[source]
        underlying_error: anyhow::Error,
    },
}

pub type Result<T> = std::result::Result<T, MessageCreationError>;

impl Message {
    pub async fn from_str(s: &str) -> Result<Self> {
        trace!("Constructing message from provided input: {}", s);
        if s.starts_with(".file ") || s.starts_with(".image ") {
            let (kind, path) = s.split_at(6); // ".file " or ".image "
            let file_data = FileData::from_file_path(path.trim()).await?;

            match kind {
                ".file " => Ok(Self::File(file_data)),
                ".image " => Ok(Self::Image(file_data)),
                _ => Err(MessageCreationError::InvalidStringProvided {
                    provided_str: s.to_string(),
                    underlying_error: anyhow!("Failed to parse provided string: {}", s),
                }),
            }
        } else {
            Ok(Self::Text(s.to_string()))
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
    pub async fn from_file_path(file_path: &str) -> Result<Self> {
        trace!("Creating file data from provided file path: {}", file_path);
        let mut file = File::open(file_path).await.map_err(|_| {
            MessageCreationError::CannotCreateFileData {
                file_path: file_path.to_string(),
                underlying_error: anyhow!("Cannot open file at path: {}", file_path),
            }
        })?;
        let file_name = match Path::new(file_path).file_name() {
            Some(name) => name.to_string_lossy().to_string(),
            None => String::from("Unknown"),
        };

        let file_extension = Self::get_file_extension(file_path);
        let file_size = file
            .metadata()
            .await
            .map_err(|_| MessageCreationError::CannotCreateFileData {
                file_path: file_path.to_string(),
                underlying_error: anyhow!("Cannot read file metadata at path: {}", file_path),
            })?
            .len() as usize;
        let mut buffer = Vec::with_capacity(file_size);

        file.read_to_end(&mut buffer).await.map_err(|_| {
            MessageCreationError::CannotCreateFileData {
                file_path: file_path.to_string(),
                underlying_error: anyhow!("Cannot read file content at path: {}", file_path),
            }
        })?;

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
