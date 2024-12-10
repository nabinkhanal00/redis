use std::fmt;
use std::io::Cursor;
use std::string::FromUtf8Error;

use bytes::{Buf, Bytes};

#[derive(Debug)]
pub enum Error {
    Incomplete,
    Other(crate::error::Error),
}

#[derive(Debug)]
pub enum Frame {
    Simple(String),
    Error(String),
    Integer(i64),
    Bulk(Bytes),
    Array(Vec<Frame>),
}

impl Frame {
    pub fn check(cursor: &mut Cursor<&[u8]>) -> Result<(), Error> {
        match get_u8(cursor)? as char {
            '+' => {
                let _ = get_line(cursor)?;
                Ok(())
            }
            '*' => {
                let count = get_decimal(cursor)?;
                for _ in 0..count {
                    let _ = Self::check(cursor)?;
                }
                Ok(())
            }
            '$' => {
                let len = get_decimal(cursor)? as usize;
                let str = get_line(cursor)?;
                if str.len() != len {
                    return Err("bulk string size not matching".into());
                }
                Ok(())
            }

            _ => {
                unimplemented!()
            }
        }
    }
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Frame, Error> {
        match get_u8(cursor)? as char {
            '+' => {
                let line = get_line(cursor)?.to_vec();
                let string = String::from_utf8(line)?;
                Ok(Frame::Simple(string))
            }
            '$' => {
                let _ = get_decimal(cursor)? as usize;
                let str = get_line(cursor)?;
                let b = Bytes::copy_from_slice(str);
                Ok(Frame::Bulk(b))
            }
            '*' => {
                let count = get_decimal(cursor)?;
                let mut frames = Vec::new();
                for i in 0..count {
                    let frame = Self::parse(cursor)?;
                    frames.push(frame);
                }
                Ok(Frame::Array(frames))
            }
            _ => {
                unimplemented!()
            }
        }
    }
}

fn get_line<'a>(cursor: &mut Cursor<&'a [u8]>) -> Result<&'a [u8], Error> {
    let start = cursor.position() as usize;
    let end = cursor.get_ref().len() - 1;

    for i in start..end {
        if cursor.get_ref()[i] == b'\r' && cursor.get_ref()[i + 1] == b'\n' {
            cursor.set_position((i + 2) as u64);
            return Ok(&cursor.get_ref()[start..i]);
        }
    }
    Err(Error::Incomplete)
}

fn get_u8(cursor: &mut Cursor<&[u8]>) -> Result<u8, Error> {
    if !cursor.has_remaining() {
        return Err(Error::Incomplete);
    }
    Ok(cursor.get_u8())
}

fn get_decimal(cursor: &mut Cursor<&[u8]>) -> Result<u64, Error> {
    use atoi::atoi;
    if !cursor.has_remaining() {
        return Err(Error::Incomplete);
    }

    let line = get_line(cursor)?;

    atoi::<u64>(line).ok_or_else(|| "protocol error: invalid frame format".into())
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Incomplete => "stream ended early".fmt(fmt),
            Error::Other(err) => err.fmt(fmt),
        }
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error::Other(value.into())
    }
}

impl From<FromUtf8Error> for Error {
    fn from(_value: FromUtf8Error) -> Self {
        "protocol error; invalid frame format".into()
    }
}
