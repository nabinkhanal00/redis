use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use bytes::Bytes;

use command::Command;
use connection::Connection;
use tokio::io::Error;
use tokio::net::TcpListener;

mod command;
mod commands;
mod connection;
mod error;
mod frame;
mod result;

struct DBElement {
    value: Bytes,
    expiry: Option<Instant>,
}
type DB = Arc<Mutex<HashMap<String, DBElement>>>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("connected to {:?}", addr);
        let connection = Connection::new(stream);
        tokio::spawn(handle_connection(connection, db.clone()));
    }
}

async fn handle_connection(mut con: Connection, db: DB) {
    loop {
        let frame = con.read_frame().await;
        if let Ok(frame) = frame {
            if let Some(frame) = frame {
                let command = Command::new(frame).unwrap();
                command.execute(&mut con, db.clone()).await;
            } else {
                println!("Connection closed");
                return;
            }
        } else {
            println!("Connection closed abruptly");
            return;
        }
    }
}
