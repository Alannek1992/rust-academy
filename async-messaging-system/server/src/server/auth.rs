use common::connection::Connection;

pub struct AuthConnection {
    connection: Connection,
    is_authenticated: bool,
}

impl AuthConnection {
    pub fn new(connection: Connection) -> Self {
        Self {
            connection,
            is_authenticated: false,
        }
    }

    pub fn authenticate(&mut self) {
        self.is_authenticated = true;
    }

    pub fn is_authenticated(&self) -> bool {
        self.is_authenticated
    }
}
