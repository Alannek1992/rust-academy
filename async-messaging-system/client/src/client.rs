use crate::config::ClientConfig;
use anyhow::{anyhow, Result};
use common::connection::{
    frame::{auth::Auth, Payload},
    Connection,
};
use log::trace;
use tokio::net::TcpStream;

use self::{cmd::Cmd, message_util::MessageUtil};

mod cmd;
mod message_util;
pub struct Client {
    connection: Connection,
}

impl Client {
    pub async fn new(config: ClientConfig) -> Result<Self> {
        let socket_address = config.get_server_socket_address()?;
        let tcp_stream = TcpStream::connect(socket_address).await?;
        let connection = Connection::new(tcp_stream);

        Ok(Self { connection })
    }

    pub async fn run(&mut self) -> Result<()> {
        trace!("Starting client");
        self.handle_authentification().await?;
        Ok(())
    }

    async fn handle_authentification(&mut self) -> Result<()> {
        // get authentification details from user via command line
        let auth_details = Cmd::read_auth_details().await?;
        // convert the provided input into frame
        let sign_frame = if auth_details.existing_account {
            MessageUtil::create_auth_login_frame(&auth_details.username, &auth_details.password)
        } else {
            MessageUtil::create_auth_register_frame(&auth_details.username, &auth_details.password)
        };
        // sending auth frame to the server
        self.connection.write_frame(&sign_frame).await?;

        let auth_frame = self
            .connection
            .read_frame()
            .await?
            .ok_or(anyhow!("Unexpected end of TCP stream"))?;

        // checking if the right frame was received from the server
        match auth_frame.payload {
            Payload::Auth(Auth::LoggedIn(is_logged_in)) => {
                // if authentication was succesfull return Ok, otherwise Err
                if is_logged_in {
                    Ok(())
                } else {
                    Err(anyhow!("Authentication failed"))
                }
            }
            _ =>
            // Fallback error when unexpected frame was received from the server
            {
                Err(anyhow!(
                    "Encountered unexpected frame during authentication"
                ))
            }
        }
    }
}
