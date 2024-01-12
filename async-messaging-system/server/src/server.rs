use std::{collections::HashMap, net::SocketAddr};

use crate::config::ServerConfig;
use anyhow::Result;
use log::trace;
use tokio::net::TcpListener;

use self::auth::AuthTcpStream;

mod auth;

pub struct Server {
    config: ServerConfig,
    listener: TcpListener,
    clients: HashMap<SocketAddr, AuthTcpStream>
}

impl Server {
    pub async fn new(config: ServerConfig) -> Result<Self> {
        let socket_address = config.to_socket_address()?;
        let listener = TcpListener::bind(socket_address).await?;

        Ok(Self { config, listener, clients: HashMap::new() })
    }

    pub async fn run(&self) -> Result<Self> {
        trace!("Starting server");
        loop {
            let (stream, addr) = self.listener.accept().await?;
            tokio::spawn(||)
        }
    }
}
