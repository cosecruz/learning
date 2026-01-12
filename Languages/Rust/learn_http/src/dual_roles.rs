use std::error::Error;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::{error, thread};

#[derive(Clone)]
pub struct ClientServer {
    pub addr: &'static str,
}

impl ClientServer {
    pub fn new(addr: &'static str) -> Self {
        println!("# new server client instance");
        Self { addr }
    }

    fn server(&self) -> Result<(), Box<dyn error::Error>> {
        println!("[server] starting on addr: {}", self.addr);

        let listener = TcpListener::bind(self.addr)?;

        let (mut socket, _addr) = listener.accept()?;

        println!("[server] accepted connection from {} ", self.addr);

        //read data from client
        let mut buf = vec![];

        let n = socket.read(&mut buf)?;

        let recvd = String::from_utf8_lossy(&buf[..n]);
        println!("[server] recvd: {}", recvd);

        //write response back

        let res = "Hello from server!";

        socket
            .write_all(res.as_bytes())
            .expect("Failed to write to stream");

        println!("[server] sent response, closing connection");

        Ok(())
    }
    fn client(&self) -> Result<(), Box<dyn Error>> {
        println!("[client] connected");

        let mut stream = TcpStream::connect(self.addr)?;

        println!("[client connected]");

        //send data to server
        let message = "Hello for client";
        stream.write_all(message.as_bytes())?;

        println!("[client] sent message");

        //read response from server
        let mut buf = [0u8; 1024];
        let n = stream.read(&mut buf)?;

        let res = String::from_utf8_lossy(&buf[..n]);
        println!("[client] recvd: {}", res);

        Ok(())
    }
}

pub fn connect() {
    let sc = ClientServer::new("127.0.0.1:8080");

    let sc1 = sc.clone();
    //spawn server thread
    let server_handl = thread::spawn(move || {
        if let Err(e) = sc1.server() {
            eprintln!("Error: {}", e);
        }
    });

    //give server time to start listeneing
    thread::sleep(Duration::from_millis(100));

    //run client
    match sc.client() {
        Ok(_) => println!("move on"),
        Err(e) => eprintln!("Error: {}", e),
    }

    server_handl.join().unwrap();
}
