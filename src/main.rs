mod client;
mod errors;
mod parser;
use client::Client;
use parser::RESP;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let mut connection = Client::new("127.0.0.1").await?;
    connection.set("test2", "test2-echo").await?;

    Ok(())
}
