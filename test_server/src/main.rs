use ipc::from_server::FromServer;
use ipc::to_server::ToServer;
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to a server
    let mut stream = TcpStream::connect("127.0.0.1:6379").await?;

    let encode = bincode::serialize(&ToServer::CreateGame).unwrap();
    stream.write_all(&encode).await?;
    let mut buf = vec![0; 1024];
    let n = stream.read(&mut buf).await?;
    println!("{n} bytes read");
    let decoded = bincode::deserialize::<FromServer>(&buf[..n])?;
    println!("{decoded:?}");
    Ok(())
}
