use bytes::Bytes;

use crate::{connection::Connection, frame::Frame, CONFIG};
pub struct ConfigExecutor {
    pub command: Bytes,
    pub key: Bytes,
}

impl ConfigExecutor {
    pub async fn execute(&self, con: &mut Connection, cfg: CONFIG) {
        let output: Frame;
        {
            let cfg = cfg.lock().unwrap();
            let command = Frame::string_from_bulk(self.command.clone());
            let key = Frame::string_from_bulk(self.key.clone());
            let value = cfg.get(&key).unwrap().clone();
            if command.to_lowercase() == "get" {
                let name = Bytes::from(format!("${}\r\n{}\r\n", key.len(), key));
                let value = Bytes::from(format!("${}\r\n{}\r\n", value.len(), value));
                output = Frame::Array(vec![Frame::Bulk(name), Frame::Bulk(value)]);
            } else {
                output = Frame::Error("invalid command".to_string());
            }
        }
        let _ = con.write_frame(output).await;
    }
}
