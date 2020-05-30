use std::io::Error;
use std::str;
use tokio::io::{self, AsyncWriteExt, BufWriter};
use tokio::net::TcpStream;
use tokio::prelude::*;

pub struct Client {
    connection: TcpStream,
}

impl Client {
    pub async fn new(host: &str) -> Result<Client, Error> {
        let address = format!("{}:6379", host);
        let connection = TcpStream::connect(address).await?;

        Ok(Client { connection })
    }

    pub async fn send(&mut self, command: &str) -> Result<usize, Error> {
        let result = command.as_bytes();
        self.connection.write(result).await
    }

    pub async fn read(&mut self) -> Result<String, Error> {
        let mut buffer: Vec<u8> = Vec::with_capacity(20);
        buffer.extend_from_slice(&[0; 50]);

        match self.connection.read(&mut buffer).await {
            Ok(_) => Ok(format!("{}", String::from_utf8_lossy(&buffer))),
            Err(e) => Err(e),
        }
    }
}
