use common::connection::{
    frame::{auth_message::Username, Frame},
    Connection,
};
use common::error::Result;
use tokio::net::TcpStream;

pub struct AuthConnection {
    connection: Connection,
    authenticated_user: Option<Username>,
}

impl AuthConnection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            connection: Connection::new(stream),
            authenticated_user: None,
        }
    }

    pub fn authenticate(&mut self, username: Username) {
        self.authenticated_user = Some(username);
    }

    pub fn is_authenticated(&self) -> bool {
        self.authenticated_user.is_some()
    }

    pub async fn write_frame(&mut self, frame: &Frame) -> Result<()> {
        self.connection.write_frame(frame).await
    }

    pub async fn read_frame(&mut self) -> Result<Option<Frame>> {
        self.connection.read_frame().await
    }
}
