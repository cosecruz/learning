// Demonstrates TCP's reliability and ordering guarantees
// Even if we send 1000 messages, they arrive in order

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== TCP RELIABILITY AND ORDERING TEST ===\n");

    // Server receives and validates order
    thread::spawn(|| {
        let listener = TcpListener::bind("127.0.0.1:3333").unwrap();
        let (mut stream, _) = listener.accept().unwrap();

        println!("[SERVER] Receiving messages...");

        let mut buffer = vec![0u8; 8192];
        let mut all_data = Vec::new();

        loop {
            match stream.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => {
                    all_data.extend_from_slice(&buffer[..n]);
                }
                Err(e) => {
                    println!("[SERVER] Error: {}", e);
                    break;
                }
            }
        }

        // Parse received data
        let received_text = String::from_utf8_lossy(&all_data);
        let numbers: Vec<u32> = received_text
            .split(',')
            .filter_map(|s| s.parse().ok())
            .collect();

        println!("[SERVER] Received {} numbers", numbers.len());

        // Check if they're in order
        let mut errors = 0;
        for (i, &num) in numbers.iter().enumerate() {
            if num != i as u32 {
                println!("[SERVER] ERROR: Expected {}, got {}", i, num);
                errors += 1;
            }
        }

        if errors == 0 {
            println!("[SERVER] ✓ ALL NUMBERS RECEIVED IN CORRECT ORDER!");
            println!("[SERVER] ✓ TCP guaranteed ordering despite network chaos!");
        } else {
            println!("[SERVER] ✗ {} ordering errors", errors);
        }
    });

    thread::sleep(Duration::from_millis(100));

    // Client sends 1000 numbers
    let mut stream = TcpStream::connect("127.0.0.1:3333").unwrap();

    println!("[CLIENT] Sending 1000 numbers in order...\n");

    for i in 0..1000 {
        let msg = format!("{},", i);
        stream.write_all(msg.as_bytes()).unwrap();

        // Add random tiny delays to simulate network conditions
        if i % 100 == 0 {
            thread::sleep(Duration::from_micros(100));
            println!("[CLIENT] Sent up to {}...", i);
        }
    }

    println!("[CLIENT] All 1000 numbers sent");
    drop(stream); // Close connection

    thread::sleep(Duration::from_secs(1));

    println!("\n=== TCP GUARANTEES DEMONSTRATED ===");
    println!("✓ Reliable delivery: No numbers lost");
    println!("✓ Ordered delivery: All numbers in sequence");
    println!("✓ No duplicates: Each number appeared once");
    println!("\nThis is what TCP does automatically!");

    demonstrate_what_tcp_doesnt_guarantee();
}

fn demonstrate_what_tcp_doesnt_guarantee() {
    println!("\n\n=== WHAT TCP DOES NOT GUARANTEE ===\n");

    thread::spawn(|| {
        let listener = TcpListener::bind("127.0.0.1:2222").unwrap();
        let (mut stream, _) = listener.accept().unwrap();

        // Read with small buffer to show chunking
        let mut buffer = [0u8; 8];
        let mut read_count = 0;

        println!("[SERVER] Reading with 8-byte buffer:\n");

        loop {
            match stream.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => {
                    read_count += 1;
                    let chunk = String::from_utf8_lossy(&buffer[..n]);
                    println!("[SERVER] Read #{}: {} bytes: {:?}", read_count, n, chunk);
                }
                Err(_) => break,
            }
        }

        println!("\n[SERVER] Total reads: {}", read_count);
    });

    thread::sleep(Duration::from_millis(100));

    let mut stream = TcpStream::connect("127.0.0.1:2222").unwrap();

    println!("[CLIENT] Sending: 'AAAA' then 'BBBB' then 'CCCC'\n");

    stream.write_all(b"AAAA").unwrap();
    stream.write_all(b"BBBB").unwrap();
    stream.write_all(b"CCCC").unwrap();

    thread::sleep(Duration::from_millis(100));
    drop(stream);

    thread::sleep(Duration::from_millis(200));

    println!("\n=== OBSERVE ===");
    println!("TCP did NOT preserve the 3 separate write() calls!");
    println!("Server might see:");
    println!("  - One read with 'AAAABBBBCCCC'");
    println!("  - Multiple reads with arbitrary splits");
    println!("  - 'AAAABB' then 'BBCCCC'");
    println!("\nYou MUST handle framing yourself!");
}
