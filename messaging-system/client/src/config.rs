use std::fmt::Display;
use std::net::SocketAddr;

use common::config::ServerConfig;
use common::error::Result;

pub struct ClientConfig {
    server_config: ServerConfig,
}

impl ClientConfig {
    pub fn new(server_config: ServerConfig) -> Self {
        Self { server_config }
    }

    pub fn from_args(args: &[String]) -> Result<Self> {
        let server_config = ServerConfig::from_args(args)?;
        Ok(Self::new(server_config))
    }

    pub fn to_socket_address(&self) -> Result<SocketAddr> {
        self.server_config.to_socket_address()
    }
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            server_config: ServerConfig::default(),
        }
    }
}

impl Display for ClientConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.server_config)
    }
}
