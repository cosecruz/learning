//

use std::{
    error::Error,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
    thread,
    time::Duration,
};

struct Stats {
    bytes_sent: AtomicUsize,
    bytes_recvd: AtomicUsize,
}

pub struct MCS;

impl MCS {
    pub fn connect(addr: &str) -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind(addr)?;
        listener.set_nonblocking(true)?;

        println!("[SERVER] listening on {}", addr);

        let shutdown = Arc::new(AtomicBool::new(false));
        let conn_count = Arc::new(AtomicUsize::new(0));
        let stats = Arc::new(Stats {
            bytes_sent: AtomicUsize::new(0),
            bytes_recvd: AtomicUsize::new(0),
        });

        // Shutdown listener
        {
            let shutdown = shutdown.clone();
            thread::spawn(move || {
                let mut line = String::new();
                while std::io::stdin().read_line(&mut line).is_ok() {
                    if line.trim() == "quit" {
                        shutdown.store(true, Ordering::Release);
                        break;
                    }
                    line.clear();
                }
            });
        }

        while !shutdown.load(Ordering::Acquire) {
            match listener.accept() {
                Ok((stream, addr)) => {
                    let shutdown = shutdown.clone();
                    let conn_count = conn_count.clone();
                    let stats = stats.clone();

                    conn_count.fetch_add(1, Ordering::Relaxed);

                    thread::spawn(move || {
                        if let Err(e) =
                            Self::handle_client(stream, addr.to_string(), shutdown, stats)
                        {
                            eprintln!("[CLIENT] error: {}", e);
                        }
                        conn_count.fetch_sub(1, Ordering::Relaxed);
                    });
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(50));
                }
                Err(e) => return Err(Box::new(e)),
            }
        }

        println!(
            "[SERVER] shutdown. bytes sent={}, received={}",
            stats.bytes_sent.load(Ordering::Relaxed),
            stats.bytes_recvd.load(Ordering::Relaxed),
        );

        Ok(())
    }

    fn handle_client(
        stream: TcpStream,
        peer: String,
        shutdown: Arc<AtomicBool>,
        stats: Arc<Stats>,
    ) -> Result<(), Box<dyn Error>> {
        let mut reader = BufReader::new(&stream);
        let mut writer = stream.try_clone()?;

        writer.write_all(b"Welcome! Type 'quit'\n")?;

        let mut line = String::new();

        while !shutdown.load(Ordering::Acquire) {
            line.clear();

            let n = reader.read_line(&mut line)?;
            if n == 0 {
                break;
            }

            stats.bytes_recvd.fetch_add(n, Ordering::Relaxed);

            if line.trim() == "quit" {
                break;
            }

            let response = format!("[ECHO] {}", line);
            writer.write_all(response.as_bytes())?;
            stats
                .bytes_sent
                .fetch_add(response.len(), Ordering::Relaxed);
        }

        println!("[CLIENT-{}] disconnected", peer);
        Ok(())
    }
}
