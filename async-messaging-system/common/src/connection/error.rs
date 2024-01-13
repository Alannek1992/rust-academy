use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConnectionError {}

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub type Result<T> = std::result::Result<T, Error>;
