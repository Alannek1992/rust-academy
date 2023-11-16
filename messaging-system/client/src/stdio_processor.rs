use std::io;
use std::str::FromStr;
use std::{io::Write, net::TcpStream};

use common::api::{Message, MessageEnvelope, Username};
use common::error::Result;
use common::util;

pub struct StdioProcessor {
    stream: TcpStream,
}

impl StdioProcessor {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }

    fn read_user_input() -> Result<String> {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input)
    }

    fn read_username() -> Result<Username> {
        util::print_msg_to_stdout("Please enter your username", util::ColorFacade::Yellow);
        Self::read_user_input()
    }

    fn read_message() -> Result<Message> {
        let input = Self::read_user_input()?;
        Message::from_str(&input)
    }

    // returns information how many bytes were actually written
    fn send_message(&mut self, username: &Username, message: Message) -> Result<usize> {
        let envelope = MessageEnvelope::new(username, message);
        let envelope = envelope.serialize()?;
        self.stream.write_all(&envelope)?;
        Ok(envelope.len())
    }

    pub fn run(&mut self) {
        let username = Self::read_username().unwrap();
        let mut msg = Message::Login;

        self.send_message(&username, msg).unwrap();

        loop {
            msg = Self::read_message().unwrap();
            self.send_message(&username, msg).unwrap();
        }
    }
}
