use std::io::Result;

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
    sync::mpsc,
};

pub async fn handle_connection(socket: TcpStream) -> Result<()> {
    let (reader, mut writer) = socket.into_split();
    let mut reader = BufReader::new(reader);

    // Channel for sending messages from input to writer
    let (tx, mut rx) = mpsc::channel::<String>(100);

    // Task to read input from the console and send it to the channel
    tokio::spawn(async move {
        let stdin = tokio::io::stdin();
        let mut lines = BufReader::new(stdin).lines();
        while let Ok(Some(line)) = lines.next_line().await {
            if tx.send(line).await.is_err() {
                break;
            }
        }
    });

    // Task to read messages from the channel and write them to the socket
    let writer_task = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            if writer.write_all(message.as_bytes()).await.is_err() {
                break;
            }
            writer.write_all(b"\n").await.unwrap();
        }
    });

    // Task to read messages from the socket and print them to the console
    let reader_task = tokio::spawn(async move {
        let mut buffer = String::new();
        while reader.read_line(&mut buffer).await.unwrap_or(0) > 0 {
            print!("> {}", buffer);
            buffer.clear();
        }
    });

    tokio::try_join!(writer_task, reader_task).ok();
    Ok(())
}
