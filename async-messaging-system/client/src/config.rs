use std::net::SocketAddr;

use anyhow::{anyhow, Result};
use log::{trace, warn};

const DEFAULT_STORAGE_PATH: &str = "./storage";

pub struct ClientConfig {
    server_hostname: String,
    server_port: String,
    storage_path: String,
}

impl ClientConfig {
    pub fn new(server_hostname: &str, server_port: &str, storage_path: &str) -> Self {
        trace!(
            "Creating client config with connection to {}:{}",
            server_hostname,
            server_port
        );
        trace!("The chat content will be stored into: {}", storage_path);

        Self {
            server_hostname: server_hostname.to_string(),
            server_port: server_port.to_string(),
            storage_path: storage_path.to_string(),
        }
    }

    pub fn from_args(args: &[String]) -> Result<Self> {
        if args.len() < 3 {
            warn!(
                "Not provided client configuration within arguments: {:?}",
                args
            );
            return Err(anyhow!("The client configuration not provided!"));
        }

        let hostname = &args[1];
        let port = &args[2];
        let storage_path = if args.len() == 4 {
            &args[1]
        } else {
            DEFAULT_STORAGE_PATH
        };

        Ok(Self::new(hostname, port, storage_path))
    }

    pub fn get_server_socket_address(&self) -> Result<SocketAddr> {
        Ok(format!("{}:{}", self.server_hostname, self.server_port).parse()?)
    }
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self::new("127.0.0.1", "11111", DEFAULT_STORAGE_PATH)
    }
}
