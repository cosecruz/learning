use std::error::Error;
use std::net::SocketAddr;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub async fn connect_tokyo(addr: &'static str) -> Result<(), Box<dyn Error>> {
    let socket: SocketAddr = addr.parse()?;
    let listener = TcpListener::bind(socket).await?;

    println!("[SERVER] listeneing at port {}", socket.port());

    loop {
        let (mut stream, _) = listener.accept().await?;

        println!("[SERVER] new connection for {}", addr);

        tokio::spawn(async move {
            if let Err(e) = handle_conn(stream).await {
                eprintln!("[SERVER] Error: {}", e);
            }
        });
    }
}

async fn handle_conn(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buf = [0u8; 1024];

    loop {
        let n = stream.read(&mut buf).await?;
        if n == 0 {
            // Connection closed
            println!("[SERVER] Connection closed");
            return Ok(());
        }
        println!("[SERVER] Received {} bytes", n);
        stream.write_all(&buf[..n]).await?;
        println!("[SERVER] Echoed {} bytes", n);
    }
}
