mod client;
mod errors;
mod parser;
use client::Client;
use parser::RESP;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let mut connection = Client::new("127.0.0.1").await?;
    let command = RESP::array(vec!["get", "foo"]);
    connection.send(&command).await?;
    let read = connection.read().await?;
    println!("Read Result {:?}", RESP::parse(read));
    Ok(())
}
