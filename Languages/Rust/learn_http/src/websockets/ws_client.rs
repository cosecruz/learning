// WebSocket client to test our server
// Add to Cargo.toml:
//   tokio = { version = "1", features = ["full"] }
//   tokio-tungstenite = "0.21"
//   futures-util = "0.3"

use futures_util::{SinkExt, StreamExt};
use std::time::Duration;
use tokio::time::sleep;
use tokio_tungstenite::{connect_async, tungstenite::Message};

pub async fn run_main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== WEBSOCKET CLIENT ===\n");

    // Connect to WebSocket server
    // This sends HTTP Upgrade request and waits for 101 response
    println!("[CLIENT] Connecting to ws://127.0.0.1:9000...");

    let (ws_stream, response) = connect_async("ws://127.0.0.1:9000").await?;

    println!("[CLIENT] Connected!");
    println!("[CLIENT] HTTP response status: {}", response.status());
    println!("[CLIENT] Headers:");
    for (name, value) in response.headers() {
        println!("  {}: {:?}", name, value);
    }
    println!();

    // Split stream for concurrent read/write
    let (mut write, mut read) = ws_stream.split();

    // Spawn task to handle incoming messages
    let read_handle = tokio::spawn(async move {
        while let Some(message) = read.next().await {
            match message {
                Ok(Message::Text(text)) => {
                    println!("[CLIENT] ← Received: {:?}", text);
                }
                Ok(Message::Binary(data)) => {
                    println!("[CLIENT] ← Received binary: {} bytes", data.len());
                }
                Ok(Message::Close(frame)) => {
                    println!("[CLIENT] ← Server closed connection");
                    if let Some(cf) = frame {
                        println!("  Code: {}, Reason: {}", cf.code, cf.reason);
                    }
                    break;
                }
                Ok(Message::Ping(_)) => {
                    println!("[CLIENT] ← Ping");
                }
                Ok(Message::Pong(_)) => {
                    println!("[CLIENT] ← Pong");
                }
                Err(e) => {
                    eprintln!("[CLIENT] Error: {}", e);
                    break;
                }
                _ => {}
            }
        }
    });

    // Send some test messages
    println!("[CLIENT] → Sending test messages...\n");

    sleep(Duration::from_millis(100)).await;

    write
        .send(Message::Text("Hello, WebSocket!".to_string().into()))
        .await?;
    sleep(Duration::from_secs(1)).await;

    write
        .send(Message::Text("This is message 2".to_string().into()))
        .await?;
    sleep(Duration::from_secs(1)).await;

    // Send binary message
    let binary_data = vec![1, 2, 3, 4, 5];
    write.send(Message::Binary(binary_data.into())).await?;
    sleep(Duration::from_secs(1)).await;

    // Send ping
    write.send(Message::Ping(vec![].into())).await?;
    sleep(Duration::from_secs(1)).await;

    // Close connection
    println!("\n[CLIENT] → Closing connection...");
    write.send(Message::Close(None)).await?;

    // Wait for read task to finish
    read_handle.await?;

    println!("[CLIENT] Connection closed gracefully");

    Ok(())
}

// FRAME-LEVEL DETAILS:
//
// When we send Message::Text("Hello"), tungstenite:
//
// 1. Creates frame header:
//    FIN = 1 (final frame)
//    Opcode = 0x1 (text)
//    MASK = 1 (client must mask)
//    Payload len = 5
//
// 2. Generates random 4-byte masking key:
//    e.g., [0xAB, 0xCD, 0xEF, 0x12]
//
// 3. Masks payload:
//    'H' (0x48) XOR 0xAB = 0xE3
//    'e' (0x65) XOR 0xCD = 0xA8
//    'l' (0x6C) XOR 0xEF = 0x83
//    'l' (0x6C) XOR 0x12 = 0x7E
//    'o' (0x6F) XOR 0xAB = 0xC4
//
// 4. Sends frame:
//    [0x81][0x85][0xAB][0xCD][0xEF][0x12][0xE3][0xA8][0x83][0x7E][0xC4]
//     ^^^^  ^^^^  ^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//     FIN   MASK  Masking key         Masked payload
//     +Op   +Len
//
// Server receives and:
// 1. Reads header (FIN=1, Opcode=1, MASK=1, Len=5)
// 2. Reads masking key
// 3. Reads 5 masked bytes
// 4. Unmasks: XOR each byte with mask[i % 4]
// 5. Validates UTF-8 (because opcode=1 is text)
// 6. Returns Message::Text("Hello")

// WEBSOCKET VS HTTP COMPARISON:
//
// HTTP Request/Response cycle:
//   Client → Server: [HTTP headers + body]
//   Server → Client: [HTTP headers + body]
//   [repeat for each request]
//
// WebSocket:
//   Client → Server: [HTTP Upgrade]
//   Server → Client: [101 Switching Protocols]
//   [TCP connection is now WebSocket]
//   Client ↔ Server: [frames] [frames] [frames] ...
//   Either side can send anytime!
//   Client → Server: [Close frame]
//   Server → Client: [Close frame]
//   [TCP FIN]
//
// Key difference: WebSocket is PERSISTENT and BIDIRECTIONAL
