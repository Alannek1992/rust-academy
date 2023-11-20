use common::api::Message;
use common::error::{Error, Result};
use common::util::{self, ColorFacade};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{net::TcpStream, thread};

use crate::config::ClientConfig;

use self::message_receiver::MessageReceiver;
use self::message_sender::MessageSender;

mod message_receiver;
mod message_sender;

pub struct Client {
    tcp_stream: TcpStream,
    message_sender: Arc<MessageSender>,
    message_receiver: MessageReceiver,
}

impl Client {
    pub fn new(config: ClientConfig) -> Result<Self> {
        let socket_address = config.to_socket_address()?;
        let mut tcp_stream = TcpStream::connect(socket_address)?;
        let message_sender = MessageSender::new(&mut tcp_stream)?;
        let message_receiver = MessageReceiver::new(&config.storage_path);

        util::print_msg_to_stdout(&format!("Connected to: {}", config), ColorFacade::Yellow);

        Ok(Self {
            tcp_stream,
            message_sender: Arc::new(message_sender),
            message_receiver,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        let mut tcp_stream = self.tcp_stream.try_clone()?;
        let msg_sender = self.message_sender.clone();
        let is_running = Arc::new(AtomicBool::new(true));
        let is_running_c = is_running.clone();

        // reads from stdin and send msg to the server
        let msg_sending_thread = thread::spawn(move || {
            while is_running_c.load(Ordering::Relaxed) {
                match msg_sender.send_message(&mut tcp_stream) {
                    Ok(me) if me.content == Message::Exit => {
                        is_running_c.store(false, Ordering::Relaxed);
                        break;
                    }
                    Err(e) => {
                        util::print_error_to_stdout(e);
                    }
                    _ => continue,
                }
            }
        });

        while is_running.load(Ordering::Relaxed) {
            if let Err(e) = self
                .message_receiver
                .read_and_process_msg(&mut self.tcp_stream)
            {
                if util::is_stream_closed(&mut self.tcp_stream) {
                    util::print_msg_to_stdout("TCP Stream closed!", util::ColorFacade::Yellow);
                    is_running.store(false, Ordering::Relaxed);
                    break;
                }
                util::print_error_to_stdout(e);
            };
        }

        if let Err(_) = msg_sending_thread.join() {
            util::print_error_to_stdout(Error::new("The message sending thread failed"));
        }
        Ok(())
    }
}
