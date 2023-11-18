use common::api::Message;
use common::error::{Error, Result};
use common::util;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{
    net::{SocketAddr, TcpStream},
    thread,
};

use self::message_receiver::MessageReceiver;
use self::message_sender::MessageSender;

mod message_receiver;
mod message_sender;

pub struct Client {
    tcp_stream: TcpStream,
}

impl Client {
    pub fn new(socket_address: SocketAddr) -> Result<Self> {
        let tcp_stream = TcpStream::connect(socket_address)?;
        Ok(Self { tcp_stream })
    }

    pub fn run(&mut self) -> Result<()> {
        let mut stream_clone = self.tcp_stream.try_clone()?;
        let is_healthy = Arc::new(AtomicBool::new(true));
        let is_healthy_clone = is_healthy.clone();

        // reads from stdin and send msg to the server
        let msg_sending_thread = thread::spawn(move || {
            let mut message_sender = MessageSender::new(&mut stream_clone);
            let username = match message_sender.login() {
                Ok(u) => u,
                Err(e) => {
                    is_healthy_clone.store(false, Ordering::Relaxed);
                    util::print_error_to_stdout(e);
                    return;
                }
            };

            while is_healthy_clone.load(Ordering::Relaxed) {
                match message_sender.send_message(&username) {
                    Ok(me) if me.content == Message::Exit => {
                        is_healthy_clone.store(false, Ordering::Relaxed);
                        break;
                    }
                    Err(e) => {
                        is_healthy_clone.store(false, Ordering::Relaxed);
                        util::print_error_to_stdout(e);
                        break;
                    }
                    _ => continue,
                }
            }
        });

        let mut msg_receiver = MessageReceiver::new(&mut self.tcp_stream);
        while is_healthy.load(Ordering::Relaxed) {
            if let Err(_) = msg_receiver.read_and_process_msg() {
                is_healthy.store(false, Ordering::Relaxed);
                util::print_msg_to_stdout("TCP Stream closed!", util::ColorFacade::Yellow);
                break;
            };
        }

        if let Err(_) = msg_sending_thread.join() {
            util::print_error_to_stdout(Error::new("The message sending thread failed"));
        }
        Ok(())
    }
}
