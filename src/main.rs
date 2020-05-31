mod client;
mod errors;
mod parser;
use client::Client;
use parser::RESP;
use tokio::prelude::*;

#[tokio::main]
async fn main() {
    let connection = Client::new("127.0.0.1").await;
    match connection {
        Ok(mut conn) => {
            let command = RESP::array(vec!["get", "foo"]);
            let write = conn.send(&command).await;
            let read = conn.read().await;
            match read {
                Ok(val) => println!("Read Result {:?}", RESP::parse(val)),
                Err(e) => println!("Read Error"),
            }
        }
        Err(e) => println!("Connection error"),
    }
}
