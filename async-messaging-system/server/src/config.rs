use std::{
    fmt::{Display, Formatter},
    net::SocketAddr,
};

use anyhow::{anyhow, Result};
use log::{trace, warn};

pub struct ServerConfig {
    hostname: String,
    port: String,
}

impl ServerConfig {
    pub fn new(hostname: &str, port: &str) -> Self {
        trace!(
            "Creating server config with hostname: {}, port: {}",
            hostname,
            port
        );
        Self {
            hostname: hostname.to_string(),
            port: port.to_string(),
        }
    }

    pub fn from_args(args: &[String]) -> Result<Self> {
        if args.len() != 3 {
            warn!(
                "Not provided server configuration within arguments: {:?}",
                args
            );
            return Err(anyhow!("The server configuration not provided!"));
        }
        Ok(Self::new(&args[1], &args[2]))
    }

    pub fn to_socket_address(&self) -> Result<SocketAddr> {
        let socket_address = self.to_string().parse()?;
        Ok(socket_address)
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
