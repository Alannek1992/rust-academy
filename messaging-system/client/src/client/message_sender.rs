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

pub struct MessageSender<'t> {
    stream: &'t mut TcpStream,
}

impl<'t> MessageSender<'t> {
    pub fn new(stream: &'t mut TcpStream) -> Self {
        Self { stream }
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

    pub fn login(&mut self) -> Result<Username> {
        let username = Self::read_username()?;
        self.send(&username, Message::Login)?;
        Ok(username)
    }

    pub fn send_message(&mut self, username: &Username) -> Result<MessageEnvelope> {
        let msg = Self::read_message()?;
        let result = self.send(username, msg)?;
        Ok(result)
    }

    fn send(&mut self, username: &Username, message: Message) -> Result<MessageEnvelope> {
        let envelope = MessageEnvelope::new(username, message);
        let envelope_serialized = envelope.serialize()?;
        self.stream.write_all(&envelope_serialized)?;

        Ok(envelope)
    }
}
