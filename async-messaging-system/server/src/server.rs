use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use crate::config::ServerConfig;
use anyhow::Result;
use common::util;
use log::trace;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

use self::auth::AuthConnectionWrapper;

mod auth;

pub struct Server {
    config: ServerConfig,
    listener: TcpListener,
    clients: Arc<Mutex<HashMap<SocketAddr, AuthConnectionWrapper>>>,
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
        clients: &mut HashMap<SocketAddr, AuthConnectionWrapper>,
        client: (SocketAddr, TcpStream),
    ) {
        trace!("New client joined: {}", client.0);
        let connection = AuthConnectionWrapper::new(client.1);
        clients.insert(client.0, connection);
    }

    async fn handle_client(
        clients: Arc<Mutex<HashMap<SocketAddr, AuthConnectionWrapper>>>,
        client: (SocketAddr, TcpStream),
    ) -> Result<()> {
        let mut clients = clients.lock().await;
        let auth_connection_wrapper = clients.get_mut(&client.0);

        match auth_connection_wrapper {
            Some(acw) => {
                match acw.connection.read_frame().await {
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
    pub async fn run(&mut self) -> Result<()> {
        trace!("Starting server");
        loop {
            let (stream, addr) = self.listener.accept().await?;
            let clients = Arc::clone(&self.clients);
            tokio::spawn(async move {
                if let Err(e) = Server::handle_client(clients, (addr, stream)).await {
                    util::print_msg_to_stderr(e);
                }
            });
        }
    }
}
