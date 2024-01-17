
use anyhow::{Result, anyhow};
use bytes::{Buf, BytesMut};
use log::trace;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
    net::TcpStream,
};

use self::frame::{Frame, FrameDeserializationError};

pub mod frame;

pub struct Connection {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream: BufWriter::new(stream),
            buffer: BytesMut::with_capacity(4 * 1024),
        }
    }

    pub async fn read_frame(&mut self) -> Result<Option<Frame>> {
        loop {
            // Attempt to parse a frame from the buffered data. If enough data
            // has been buffered, the frame is returned.
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }

            // There is not enough buffered data to read a frame. Attempt to
            // read more data from the socket.
            //
            // On success, the number of bytes is returned. `0` indicates "end
            // of stream".
            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                // The remote closed the connection. For this to be a clean
                // shutdown, there should be no data in the read buffer. If
                // there is, this means that the peer closed the socket while
                // sending a frame.
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err(anyhow!("Peer abruptly interrupted the connection!").into());
                }
            }
        }
    }

    pub async fn write_frame(&mut self, frame: &Frame) -> Result<()> {
        trace!("Writing a frame into TCP stream");
        let serialized_frame = frame.serialize()?;
        self.stream.write_all(&serialized_frame).await?;
        self.stream.flush().await?;
        Ok(())
    }

    fn parse_frame(&mut self) -> std::result::Result<Option<Frame>, FrameDeserializationError> {
        trace!("Parsing a frame from the TCP stream");
        let mut cloned_buf = self.buffer.clone();

        match Frame::deserialize(&mut cloned_buf) {
            Ok(frame) => {
                // Update the buffer to remove the consumed bytes
                let consumed = self.buffer.len() - cloned_buf.remaining();
                self.buffer.advance(consumed);
                Ok(Some(frame))
            }
            Err(err) => match err {
                FrameDeserializationError::Incomplete => Ok(None),
                _ => Err(err),
            },
        }
    }
}
