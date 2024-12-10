use crate::result::Result;
use crate::DB;
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
            if let Some(frame) = frames.next() {
                match frame {
                    Frame::Bulk(cmd) => {
                        let cmd = Frame::string_from_bulk(cmd).to_uppercase();
                        match cmd.as_str() {
                            "PING" => Ok(Command::PING(PingExecutor {})),
                            "GET" => {
                                if let Some(key) = frames.next() {
                                    if let Frame::Bulk(key) = key {
                                        return Ok(Command::GET(GetExecutor { key }));
                                    } else {
                                        return Err("invalid frame".into());
                                    }
                                } else {
                                    return Err("get needs a key".into());
                                }
                            }
                            "SET" => {
                                if let Some(key) = frames.next() {
                                    if let Frame::Bulk(key) = key {
                                        if let Some(value) = frames.next() {
                                            return Ok(Command::SET(SetExecutor { key, value }));
                                        } else {
                                            return Err("set needs a value".into());
                                        }
                                    } else {
                                        return Err("invalid frame".into());
                                    }
                                } else {
                                    return Err("set needs a key".into());
                                }
                            }
                            "ECHO" => {
                                if let Some(value) = frames.next() {
                                    if let Frame::Bulk(value) = value {
                                        return Ok(Command::ECHO(EchoExecutor { value }));
                                    } else {
                                        return Err("invalid frame".into());
                                    }
                                } else {
                                    return Err("get needs a value".into());
                                }
                            }
                            _ => {
                                return Err("invalid command".into());
                            }
                        }
                    }
                    _ => {
                        return Err("invalid frame".into());
                    }
                }
            } else {
                return Err("invalid frame".into());
            }
        } else {
            Err("invalid frame".into())
        }
    }
    pub async fn execute(&self, con: &mut Connection, db: DB) {
        match self {
            Self::PING(executor) => {
                executor.execute(con).await;
            }
            Self::ECHO(executor) => {
                executor.execute(con).await;
            }
            Self::GET(executor) => {
                executor.execute(con, db).await;
            }
            Self::SET(executor) => {
                executor.execute(con, db).await;
            }
        }
    }
}
