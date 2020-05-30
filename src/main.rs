use std::io::prelude::*;
use std::io::Cursor;
use std::mem::transmute;
use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;
use std::time::Duration;

mod client;
use client::Client;

fn main() {
    let mut connection = Client::new("127.0.0.1");
    let command = "*2\r\n$3\r\nget\r\n$3\r\nfoo\r\n";
    let result = connection.send(command);
    println!("Result {:?}", result);
}
