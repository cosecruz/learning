// Demonstrates that sockets are just file descriptors
// We'll use OS-specific code to see the actual FD numbers

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

#[cfg(unix)]
use std::os::unix::io::AsRawFd;

#[cfg(windows)]
use std::os::windows::io::AsRawSocket;

pub fn run_main() {
    println!("=== File Descriptor Demonstration ===\n");

    // Start a server
    thread::spawn(|| {
        let listener = TcpListener::bind("127.0.0.1:7777").unwrap();

        #[cfg(unix)]
        println!(
            "[SERVER] Listener file descriptor: {}",
            listener.as_raw_fd()
        );

        #[cfg(windows)]
        println!(
            "[SERVER] Listener socket handle: {}",
            listener.as_raw_socket()
        );

        let (stream, _) = listener.accept().unwrap();

        #[cfg(unix)]
        println!(
            "[SERVER] Accepted connection, new FD: {}",
            stream.as_raw_fd()
        );

        #[cfg(windows)]
        println!(
            "[SERVER] Accepted connection, new handle: {}",
            stream.as_raw_socket()
        );

        thread::sleep(Duration::from_secs(2));
    });

    thread::sleep(Duration::from_millis(100));

    // Client connects
    let stream = TcpStream::connect("127.0.0.1:7777").unwrap();

    #[cfg(unix)]
    {
        let fd = stream.as_raw_fd();
        println!("[CLIENT] Connected, my FD: {}", fd);
        println!("\n=== WHAT IS THIS NUMBER? ===");
        println!("This is an index into the kernel's file descriptor table");
        println!("for this process. The kernel uses it to find socket state.");
        println!("\nStdin=0, Stdout=1, Stderr=2, then sockets start at 3+");
    }

    #[cfg(windows)]
    {
        let handle = stream.as_raw_socket();
        println!("[CLIENT] Connected, my socket handle: {}", handle);
        println!("\n=== WHAT IS THIS NUMBER? ===");
        println!("This is a SOCKET handle (Windows doesn't use Unix FDs for sockets)");
        println!("The kernel uses it to find socket state.");
    }

    // Demonstrate multiple connections = multiple FDs
    println!("\n=== MULTIPLE CONNECTIONS ===");

    let server = thread::spawn(|| {
        let listener = TcpListener::bind("127.0.0.1:6666").unwrap();
        for i in 0..3 {
            let (stream, _) = listener.accept().unwrap();
            #[cfg(unix)]
            println!("[SERVER] Connection {} -> FD {}", i, stream.as_raw_fd());
        }
    });

    thread::sleep(Duration::from_millis(100));

    let mut streams = vec![];
    for i in 0..3 {
        let stream = TcpStream::connect("127.0.0.1:6666").unwrap();

        #[cfg(unix)]
        println!("[CLIENT] Connection {} -> FD {}", i, stream.as_raw_fd());

        streams.push(stream);
    }

    println!("\n=== KEY INSIGHT ===");
    println!("Each connection gets its OWN file descriptor");
    println!("The kernel maintains separate state for each");
    println!("When you drop() a TcpStream, Rust closes that FD");

    server.join().unwrap();
}
