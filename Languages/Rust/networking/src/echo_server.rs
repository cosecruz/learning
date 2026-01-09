use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

pub fn connect() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    println!("server listening on port 8080");

    for stream in listener.incoming() {
        let stream = stream?;

        thread::spawn(move || {
            if let Err(e) = handle_client(stream) {
                eprintln!("Error handling client: {}", e);
            }
        });
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buf = [0u8; 1024];

    loop {
        let n = stream.read(&mut buf)?;
        if n == 0 {
            //connection closed
            return Ok(());
        }
        stream.write_all(&buf[..n])?;
    }
}
