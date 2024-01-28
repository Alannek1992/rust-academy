use anyhow::Result;
use common::connection::{
    frame::auth::{AuthToken, Username},
    Connection,
};
use tokio::net::TcpStream;

// TODO - provide a meaningful way how to setup secret
const SECRET_KEY: &[u8] = b"RUST_ACADEMY";

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

    pub fn authenticate(&mut self, auth_token: AuthToken) -> Result<()> {
        let user_claims = AuthToken::validate_token(&auth_token, SECRET_KEY)?;
        if self.authenticated_user.is_none() {
            self.authenticated_user = Some(user_claims.username);
        };
        Ok(())
    }
}
