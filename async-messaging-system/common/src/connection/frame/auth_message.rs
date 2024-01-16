use argon2rs::argon2d_simple;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum AuthMessage {
    Register(UserSignData),
    Login(UserSignData),
    Logout,
}

pub type Username = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSignData {
    username: Username,
    password: [u8; 32],
}

impl UserSignData {
    pub fn new(username: &str, password: &str) -> UserSignData {
        // TODO provide meaningful way to setup the salt
        let salt = "RUST_ACADEMY";
        let password = argon2d_simple(password, salt);

        Self {
            username: username.to_string(),
            password,
        }
    }
}
