use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use crate::config::ServerConfig;
use anyhow::Result;
use log::trace;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

use self::auth::AuthConnection;

mod auth;

pub struct Server {
    config: ServerConfig,
    listener: TcpListener,
    clients: Arc<Mutex<HashMap<SocketAddr, AuthConnection>>>,
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
        clients: &mut HashMap<SocketAddr, AuthConnection>,
        client: (SocketAddr, TcpStream),
    ) {
        let connection = AuthConnection::new(client.1);
        clients.insert(client.0, connection);
    }

    async fn handle_client(
        clients: Arc<Mutex<HashMap<SocketAddr, AuthConnection>>>,
        client: (SocketAddr, TcpStream),
    ) -> Result<()> {
        let mut clients = clients.lock().await;
        let connection = clients.get_mut(&client.0);

        match connection {
            Some(connection) => {
                match connection.read_frame().await {
                    Ok(option) => {
                        // No need to consider the None case since it indicated the stream was closed properly and cleanup was done
                        if let Some(frame) = option {
                            // proceed frame
                            println!("Received frame: {:?}", frame);
                        }
                    }
                    Err(e) => {
                        // Client abruptly exited the connection and the old one is still in the map
                        // Replace it by the the new one
                        Self::accept_client(&mut clients, client);
                    }
                }
            }
            None => {
                Self::accept_client(&mut clients, client);
            }
        };

        Ok(())
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
                Server::handle_client(clients, (addr, stream)).await;
            });
        }
    }
}
