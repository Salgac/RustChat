use std::io::Result;
use tokio::net::TcpStream;

use crate::connection::handle_connection;

pub async fn start_client(addr: &str) -> Result<()> {
    let socket = TcpStream::connect(addr).await?;
    println!("Connected to server at {}", addr);

    handle_connection(socket).await
}