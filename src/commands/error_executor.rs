use crate::{connection::Connection, frame::Frame};

pub struct ErrorExecutor {
    pub value: String,
}

impl ErrorExecutor {
    pub async fn execute(&self, con: &mut Connection) {
        let _ = con.write_frame(Frame::Error(self.value.clone())).await;
    }
}
