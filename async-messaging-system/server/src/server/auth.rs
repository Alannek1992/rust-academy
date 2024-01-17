use common::connection::{frame::auth_message::Username, Connection};
use tokio::net::TcpStream;

pub struct AuthConnectionWrapper {
    pub connection: Connection,
    pub authenticated_user: Option<Username>,
}

impl AuthConnectionWrapper {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            connection: Connection::new(stream),
            authenticated_user: None,
        }
    }

    pub fn authenticate(&mut self, username: Username) {
        self.authenticated_user = Some(username);
    }
}
