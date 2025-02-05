use std::io::*;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Enter 's' to start as server or 'c' to connect as client:");
    let role = read();

    match role.as_str() {
        "s" => {
            println!("Starting server!");
            //start_server().await?;
        }
        "c" => {
            println!("Enter server address (e.g., 127.0.0.1:8080):");
            let _addr = read();
            //start_client(addr).await?;
        }
        _ => {
            println!("Invalid input. Exiting.");
        }
    }
    Ok(())
}

fn read() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input);
    String::from(input.trim())
}
