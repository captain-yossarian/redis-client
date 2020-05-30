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
            println!("Result {:?}", read);
        }
        Err(e) => println!("Error"),
    }
}
