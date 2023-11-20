use common::{
    api::{Message, MessageEnvelope},
    error::Result,
    util,
};
use std::net::TcpStream;

pub struct MessageReceiver {
    storage_path: String,
}

impl MessageReceiver {
    pub fn new(storage_path: &str) -> Self {
        Self {
            storage_path: storage_path.to_string(),
        }
    }

    pub fn read_and_process_msg(&self, tcp_stream: &mut TcpStream) -> Result<()> {
        let msg = MessageEnvelope::read_frame(tcp_stream)?;
        let msg = MessageEnvelope::deserialize(&msg)?;
        self.process_message(&msg)?;
        self.interpret_message_to_stdout(&msg);
        Ok(())
    }

    fn process_message(&self, msg_envelope: &MessageEnvelope) -> Result<()> {
        match &msg_envelope.content {
            Message::File(file_content) | Message::Image(file_content) => {
                let file_path = format!(
                    "{}/from_{}",
                    self.storage_path,
                    msg_envelope.from_user.to_lowercase()
                );
                util::write_to_file(&file_path, &file_content)?;
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn interpret_message_to_stdout(&self, msg_envelope: &MessageEnvelope) {
        // TODO - refine
        let result = match &msg_envelope.content {
            Message::File(file_data) => format!(
                "{} has sent a file with name: {}. Stored in: {}/from_{}",
                msg_envelope.from_user,
                file_data.file_name,
                self.storage_path,
                msg_envelope.from_user.to_lowercase()
            ),
            Message::Image(image_data) => format!(
                "{} has sent an image with name: {}. Stored in: {}/from_{}",
                msg_envelope.from_user,
                image_data.file_name,
                self.storage_path,
                msg_envelope.from_user.to_lowercase()
            ),
            Message::OtherText(t) => format!("{}: {}", msg_envelope.from_user, t),
            Message::Login => format!("{} has joined the channel", msg_envelope.from_user),
            Message::Exit => format!("{} has left the channel", msg_envelope.from_user),
        };

        util::print_msg_to_stdout(&result, util::ColorFacade::Green);
    }
}
