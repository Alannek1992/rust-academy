use std::{error::Error as StdError, str::FromStr};

use crate::{error::Error, util};

use super::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    File(Vec<u8>),
    Image(Vec<u8>),
    OtherText(String),
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
