use std::{
    error::Error as StdError,
    fmt::{Display, Formatter},
};

pub type Result<T> = std::result::Result<T, Box<dyn StdError>>;

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn new(message: &str) -> Box<Self> {
        Box::new(Self {
            message: message.to_string(),
        })
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl StdError for Error {}
