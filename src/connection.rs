use std::io::Cursor;

use bytes::{Buf, BytesMut};
use tokio::io::AsyncWriteExt;
use tokio::{io::AsyncReadExt, net::TcpStream};

use crate::frame::Frame;
use crate::result::Result;

const BUFFER_CAPACITY: usize = 4096;

pub struct Connection {
    stream: TcpStream,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            stream,
            buffer: BytesMut::with_capacity(BUFFER_CAPACITY),
        }
    }

    pub async fn write_frame(&mut self, frame: Frame) -> Result<()> {
        match frame {
            Frame::Simple(value) => {
                let value = format!("+{}\r\n", value);
                self.stream.write(value.as_bytes()).await?;
            }
            Frame::Bulk(bytes) => {
                self.stream.write(&bytes[..]).await?;
            }

            _ => {
                unimplemented!()
            }
        }
        Ok(())
    }

    pub async fn read_frame(&mut self) -> Result<Option<Frame>> {
        loop {
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }
            let len = self.stream.read_buf(&mut self.buffer).await?;
            if 0 == len {
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err("connection reset by peer".into());
                }
            }
        }
    }

    pub fn parse_frame(&mut self) -> Result<Option<Frame>> {
        let mut cursor = Cursor::new(&self.buffer[..]);
        match Frame::check(&mut cursor) {
            Ok(_) => {
                let len = cursor.position() as usize;
                cursor.set_position(0);
                let frame = Frame::parse(&mut cursor)?;
                self.buffer.advance(len);
                Ok(Some(frame))
            }
            Err(crate::frame::Error::Incomplete) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}
