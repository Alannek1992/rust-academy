use std::{io::Write, net::TcpStream};

use common::api::{Message, MessageEnvelope, Username};

pub struct StdioProcessor {
    stream: TcpStream,
}

impl StdioProcessor {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }

    pub fn run(&mut self) {
        let username = Username::from("Alannek");
        let msg = Message::OtherText("Ahoj, jak se mas".to_string());
        let envelope = MessageEnvelope::new(username, msg);

        self.stream
            .write_all(&envelope.serialize().unwrap())
            .unwrap();
    }
}
