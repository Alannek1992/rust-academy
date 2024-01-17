use crate::config::ClientConfig;
use anyhow::Result;
use common::connection::{frame::auth_message::Username, Connection};
use log::trace;
use tokio::net::TcpStream;

use self::cmd_util::CmdUtil;

mod cmd_util;
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
        self.read_and_send_message().await.unwrap();
        Ok(())
    }

    async fn read_and_send_message(&mut self) -> Result<()> {
        let auth_details = CmdUtil::read_auth_details().await?;

        Ok(())
    }
}
