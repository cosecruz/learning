use std::io::{BufRead, BufReader, Result, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;

//HTTP is built on top TCP//Demonstrates what HTTP really is
pub struct TCP;

impl TCP {
    pub fn connect_tcp(addr: &'static str) {
        let socket: SocketAddr = addr.parse().unwrap();
        let listener = TcpListener::bind(socket).unwrap();

        println!("[TCP Server] listening at port {}", socket.port());
        println!("[SERVER] Try visiting in your browser!\n");

        for (i, stream) in listener.incoming().enumerate() {
            let stream = stream.unwrap();

            println!(
                "[SERVER] Connection #{} from {}",
                i,
                stream.peer_addr().unwrap()
            );
            thread::spawn(move || {
                if let Err(e) = Self::handle_http_connection(stream) {
                    eprintln!("Error handling connection: {}", e);
                }
            });
        }
    }

    fn handle_http_connection(mut stream: TcpStream) -> Result<()> {
        let request = Self::read_http_request(&stream)?;

        println!("\n=== RECEIVED HTTP REQUEST ===");
        println!("{}", request);
        println!("==============================\n");

        // Parse the request line
        let lines: Vec<&str> = request.lines().collect();
        if lines.is_empty() {
            return Self::send_error_response(&mut stream, 400, "Bad Request");
        }

        let request_line = lines[0];
        let parts: Vec<&str> = request_line.split_whitespace().collect();

        if parts.len() != 3 {
            return Self::send_error_response(&mut stream, 400, "Bad Request");
        }

        let method = parts[0];
        let path = parts[1];
        let _version = parts[2];

        println!("[SERVER] Method: {}, Path: {}", method, path);

        // Route the request
        match (method, path) {
            ("GET", "/") => Self::send_home_response(&mut stream),
            ("GET", "/hello") => Self::send_hello_response(&mut stream),
            ("GET", "/json") => Self::send_json_response(&mut stream),
            _ => Self::send_error_response(&mut stream, 404, "Not Found"),
        }
    }

    fn read_http_request(stream: &TcpStream) -> Result<String> {
        let mut reader = BufReader::new(stream);
        let mut request = String::new();

        //read until we saee \r\n\r\n (end of header)
        loop {
            let mut line = String::new();

            let bytes_read = reader.read_line(&mut line)?;

            if bytes_read == 0 {
                break; //connection closed
            }

            request.push_str(&line);

            if line == "\r\n" {
                break;
            }
        }
        Ok(request)
    }

    fn send_home_response(stream: &mut TcpStream) -> Result<()> {
        let html = r#"<!DOCTYPE html>
<html>
<head><title>Raw HTTP Server</title></head>
<body>
    <h1>Hello from Raw HTTP Server!</h1>
    <p>This page was served by a hand-written HTTP server in Rust.</p>
    <p>No frameworks, just TCP + text parsing.</p>
    <ul>
        <li><a href="/hello">Hello endpoint</a></li>
        <li><a href="/json">JSON endpoint</a></li>
    </ul>
</body>
</html>"#;

        let response = format!(
            "HTTP/1.1 200 OK\r\n\
         Content-Type: text/html; charset=utf-8\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\
         \r\n\
         {}",
            html.len(),
            html
        );

        stream.write_all(response.as_bytes())?;
        println!("[SERVER] Sent 200 OK response (HTML)\n");
        Ok(())
    }

    fn send_hello_response(stream: &mut TcpStream) -> Result<()> {
        let body = "Hello, World!";

        let response = format!(
            "HTTP/1.1 200 OK\r\n\
         Content-Type: text/plain\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\
         \r\n\
         {}",
            body.len(),
            body
        );

        stream.write_all(response.as_bytes())?;
        println!("[SERVER] Sent 200 OK response (plain text)\n");
        Ok(())
    }

    fn send_json_response(stream: &mut TcpStream) -> Result<()> {
        let json = r#"{"message":"Hello","server":"Raw HTTP","language":"Rust"}"#;

        let response = format!(
            "HTTP/1.1 200 OK\r\n\
         Content-Type: application/json\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\
         \r\n\
         {}",
            json.len(),
            json
        );

        stream.write_all(response.as_bytes())?;
        println!("[SERVER] Sent 200 OK response (JSON)\n");
        Ok(())
    }

    fn send_error_response(stream: &mut TcpStream, status: u16, message: &str) -> Result<()> {
        let body = format!("<h1>{} {}</h1>", status, message);

        let response = format!(
            "HTTP/1.1 {} {}\r\n\
         Content-Type: text/html\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\
         \r\n\
         {}",
            status,
            message,
            body.len(),
            body
        );

        stream.write_all(response.as_bytes())?;
        println!("[SERVER] Sent {} {} response\n", status, message);
        Ok(())
    }
}
