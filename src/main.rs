use connection::Connection;
use frame::Frame;
use std::net::SocketAddr;
use tokio::io::Error;
use tokio::net::TcpListener;

mod command;
mod connection;
mod error;
mod frame;
mod result;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("connected to {:?}", addr);
        let connection = Connection::new(stream);
        tokio::spawn(handle_connection(connection));
    }
}

async fn handle_connection(mut con: Connection) {
    loop {
        let frame = con.read_frame().await;
        if let Ok(frame) = frame {
            if let Some(frame) = frame {
                match frame {
                    Frame::Simple(val) => {
                        if val.to_lowercase() == "ping" {
                            let _ = con.write_frame(Frame::Simple("PONG".to_string())).await;
                        }
                    }
                    _ => unimplemented!(),
                }
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
