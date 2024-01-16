use anyhow::anyhow;
use bincode::{deserialize, serialize, ErrorKind};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::error::Result;

use self::{auth_message::AuthMessage, message::Message};

pub mod auth_message;
pub mod message;

#[derive(Debug, Serialize, Deserialize)]
pub enum Frame {
    Msg(Message),
    Auth(AuthMessage),
}

#[derive(Error, Debug)]
pub enum FrameDeserializationError {
    #[error("Incomplete buffer of bytes")]
    Incomplete,
    #[error("Failed to parse frame")]
    Other {
        #[source]
        underlying_error: anyhow::Error,
    },
}

impl Frame {
    // deserializes into frame from the provided bytes
    pub fn deserialize(buf: &mut [u8]) -> std::result::Result<Self, FrameDeserializationError> {
        match deserialize(buf) {
            Ok(frame) => Ok(frame),
            Err(err) => match err.as_ref() {
                ErrorKind::SizeLimit => Err(FrameDeserializationError::Incomplete),
                _ => Err(FrameDeserializationError::Other {
                    underlying_error: anyhow!(
                        "Following error occured when parsing frame: {}",
                        err
                    ),
                }),
            },
        }
    }
    // serializes into bytes
    pub fn serialize(&self) -> Result<Vec<u8>> {
        match serialize(self) {
            Ok(bytes) => Ok(bytes),
            Err(e) => Err(e),
        }
    }
}
