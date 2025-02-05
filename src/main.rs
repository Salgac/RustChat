mod client;
mod server;
mod connection;
mod utils;

use std::io::Result;

use client::start_client;
use server::start_server;

use utils::readln;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Enter 's' to start as server or 'c' to connect as client:");
    let role = readln();

    match role.as_str() {
        "s" => {
            println!("Starting server!");
            start_server().await?;
        }
        "c" => {
            println!("Enter server address (e.g., 127.0.0.1:8080):");
            let addr = readln();
            start_client(addr.as_str()).await?;
        }
        _ => {
            println!("Invalid input. Exiting.");
        }
    }
    Ok(())
}
