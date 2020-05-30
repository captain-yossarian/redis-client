use std::io::prelude::*;
use std::io::Cursor;
use std::mem::transmute;
use std::str;
use std::thread;
use std::time::Duration;
// use tokio::net::TcpListener;
use tokio::prelude::*;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::time::{delay_for, Delay};
mod client;
use client::Client;

fn sleep(ms: u64) -> Delay {
    delay_for(Duration::from_millis(ms))
}
enum Message {
    Foo,
    Bar,
}

async fn send_message(mut channel: Sender<Message>) {
    match channel.send(Message::Bar).await {
        Ok(_) => println!("Hello"),
        Err(e) => println!("Hello"),
    }
}

#[tokio::main]
async fn main() {
    let mut connection = Client::new("127.0.0.1").await;
    match connection {
        Ok(mut conn) => {
            let command = "*2\r\n$3\r\nget\r\n$3\r\nfoo\r\n";
            let result = conn.send(command).await;
            sleep(1000).await;
            println!("Result {:?}", result);
        }
        Err(e) => println!("Error"),
    }
}
