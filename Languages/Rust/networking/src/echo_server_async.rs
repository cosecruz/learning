use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub async fn connect() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("server is listening on port 8080");
    loop {
        let (socket, _addr) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = handle_client(socket).await {
                eprintln!("Error: {}", e);
            }
        });
    }
}

async fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = vec![0u8; 1024];

    loop {
        let n = stream.read(&mut buf).await?;
        if n == 0 {
            return Ok(());
        }
        stream.write_all(&buf[..n]).await?;
    }
}

// use tokio::io::{AsyncWriteExt, AsyncReadExt};
// use tokio::net::TcpStream;

// #[tokio::main]
// async fn main() {
//     let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
//     stream.write_all(b"hello\n").await.unwrap();

//     let mut buf = vec![0; 64];
//     let n = stream.read(&mut buf).await.unwrap();
//     println!("{}", String::from_utf8_lossy(&buf[..n]));
// }
