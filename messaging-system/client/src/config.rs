use std::fmt::Display;
use std::net::SocketAddr;

use common::config::ServerConfig;
use common::error::Result;
use log::trace;

const DEFAULT_STORAGE_PATH: &str = "./storage";

pub struct ClientConfig {
    server_config: ServerConfig,
    pub storage_path: String,
}

impl ClientConfig {
    pub fn new(server_config: ServerConfig, storage_path: &str) -> Self {
        trace!("The chat content will be stored into: {}", storage_path);
        Self {
            server_config,
            storage_path: storage_path.to_string(),
        }
    }

    pub fn from_args(args: &[String]) -> Result<Self> {
        let server_config = ServerConfig::from_args(args)?;
        let storage_path = if args.len() == 4 {
            &args[1]
        } else {
            DEFAULT_STORAGE_PATH
        };
        Ok(Self::new(server_config, storage_path))
    }

    pub fn to_socket_address(&self) -> Result<SocketAddr> {
        self.server_config.to_socket_address()
    }
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self::new(ServerConfig::default(), DEFAULT_STORAGE_PATH)
    }
}

impl Display for ClientConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.server_config)
    }
}
