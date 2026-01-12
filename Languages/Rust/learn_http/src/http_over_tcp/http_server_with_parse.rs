// HTTP server using httparse library for proper parsing
// Add to Cargo.toml: httparse = "1.8"

use std::io::{Read, Result, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() -> Result<()> {
    println!("=== HTTP SERVER WITH HTTPARSE ===\n");

    let listener = TcpListener::bind("127.0.0.1:8001")?;
    println!("[SERVER] Listening on http://127.0.0.1:8001\n");

    for stream in listener.incoming() {
        let stream = stream?;
        thread::spawn(move || {
            if let Err(e) = handle_connection(stream) {
                eprintln!("Error: {}", e);
            }
        });
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    // Read request into buffer
    let mut buffer = [0u8; 8192];
    let n = stream.read(&mut buffer)?;

    // Parse HTTP request
    let mut headers = [httparse::EMPTY_HEADER; 64];
    let mut req = httparse::Request::new(&mut headers);

    match req.parse(&buffer[..n]) {
        Ok(httparse::Status::Complete(body_offset)) => {
            // Request fully parsed
            let method = req.method.unwrap();
            let path = req.path.unwrap();
            let version = req.version.unwrap();

            println!("[REQUEST] {} {} HTTP/1.{}", method, path, version);

            // Print headers
            for header in req.headers {
                println!(
                    "  {}: {}",
                    header.name,
                    String::from_utf8_lossy(header.value)
                );
            }
            println!();

            // Check for request body
            let body_start = body_offset;
            let body = &buffer[body_start..n];
            if !body.is_empty() {
                println!("[BODY] {} bytes", body.len());
            }

            // Route and respond
            route_request(&mut stream, method, path)?;
        }
        Ok(httparse::Status::Partial) => {
            println!("[WARNING] Partial request received");
            send_response(
                &mut stream,
                400,
                "Bad Request",
                "text/plain",
                b"Request too large or incomplete",
            )?;
        }
        Err(e) => {
            println!("[ERROR] Parse error: {}", e);
            send_response(
                &mut stream,
                400,
                "Bad Request",
                "text/plain",
                b"Malformed request",
            )?;
        }
    }

    Ok(())
}

fn route_request(stream: &mut TcpStream, method: &str, path: &str) -> Result<()> {
    match (method, path) {
        ("GET", "/") => {
            let html = b"<h1>HTTP Server with httparse</h1>\
                         <p>This uses proper HTTP parsing!</p>";
            send_response(stream, 200, "OK", "text/html", html)
        }
        ("GET", "/api/data") => {
            let json = br#"{"status":"success","data":[1,2,3]}"#;
            send_response(stream, 200, "OK", "application/json", json)
        }
        ("POST", "/api/echo") => {
            let response = b"Echo endpoint - POST received";
            send_response(stream, 200, "OK", "text/plain", response)
        }
        _ => {
            let html = format!(
                "<h1>404 Not Found</h1>\
                               <p>Path '{}' not found</p>",
                path
            );
            send_response(stream, 404, "Not Found", "text/html", html.as_bytes())
        }
    }
}

fn send_response(
    stream: &mut TcpStream,
    status: u16,
    status_text: &str,
    content_type: &str,
    body: &[u8],
) -> Result<()> {
    let response = format!(
        "HTTP/1.1 {} {}\r\n\
         Content-Type: {}\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\
         \r\n",
        status,
        status_text,
        content_type,
        body.len()
    );

    stream.write_all(response.as_bytes())?;
    stream.write_all(body)?;

    println!("[RESPONSE] {} {}\n", status, status_text);
    Ok(())
}

// WHAT HTTPARSE SOLVES:
//
// 1. Proper header parsing (case-insensitive, multi-line)
// 2. Request validation
// 3. Handles partial reads
// 4. Efficient zero-copy parsing
// 5. Handles HTTP/0.9, 1.0, 1.1 differences
//
// Without httparse, you'd need hundreds of lines to handle edge cases!
