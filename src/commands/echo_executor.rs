use bytes::Bytes;

use crate::{connection::Connection, frame::Frame};

pub struct EchoExecutor {
    pub value: Bytes,
}

impl EchoExecutor {
    pub async fn execute(&self, con: &mut Connection) {
        let value = self.value.as_ref();
        let value = Bytes::copy_from_slice(value);
        let _ = con.write_frame(Frame::Bulk(value)).await;
    }
}
