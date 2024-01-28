use anyhow::{anyhow, Result};
use bincode::{deserialize, serialize, ErrorKind};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use self::{auth::{Auth, AuthToken}, message::Message};

pub mod auth;
pub mod message;

#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    jwt_token: Option<AuthToken>,
}

impl Header {
    pub fn new() -> Self {
        Self { jwt_token: None }
    }
    pub fn with_token(jwt_token: AuthToken) -> Self {
        Self {
            jwt_token: Some(jwt_token),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Payload {
    Msg(Message),
    Auth(Auth),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Frame {
    pub header: Header,
    pub payload: Payload,
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
    pub fn new(header: Header, payload: Payload) -> Self {
        Self { header, payload }
    }
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
            Err(e) => Err(anyhow!(e)),
        }
    }
}
