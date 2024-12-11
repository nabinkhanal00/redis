use crate::result::Result;
use crate::DB;
use crate::{connection::Connection, frame::Frame};

use crate::commands::{EchoExecutor, ErrorExecutor, GetExecutor, PingExecutor, SetExecutor};

pub enum Command {
    PING(PingExecutor),
    ECHO(EchoExecutor),
    GET(GetExecutor),
    SET(SetExecutor),
    ERROR(ErrorExecutor),
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
                                let key = frames.next();
                                if let None = key {
                                    return Ok(Command::ERROR(ErrorExecutor {
                                        value: "set needs a key".to_string(),
                                    }));
                                }
                                let key = key.unwrap();
                                if let Frame::Bulk(key) = key {
                                    return Ok(Command::GET(GetExecutor { key }));
                                } else {
                                    return Ok(Command::ERROR(ErrorExecutor {
                                        value: "invalid frame".to_string(),
                                    }));
                                }
                            }
                            "SET" => {
                                let key = frames.next();
                                if let None = key {
                                    return Ok(Command::ERROR(ErrorExecutor {
                                        value: "set needs a key".to_string(),
                                    }));
                                }
                                let key = key.unwrap();
                                if let Frame::Bulk(key) = key {
                                    let value = frames.next();
                                    if let None = value {
                                        return Ok(Command::ERROR(ErrorExecutor {
                                            value: "set needs a value".to_string(),
                                        }));
                                    }
                                    let value = value.unwrap();
                                    let expiry = frames.next();
                                    if let Some(frame) = expiry {
                                        if let Frame::Bulk(expiry) = frame {
                                            let expiry =
                                                Frame::string_from_bulk(expiry).to_lowercase();
                                            if expiry == "px" {
                                                let expiry_time = frames.next();
                                                if let Some(expiry_time) = expiry_time {
                                                    if let Frame::Bulk(expiry_time) = expiry_time {
                                                        let expiry_time =
                                                            Frame::decimal_from_bulk(expiry_time);
                                                        if let Some(expiry_time) = expiry_time {
                                                            return Ok(Command::SET(SetExecutor {
                                                                key,
                                                                value,
                                                                px: Some(expiry_time),
                                                            }));
                                                        } else {
                                                            return Ok(Command::ERROR(ErrorExecutor {
                                                                value: "value is not an integer or out of range".to_string(),
                                                            }));
                                                        }
                                                    } else {
                                                        return Ok(Command::ERROR(ErrorExecutor {
                                                            value: "invalid frame".to_string(),
                                                        }));
                                                    }
                                                } else {
                                                    return Ok(Command::ERROR(ErrorExecutor {
                                                        value: "syntax error".to_string(),
                                                    }));
                                                }
                                            } else {
                                                return Ok(Command::ERROR(ErrorExecutor {
                                                    value: "syntax error".to_string(),
                                                }));
                                            }
                                        } else {
                                            return Ok(Command::ERROR(ErrorExecutor {
                                                value: "invalid frame".to_string(),
                                            }));
                                        }
                                    }

                                    return Ok(Command::SET(SetExecutor {
                                        key,
                                        value,
                                        px: None,
                                    }));
                                } else {
                                    return Ok(Command::ERROR(ErrorExecutor {
                                        value: "invalid frame".to_string(),
                                    }));
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
            Self::ERROR(executor) => executor.execute(con).await,
        }
    }
}
