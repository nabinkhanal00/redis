use crate::frame::Frame;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug)]
pub enum CommandError {
    UnexpectedEndOfFile,
    UnexpectedCharacter(char),
    NegativeSize(i64),
    RedisError(String),
    InvalidCommandFormat,
    CommandNotImplemented(String),
    NonStringArgument(Frame),
    NotEnoughArguments,
    TooManyArguments,
}
