use std::error::Error;

use futures_util::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;

pub async fn run_main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:9000").await?;

    println!("[SERVER] listening on port 9000");

    loop {
        let (stream, addr) = listener.accept().await?;

        println!("[SERVER] {}, peer:{}", addr, stream.peer_addr()?);

        tokio::spawn(async move {
            if let Err(e) = handle_conn(stream, addr.to_string()).await {
                eprintln!("[SERVER] Error: {}", e);
            }
        });
    }
}

async fn handle_conn(stream: TcpStream, addr: String) -> Result<(), Box<dyn Error>> {
    println!("[CLIENT {}] Performing WebSocket handshake...", addr);
    let ws_stream = accept_async(stream).await?;
    println!("[CLIENT {}] WebSocket connection established!\n", addr);

    let (mut write, mut read) = ws_stream.split();

    write
        .send(Message::Text("Welcome to Websocket server!".into()))
        .await?;

    while let Some(msg) = read.next().await {
        let message = msg?;

        match message {
            Message::Text(text) => {
                println!("[CLIENT {}] Received text: {:?}", addr, text);

                // Echo back with prefix
                let response = format!("Echo: {}", text);
                write.send(Message::Text(response.into())).await?;
            }
            Message::Binary(data) => {
                println!("[CLIENT {}] Received binary: {} bytes", addr, data.len());

                // Echo back
                write.send(Message::Binary(data)).await?;
            }
            Message::Ping(data) => {
                println!("[CLIENT {}] Received Ping", addr);

                // Automatically respond with Pong
                // (tokio-tungstenite usually handles this automatically)
                write.send(Message::Pong(data)).await?;
            }
            Message::Pong(_) => {
                println!("[CLIENT {}] Received Pong", addr);
            }

            Message::Close(frame) => {
                println!("[CLIENT {}] Received Close frame", addr);
                if let Some(cf) = frame {
                    println!("  Code: {}, Reason: {}", cf.code, cf.reason);
                }

                // Respond with Close frame and break
                write.send(Message::Close(None)).await?;
                break;
            }

            Message::Frame(_) => {
                // Raw frame (not usually seen)
                println!("[CLIENT {}] Received raw frame", addr);
            }
        }
    }
    println!("[CLIENT {}] Connection closed\n", addr);
    Ok(())
}
