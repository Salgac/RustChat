

use std::io::Result;
use tokio::net::TcpListener;

use crate::connection::handle_connection;

pub async fn start_server() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server started at 127.0.0.1:8080. Waiting for connections...");

    let (socket, addr) = listener.accept().await?;
    println!("Client connected: {}", addr);

    handle_connection(socket).await
}