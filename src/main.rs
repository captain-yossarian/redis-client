use tokio::prelude::*;
mod client;
use client::Client;
mod parser;
use parser::RESP;

#[tokio::main]
async fn main() {
    let connection = Client::new("127.0.0.1").await;
    match connection {
        Ok(mut conn) => {
            let command = RESP::array(vec!["get", "foo"]);
            let write = conn.send(&command).await;
            let read = conn.read().await;
            println!("{}", 0x51);
            match read {
                Ok(val) => println!("Read Result {:?}", RESP::parse(val)),
                Err(e) => println!("Read Error"),
            }
        }
        Err(e) => println!("Connection error"),
    }
}
