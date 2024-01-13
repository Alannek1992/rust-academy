use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use crate::config::ServerConfig;
use anyhow::Result;
use common::connection::Connection;
use log::trace;
use tokio::net::{TcpListener, TcpStream};

mod auth;

pub struct Server {
    config: ServerConfig,
    listener: TcpListener,
    clients: Arc<Mutex<HashMap<SocketAddr, Connection>>>,
}

// Associated functions
impl Server {
    pub async fn new(config: ServerConfig) -> Result<Self> {
        let socket_address = config.to_socket_address()?;
        let listener = TcpListener::bind(socket_address).await?;
        let clients: Arc<Mutex<HashMap<_, _>>> = Arc::new(Mutex::new(HashMap::new()));

        Ok(Self {
            config,
            listener,
            clients,
        })
    }

    fn accept_client(
        clients: &mut HashMap<SocketAddr, Connection>,
        client: (SocketAddr, TcpStream),
    ) {
        let auth_stream = Connection::new(client.1);
        clients.insert(client.0, auth_stream);
    }
}

// Methods
impl Server {
    pub async fn run(&self) -> Result<Self> {
        trace!("Starting server");
        loop {
            let (stream, addr) = self.listener.accept().await?;
            let clients = Arc::clone(&self.clients);
            tokio::spawn(async move {
                let mut clients = clients.lock().unwrap();
                if !clients.contains_key(&addr) {
                    Server::accept_client(&mut clients, (addr, stream));
                }
            });
        }
    }
}
