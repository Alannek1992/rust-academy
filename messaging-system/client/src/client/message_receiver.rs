use common::{
    api::{Message, MessageEnvelope},
    error::Result,
    util,
};
use std::net::TcpStream;

pub struct MessageReceiver<'t> {
    stream: &'t mut TcpStream,
}

impl<'t> MessageReceiver<'t> {
    pub fn new(stream: &'t mut TcpStream) -> Self {
        Self { stream }
    }

    fn interpret_message_to_stdio(msg_envelope: &MessageEnvelope) {
        let result = match &msg_envelope.content {
            Message::OtherText(t) => format!("{}: {}", msg_envelope.from_user, t),
            Message::Login => format!("{} has joined the channel", msg_envelope.from_user),
            Message::Exit => format!("{} has left the channel", msg_envelope.from_user),
            _ => todo!(),
        };

        util::print_msg_to_stdout(&result, util::ColorFacade::Green);
    }

    pub fn read_and_process_msg(&mut self) -> Result<()> {
        let msg = MessageEnvelope::read_frame(&mut self.stream)?;
        let msg = MessageEnvelope::deserialize(&msg)?;
        Self::interpret_message_to_stdio(&msg);
        Ok(())
    }
}
