use std::io::prelude::*;
use std::io::Cursor;
use std::mem::transmute;
use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;
use std::time::Duration;

pub struct Client {
    connection: TcpStream,
}
const CONNECTION_ERROR: &str = "Could not connect to Redis server. Please check again your host";

const CRLF: &str = "\r\n";

const PING: &str = "*1\r\n$4\r\nPING\r\n";

impl Client {
    pub fn new(host: &str) -> Client {
        let address = format!("{}:6379", host);
        let stream = TcpStream::connect(address).expect(CONNECTION_ERROR);
        let connection = stream.try_clone().unwrap();

        connection
            .set_read_timeout(Some(Duration::new(1, 0)))
            .expect("Failed to set read timeout");

        connection
            .set_write_timeout(Some(Duration::new(1, 0)))
            .expect("Failed to set write timeout");

        println!("Test {}", PING);

        Client { connection }
    }

    pub fn send(&mut self, command: &str) -> Result<String, std::io::Error> {
        let result = command.as_bytes();

        match self.connection.write(result) {
            Ok(res) => println!("SEND {:?}", res),
            Err(err) => println!("Error: send. {}", err),
        };

        let mut buffer: Vec<u8> = Vec::with_capacity(20);
        buffer.extend_from_slice(&[0; 50]);

        match self.connection.read(&mut buffer) {
            Ok(_) => {
                let mut s = String::from("");
                s.push_str(&String::from_utf8_lossy(&buffer));
                Ok(s)
            }
            Err(e) => Err(e),
        }
    }
}
