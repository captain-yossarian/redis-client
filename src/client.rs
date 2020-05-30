use std::io::prelude::*;
use std::io::Cursor;
use std::mem::transmute;
use std::str;
use std::thread;
use std::time::Duration;
use tokio::io::{self, AsyncWriteExt, BufWriter};
use tokio::net::TcpStream;
use tokio::prelude::*;

pub struct Client {
    connection: TcpStream,
}
const CONNECTION_ERROR: &str = "Could not connect to Redis server. Please check again your host";

const CRLF: &str = "\r\n";

const PING: &str = "*1\r\n$4\r\nPING\r\n";

impl Client {
    pub async fn new(host: &str) -> Result<Client, std::io::Error> {
        let address = format!("{}:6379", host);
        let connection = TcpStream::connect(address).await?;

        Ok(Client { connection })
    }

    pub async fn send(&mut self, command: &str) -> Result<String, std::io::Error> {
        let result = command.as_bytes();

        self.connection.write(result).await?;

        let mut buffer: Vec<u8> = Vec::with_capacity(20);
        buffer.extend_from_slice(&[0; 50]);

        match self.connection.read(&mut buffer).await {
            Ok(_) => {
                let mut data = String::from("");
                data.push_str(&String::from_utf8_lossy(&buffer));
                Ok(data)
            }
            Err(e) => Err(e),
        }
    }
}
