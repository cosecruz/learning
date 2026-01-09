use std::io::{self, Read, Write};
use std::net::TcpStream;

pub fn connect() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    stream.set_nonblocking(true)?;

    let buf = b"Hello, server!";
    loop {
        match stream.write(buf) {
            Ok(n) if n == buf.len() => break,
            Ok(n) => {
                // Partial write, would need to track offset
                println!("Wrote {} bytes", n);
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // Socket buffer full, try again later
                continue;
            }
            Err(e) => return Err(e),
        }
    }

    Ok(())
}
