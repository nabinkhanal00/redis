use std::time::Instant;

use bytes::Bytes;

use crate::{connection::Connection, frame::Frame, DB};

pub struct GetExecutor {
    pub key: Bytes,
}

impl GetExecutor {
    pub async fn execute(&self, con: &mut Connection, db: DB) {
        let value: Option<Bytes>;
        {
            let mut db = db.lock().unwrap();
            let key = Frame::string_from_bulk(self.key.clone());
            if let Some(v) = db.get(&key) {
                if v.expiry.is_none() {
                    value = Some(v.value.clone())
                } else {
                    let exp = v.expiry.unwrap();
                    if exp > Instant::now() {
                        value = Some(v.value.clone())
                    } else {
                        db.remove(&key);
                        value = None
                    }
                }
            } else {
                value = None;
            }
        }
        if let Some(value) = value {
            let _ = con.write_frame(Frame::Bulk(value.clone())).await;
        } else {
            let _ = con.write_frame(Frame::Null).await;
        }
    }
}
