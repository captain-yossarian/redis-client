use crate::parser::RESP;
use std::io::Error;
use std::str;
use tokio::io::AsyncWriteExt;
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

    async fn send(&mut self, command: &str) -> Result<usize, Error> {
        let result = command.as_bytes();
        self.connection.write(result).await
    }

    async fn read(&mut self) -> Result<Vec<u8>, Error> {
        let mut buffer: Vec<u8> = Vec::with_capacity(20);
        buffer.extend_from_slice(&[0; 50]);

        match self.connection.read(&mut buffer).await {
            Ok(_) => Ok(buffer),
            Err(e) => Err(e),
        }
    }

    pub async fn get(&mut self, key: &str) -> Result<usize, Error> {
        let command = RESP::make_array(vec!["get", key]);
        self.send(&command).await;
        self.read().await
    }

    pub async fn set(&mut self, key: &str, value: &str) -> Result<usize, Error> {
        let command = RESP::make_array(vec!["set", key, value]);
        println!("Command {}", command);
        self.send(&command).await
    }
}
