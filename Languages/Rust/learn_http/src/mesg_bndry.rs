// This demonstrates the CRITICAL concept:
// TCP does NOT preserve message boundaries

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

pub fn run_main() {
    println!("=== TCP MESSAGE BOUNDARY DEMONSTRATION ===\n");

    // Server that reads in chunks
    thread::spawn(|| {
        let listener = TcpListener::bind("127.0.0.1:5555").unwrap();
        let (mut stream, _) = listener.accept().unwrap();

        println!("[SERVER] Connected, reading data...\n");

        let mut buffer = [0u8; 10]; // Small buffer on purpose
        let mut read_count = 0;

        loop {
            match stream.read(&mut buffer) {
                Ok(0) => {
                    println!("[SERVER] Connection closed");
                    break;
                }
                Ok(n) => {
                    read_count += 1;
                    let data = String::from_utf8_lossy(&buffer[..n]);
                    println!("[SERVER] Read #{}: Got {} bytes: {:?}", read_count, n, data);
                }
                Err(e) => {
                    println!("[SERVER] Error: {}", e);
                    break;
                }
            }
        }
    });

    thread::sleep(Duration::from_millis(100));

    // Client sends 3 separate messages
    let mut stream = TcpStream::connect("127.0.0.1:5555").unwrap();

    println!("[CLIENT] Sending 3 separate messages:\n");

    stream.write_all(b"Message1").unwrap();
    println!("[CLIENT] Sent: 'Message1'");
    thread::sleep(Duration::from_millis(50));

    stream.write_all(b"Message2").unwrap();
    println!("[CLIENT] Sent: 'Message2'");
    thread::sleep(Duration::from_millis(50));

    stream.write_all(b"Message3").unwrap();
    println!("[CLIENT] Sent: 'Message3'");

    thread::sleep(Duration::from_millis(100));
    drop(stream); // Close connection

    thread::sleep(Duration::from_millis(200));

    println!("\n=== WHAT YOU SHOULD OBSERVE ===");
    println!("The server did NOT receive 3 separate messages!");
    println!("It might have received:");
    println!("  - All 24 bytes in one read()");
    println!("  - Split across multiple read()s at ARBITRARY boundaries");
    println!("  - e.g., 'Message1Me' then 'ssage2Mes' then 'sage3'");
    println!("\nThis is because TCP is a BYTE STREAM, not a message protocol!");

    demonstrate_solution();
}

fn demonstrate_solution() {
    println!("\n\n=== SOLUTION: LENGTH-PREFIXED FRAMING ===\n");

    thread::spawn(|| {
        let listener = TcpListener::bind("127.0.0.1:4444").unwrap();
        let (mut stream, _) = listener.accept().unwrap();

        println!("[SERVER] Reading length-prefixed messages...\n");

        loop {
            // Read 4-byte length prefix
            let mut len_bytes = [0u8; 4];
            match stream.read_exact(&mut len_bytes) {
                Ok(_) => {}
                Err(_) => break, // Connection closed
            }

            let length = u32::from_be_bytes(len_bytes) as usize;
            println!("[SERVER] Expecting message of {} bytes", length);

            // Read exact message length
            let mut msg_buffer = vec![0u8; length];
            stream.read_exact(&mut msg_buffer).unwrap();

            let message = String::from_utf8_lossy(&msg_buffer);
            println!("[SERVER] Complete message: {:?}\n", message);
        }
    });

    thread::sleep(Duration::from_millis(100));

    let mut stream = TcpStream::connect("127.0.0.1:4444").unwrap();

    println!("[CLIENT] Sending length-prefixed messages:\n");

    send_framed_message(&mut stream, "First message");
    send_framed_message(&mut stream, "Second message");
    send_framed_message(&mut stream, "Third message");

    thread::sleep(Duration::from_millis(200));

    println!("\n=== RESULT ===");
    println!("Now the server receives COMPLETE, SEPARATE messages!");
    println!("This is how HTTP, WebSocket, and other protocols work.");
}

fn send_framed_message(stream: &mut TcpStream, message: &str) {
    let bytes = message.as_bytes();
    let length = bytes.len() as u32;

    // Send length prefix (4 bytes, big-endian)
    stream.write_all(&length.to_be_bytes()).unwrap();

    // Send actual message
    stream.write_all(bytes).unwrap();

    println!("[CLIENT] Sent [{}]{:?}", length, message);
}
