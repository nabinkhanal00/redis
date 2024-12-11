use std::time::{Duration, Instant};

use bytes::Bytes;

use crate::{connection::Connection, frame::Frame, DBElement, DB};

pub struct SetExecutor {
    pub key: Bytes,
    pub value: Frame,
    pub px: Option<u64>,
}

impl SetExecutor {
    pub async fn execute(&self, con: &mut Connection, db: DB) {
        {
            let mut db = db.lock().unwrap();
            let key = Frame::string_from_bulk(self.key.clone());
            if let Some(px) = self.px {
                db.insert(
                    key,
                    DBElement {
                        value: self.value.encode(),
                        expiry: Some(Instant::now() + Duration::from_millis(px)),
                    },
                );
            } else {
                db.insert(
                    key,
                    DBElement {
                        value: self.value.encode(),
                        expiry: None,
                    },
                );
            }
        }
        let _ = con.write_frame(Frame::Simple("OK".to_string())).await;
    }
}
