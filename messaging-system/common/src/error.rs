use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, MsgSystemError>;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum MsgSystemError {
    #[error("Failed to deserialize the message")]
    DeserializationFailed,
    #[error("Cannot read from TCP stream")]
    ReadingFromTCPStreamFailed,
    #[error("Failed to serialize the message")]
    SerializationFailed,
    #[error("Failed to construct message. Invalid string provided: {provided_str:?}")]
    CannotConstructMessage { provided_str: String },
    #[error("Failed to construct file data from file path: {file_path:?}")]
    CannotCreateFileData { file_path: String },
    #[error("Server configuration not provided")]
    ServerConfigurationNotProvided,
    #[error("Cannot derive socket address from: {config:?}")]
    CannotDeriveSocketAddress { config: String },
    #[error("Failed to write a file to directory: {output_directory:?}")]
    CannotWriteFile { output_directory: String },
}
