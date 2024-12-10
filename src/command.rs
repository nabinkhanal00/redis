use crate::result::Result;
use crate::{connection::Connection, frame::Frame};

use crate::commands::{EchoExecutor, GetExecutor, PingExecutor, SetExecutor};

pub enum Command {
    PING(PingExecutor),
    ECHO(EchoExecutor),
    GET(GetExecutor),
    SET(SetExecutor),
}

impl Command {
    pub fn new(frames: Frame) -> Result<Self> {
        if let Frame::Array(frames) = frames {
            let mut frames = frames.into_iter();
            let cmd = frames.next();
            Ok(Command::PING(PingExecutor {}))
        } else {
            Err("invalid frame".into())
        }
    }
    pub async fn execute(&self, con: &mut Connection) {
        match self {
            Self::PING(executor) => {
                executor.execute(con).await;
            }
            _ => {
                unimplemented!()
            }
        }
    }
}
