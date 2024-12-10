use crate::connection::Connection;
use crate::frame::Frame;

pub struct PingExecutor {}

impl PingExecutor {
    pub async fn execute(&self, con: &mut Connection) {
        let _ = con.write_frame(Frame::Simple("PONG".to_string())).await;
    }
}
