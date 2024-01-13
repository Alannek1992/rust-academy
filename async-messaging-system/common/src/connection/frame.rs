mod message;

pub enum Frame {
    Msg,
    Auth,
    IncomingMsgNotification
}