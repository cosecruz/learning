use std::convert::Infallible;
#[allow(dead_code)]
use std::error;

use bytes::Bytes;
use http_body_util::Full;
use hyper::body::Incoming;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

async fn run_main() -> Result<(), Box<dyn error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    println!("[SERVER] listenening at 8080");

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("[SERVER] Connection from {}", addr);

        tokio::spawn(async move {
            let io = TokioIo::new(stream);

            let service = service_fn(handle_request);

            let result = hyper::server::conn::http1::Builder::new()
                .serve_connection(io, service)
                .await;

            if let Err(e) = result {
                eprintln!("[SERVER] connection error: {}", e)
            }
        });
    }
}

async fn handle_request(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    println!("[REQUEST] {} {}", req.method(), req.uri().path());

    for (name, value) in req.headers() {
        println!("  {}: {:?}", name, value);
    }

    let response = match (req.method().as_str(), req.uri().path()) {
        ("GET", "/") => {
            let html = r#"
                <h1>Hyper HTTP Server</h1>
                <p>Powered by Hyper 1.x</p>
            "#;

            Response::builder()
                .status(200)
                .header("Content-Type", "text/html")
                .body(Full::new(Bytes::from(html)))
                .unwrap()
        }
        _ => Response::builder()
            .status(404)
            .body(Full::new(Bytes::from("Not Found")))
            .unwrap(),
    };
    Ok(response)
}
