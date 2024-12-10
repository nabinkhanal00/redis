use bytes::Bytes;

use crate::{connection::Connection, frame::Frame, DB};

pub struct SetExecutor {
    pub key: Bytes,
    pub value: Frame,
}

impl SetExecutor {
    pub async fn execute(&self, con: &mut Connection, db: DB) {
        {
            let mut db = db.lock().unwrap();
            let key = Frame::string_from_bulk(self.key.clone());
            db.insert(key, self.value.encode());
        }
        let _ = con.write_frame(Frame::Simple("OK".to_string())).await;
    }
}
