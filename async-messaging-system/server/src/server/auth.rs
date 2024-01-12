use tokio::net::TcpStream;

pub struct AuthTcpStream {
    stream: TcpStream,
    is_authenticated: bool,
}

impl AuthTcpStream {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
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
