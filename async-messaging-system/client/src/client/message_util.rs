use anyhow::Result;
use common::connection::frame::{
    auth::{Auth, AuthToken, UserSignData},
    message::Message,
    Frame, Header, Payload,
};

pub struct MessageUtil;

impl MessageUtil {
    pub async fn create_message_frame(user_input: &str, auth_token: AuthToken) -> Result<Frame> {
        let msg = Message::from_str(user_input).await?;
        let payload = Payload::Msg(msg);
        Ok(Frame::new(Header::with_token(auth_token), payload))
    }

    pub fn create_auth_login_frame(username: &str, password: &str) -> Frame {
        let user_sign_data = UserSignData::new(username, password);
        let auth_message = Auth::Login(user_sign_data);
        let payload = Payload::Auth(auth_message);
        Frame::new(Header::new(), payload)
    }

    pub fn create_auth_register_frame(username: &str, password: &str) -> Frame {
        let user_sign_data = UserSignData::new(username, password);
        let auth_message = Auth::Register(user_sign_data);
        let payload = Payload::Auth(auth_message);
        Frame::new(Header::new(), payload)
    }

    pub fn create_auth_logout_frame(auth_token: AuthToken) -> Frame {
        let auth_message = Auth::Logout;
        let payload = Payload::Auth(auth_message);
        Frame::new(Header::with_token(auth_token), payload)
    }
}
