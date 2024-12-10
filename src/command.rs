use crate::frame::Frame;

pub enum Command {
    PING,
    ECHO(Frame),
    GET(Frame),
    SET(Frame, Frame),
}
