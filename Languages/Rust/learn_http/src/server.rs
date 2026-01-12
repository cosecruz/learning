use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{error, thread};

pub struct Server {
    pub addr: &'static str,
}

impl Server {
    pub fn new(addr: &'static str) -> Self {
        Self { addr }
    }

    pub fn connect(&self) {
        let listener = TcpListener::bind(self.addr).unwrap();

        println!("Server listeneing at address: {}", self.addr);

        loop {
            let (socket, _addr) = listener.accept().unwrap();
            thread::spawn(move || {
                if let Err(e) = Self::handle_conn(socket) {
                    eprintln!("Error: {}", e)
                }
            });
        }
    }

    fn handle_conn(mut stream: TcpStream) -> Result<(), Box<dyn error::Error>> {
        let mut buf = vec![0u8; 1024];

        loop {
            let n = stream.read(&mut buf)?;
            if n == 0 {
                return Ok(());
            }

            stream.write_all(&buf[..n])?
        }
    }
}
