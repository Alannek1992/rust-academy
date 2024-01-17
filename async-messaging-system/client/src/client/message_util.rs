use anyhow::Result;
use common::connection::frame::{
    auth_message::{AuthMessage, UserSignData},
    message::Message,
    Frame,
};

pub struct MessageUtil;

impl MessageUtil {
    pub async fn create_message_frame(user_input: &str) -> Result<Frame> {
        let msg = Message::from_str(user_input).await?;
        Ok(Frame::Msg(msg))
    }

    pub fn create_auth_login_frame(username: &str, password: &str) -> Frame {
        let user_sign_data = UserSignData::new(username, password);
        let auth_message = AuthMessage::Login(user_sign_data);
        Frame::Auth(auth_message)
    }

    pub fn create_auth_register_frame(username: &str, password: &str) -> Frame {
        let user_sign_data = UserSignData::new(username, password);
        let auth_message = AuthMessage::Register(user_sign_data);
        Frame::Auth(auth_message)
    }

    pub fn create_auth_logout_frame() -> Frame {
        let auth_message = AuthMessage::Logout;
        Frame::Auth(auth_message)
    }
}
