use std::fmt::{Display, Formatter};

use crate::{
    error::{Error, Result},
    util,
};

pub struct ServerConfig {
    hostname: String,
    port: String,
}

impl ServerConfig {
    pub fn new(hostname: &str, port: &str) -> Self {
        Self {
            hostname: hostname.to_string(),
            port: port.to_string(),
        }
    }

    pub fn from_args(args: &[String]) -> Result<Self> {
        if args.len() != 3 {
            return Err(Error::new("The server configuration not provided!"));
        }

        if !util::is_valid_hostname(&args[1]) {
            return Err(Error::new("The invalid hostname provided!"));
        }

        if !util::is_port_valid(&args[2]) {
            return Err(Error::new("The invalid port provided!"));
        }

        Ok(Self::new(&args[1], &args[2]))
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self::new("127.0.0.1", "11111")
    }
}

impl Display for ServerConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.hostname, self.port)
    }
}
