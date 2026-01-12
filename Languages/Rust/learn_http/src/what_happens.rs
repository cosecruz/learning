// Demonstrates what's happening during system calls
// You'll see timing differences that prove kernel involvement

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::{Duration, Instant};

pub fn run_main() {
    println!("=== System Call Timing Demonstration ===\n");

    // Start server in background
    thread::spawn(|| {
        let listener = TcpListener::bind("127.0.0.1:9999").unwrap();
        let (mut stream, _) = listener.accept().unwrap();

        // Server waits 2 seconds before sending data
        thread::sleep(Duration::from_secs(2));
        stream.write_all(b"Data after 2 seconds").unwrap();
    });

    thread::sleep(Duration::from_millis(100));

    println!("[CLIENT] Connecting...");
    let mut stream = TcpStream::connect("127.0.0.1:9999").unwrap();
    println!("[CLIENT] Connected!\n");

    // Demonstrate blocking read
    println!("=== BLOCKING READ DEMONSTRATION ===");
    println!("[CLIENT] Calling stream.read() - this will BLOCK");
    println!("[CLIENT] My thread is now PAUSED by the OS kernel");
    println!("[CLIENT] I cannot execute ANY code until data arrives\n");

    let start = Instant::now();
    let mut buffer = [0u8; 128];

    // THIS CALL BLOCKS FOR ~2 SECONDS
    let n = stream.read(&mut buffer).unwrap();

    let elapsed = start.elapsed();

    println!("[CLIENT] read() returned after {:?}", elapsed);
    println!(
        "[CLIENT] Received {} bytes: {:?}",
        n,
        String::from_utf8_lossy(&buffer[..n])
    );

    println!("\n=== WHAT HAPPENED ===");
    println!("1. Your Rust code called stream.read()");
    println!("2. This triggered a system call to the kernel");
    println!("3. Kernel saw no data in socket buffer");
    println!("4. OS scheduler put this thread to SLEEP");
    println!("5. Your thread was NOT using CPU during those 2 seconds");
    println!("6. When network data arrived, kernel woke your thread");
    println!("7. read() returned with the data");

    demonstrate_nonblocking();
}

fn demonstrate_nonblocking() {
    println!("\n\n=== NON-BLOCKING MODE DEMONSTRATION ===");

    thread::spawn(|| {
        thread::sleep(Duration::from_millis(100));
        let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
        let (mut stream, _) = listener.accept().unwrap();
        thread::sleep(Duration::from_secs(1));
        stream.write_all(b"Data after 1 second").unwrap();
    });

    thread::sleep(Duration::from_millis(200));

    let stream = TcpStream::connect("127.0.0.1:8888").unwrap();

    // Enable non-blocking mode
    stream.set_nonblocking(true).unwrap();

    println!("[CLIENT] Stream is now in NON-BLOCKING mode");
    println!("[CLIENT] Attempting to read immediately...\n");

    let mut buffer = [0u8; 128];
    let mut attempts = 0;

    loop {
        match stream.read(&mut buffer) {
            Ok(n) if n > 0 => {
                println!(
                    "[CLIENT] SUCCESS! Got {} bytes after {} attempts",
                    n, attempts
                );
                println!("[CLIENT] Data: {:?}", String::from_utf8_lossy(&buffer[..n]));
                break;
            }

            Ok(0) => {
                println!("[CLIENT] Connection closed");
                break;
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                attempts += 1;
                if attempts <= 5 {
                    println!(
                        "[CLIENT] Attempt {}: WouldBlock (no data yet, returned immediately)",
                        attempts
                    );
                }
                thread::sleep(Duration::from_millis(300));
            }
            Err(e) => {
                println!("[CLIENT] Error: {}", e);
                break;
            }
        }
    }

    println!("\n=== KEY DIFFERENCE ===");
    println!("BLOCKING:     read() waits for data (thread sleeps)");
    println!("NON-BLOCKING: read() returns immediately with WouldBlock error");
    println!("              Your code can do other things between attempts");
}
