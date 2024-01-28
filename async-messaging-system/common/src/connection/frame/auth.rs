use anyhow::{anyhow, Result};
use argon2rs::argon2d_simple;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub enum Auth {
    Register(UserSignData),
    Login(UserSignData),
    Logout,
    LoggedIn(bool),
}

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

pub type Username = String;

#[derive(Serialize, Deserialize)]
pub struct UserClaims {
    pub username: Username,
    exp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthToken(String);

impl AuthToken {
    pub fn generate_token(
        username: &Username,
        expiration_time: u64,
        secret_key: &[u8],
    ) -> Result<Self> {
        let now = SystemTime::now();
        let exp = now
            .checked_add(Duration::from_secs(expiration_time))
            .ok_or(anyhow!("Overflow in auth token expiration calculation"))?;

        let payload = UserClaims {
            username: username.clone(),
            exp: exp.duration_since(UNIX_EPOCH)?.as_secs(),
        };

        let token = encode(
            &Header::default(),
            &payload,
            &EncodingKey::from_secret(secret_key),
        )?;

        Ok(Self(token))
    }

    pub fn validate_token(token: &Self, secret_key: &[u8]) -> Result<UserClaims> {
        let decoding_key = DecodingKey::from_secret(secret_key);

        let validation = Validation::default();

        let decoded_token = decode::<UserClaims>(&token.0, &decoding_key, &validation)?;

        Ok(decoded_token.claims)
    }
}
