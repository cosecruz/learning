use std::error::Error;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::signal;
use tokio::sync::broadcast;

#[derive(Clone)]
struct Metrics {
    connections: Arc<AtomicU64>,
    bytes_recvd: Arc<AtomicU64>,
    bytes_sent: Arc<AtomicU64>,
}

impl Metrics {
    fn new() -> Self {
        Self {
            connections: Arc::new(AtomicU64::new(0)),
            bytes_recvd: Arc::new(AtomicU64::new(0)),
            bytes_sent: Arc::new(AtomicU64::new(0)),
        }
    }

    fn report(&self) {
        println!(
            "Connections: {}, RX: {} bytes, TX: {} bytes",
            self.connections.load(Ordering::Relaxed),
            self.bytes_recvd.load(Ordering::Relaxed),
            self.bytes_sent.load(Ordering::Relaxed),
        );
    }
}

pub async fn connect() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on port 8080");

    let metrics = Metrics::new();
    let (shutdown_tx, _) = broadcast::channel(1);

    // Metrics reporter
    let metrics_clone = metrics.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(10));
        loop {
            interval.tick().await;
            metrics_clone.report();
        }
    });

    // Ctrl+C handler
    let shutdown_tx_clone = shutdown_tx.clone();
    tokio::spawn(async move {
        signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
        println!("Received shutdown signal");
        let _ = shutdown_tx_clone.send(());
    });

    let mut shutdown_rx = shutdown_tx.subscribe();

    loop {
        tokio::select! {
            result = listener.accept() => {
                let (socket, addr) = result?;
                println!("New connection from {}", addr);

                let metrics = metrics.clone();
                let shutdown_rx = shutdown_tx.subscribe();

                tokio::spawn(async move {
                    if let Err(e) = handle_client(socket, metrics, shutdown_rx).await {
                        eprintln!("Error handling client: {}", e);
                    }
                });
            }
            _ = shutdown_rx.recv() => {
                println!("Shutting down server");
                break;
            }
        }
    }

    Ok(())
}

async fn handle_client(
    socket: TcpStream,
    metrics: Metrics,
    mut shutdown: broadcast::Receiver<()>,
) -> Result<(), Box<dyn Error>> {
    metrics.connections.fetch_add(1, Ordering::Relaxed);

    let (reader, mut writer) = socket.into_split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    loop {
        tokio::select! {
                            result = reader.read_line(&mut line) => {
                                let n = result?;
                                if n == 0 {
                                    break;
                                }

                                metrics.bytes_recvd.fetch_add(n as u64, Ordering::Relaxed);

                                // writer.write_all(line.as_bytes()).await?;
                                match tokio::time::timeout(
            Duration::from_secs(5),
            writer.write_all(line.as_bytes()),
        ).await {
            Ok(Ok(())) => {
                // write succeeded
                metrics.bytes_sent.fetch_add(n as u64, Ordering::Relaxed);
            }
            Ok(Err(e)) => {
                // I/O error (broken pipe, reset, etc.)
                eprintln!("write error: {}", e);
                return Err(e.into());
            }
            Err(_elapsed) => {
                // ⏱️ WRITE TIMED OUT
                eprintln!("write timed out; disconnecting slow client");
                break; // or return Ok(())
            }
        }


                                metrics.bytes_sent.fetch_add(n as u64, Ordering::Relaxed);

                                line.clear();
                            }
                            _ = shutdown.recv() => {
                                println!("Closing client connection");
                                break;
                            }
                        }
    }

    metrics.connections.fetch_sub(1, Ordering::Relaxed);
    Ok(())
}
