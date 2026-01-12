use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

pub fn connect() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("severt listener should be bound");

    println!("[SERVER] listeneing at port 8080");

    thread::spawn(move || {
        let (mut stream, _addr) = listener.accept().unwrap();

        // step 1 : read message length that will be sent
        let mut len_buf = [0u8; 4];
        stream.read_exact(&mut len_buf).unwrap();
        let msg_len: usize = u32::from_be_bytes(len_buf) as usize;

        println!("[SERVER] received incoming message length of {}", msg_len);

        // step 2: use the message length to create a buffer that receive exact byte size to handle partial reds
        let mut buffer = vec![0u8; msg_len];

        stream.read_exact(&mut buffer).unwrap();

        let msg = String::from_utf8_lossy(&buffer);
        println!("[SERVER] ✓ Received complete message: {:?}\n", msg);
    });

    thread::sleep(Duration::from_millis(100));

    // client

    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    println!("[CLIENT] connected to server at 127.0.0.1 8080");

    // create message: get the length of the message
    // write message length to stream first
    //then write the message

    let msg = "Hello, world";
    let msg_bytes = msg.as_bytes();
    let msg_len = msg_bytes.len() as u32;

    println!("[CLIENT] Sending length prefix: {} bytes", msg_len);

    stream.write_all(&msg_len.to_be_bytes()).unwrap();
    println!("[CLIENT] Sending message in 2 chunks...");
    stream.write_all(&msg_bytes[..5]).unwrap();
    thread::sleep(Duration::from_secs(1));
    stream.write_all(&msg_bytes[5..]).unwrap();

    thread::sleep(Duration::from_millis(500));
}
