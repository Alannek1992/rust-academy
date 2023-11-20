use common::{
    api::{Message, MessageEnvelope, Username},
    error::Result,
    util,
};
use std::{
    io::{self, Write},
    net::TcpStream,
    str::FromStr,
};

pub struct MessageSender {
    username: String,
}

impl MessageSender {
    pub fn new(tcp_stream: &mut TcpStream) -> Result<Self> {
        let username = Self::login(tcp_stream)?;
        Ok(Self { username })
    }

    fn read_username() -> Result<Username> {
        util::print_msg_to_stdout("Please enter your username", util::ColorFacade::Yellow);
        Self::read_user_input()
    }

    fn read_message() -> Result<Message> {
        let input = Self::read_user_input()?;
        Message::from_str(&input)
    }

    fn read_user_input() -> Result<String> {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_string())
    }

    fn login(tcp_stream: &mut TcpStream) -> Result<Username> {
        let username = Self::read_username()?;
        Self::send(tcp_stream, &username, Message::Login)?;
        Ok(username)
    }

    fn send(
        tcp_stream: &mut TcpStream,
        username: &Username,
        message: Message,
    ) -> Result<MessageEnvelope> {
        let envelope = MessageEnvelope::new(username, message);
        let envelope_serialized = envelope.serialize()?;
        tcp_stream.write_all(&envelope_serialized)?;

        Ok(envelope)
    }

    pub fn send_message(&self, tcp_stream: &mut TcpStream) -> Result<MessageEnvelope> {
        let msg = Self::read_message()?;
        let result = Self::send(tcp_stream, &self.username, msg)?;
        Ok(result)
    }
}
