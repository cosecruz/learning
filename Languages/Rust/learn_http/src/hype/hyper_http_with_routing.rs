use std::convert::Infallible;
use std::error::Error;

use bytes::Bytes;
use http_body_util::Full;
use hyper::body::Incoming;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

async fn run_main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    println!("[SERVER] listening at port 8080");

    loop {
        let (stream, addr) = listener.accept().await?;
        println!(
            "[SERVER] connected at addr {} with peer {:?}",
            addr,
            stream.peer_addr()
        );

        tokio::spawn(async move {
            let io = TokioIo::new(stream);

            let service = service_fn(handle_request);

            if let Err(e) = hyper::server::conn::http1::Builder::new()
                .keep_alive(true)
                .serve_connection(io, service)
                .await
            {
                eprintln!("[SERVER] Connection error: {}", e);
            }
        });
    }
}

async fn handle_request(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    println!("[Request]  {}, {}", method, path);

    let response = match (method, path.as_str()) {
        (Method::GET, "/") => handle_home().await,

        // GET /users/{id}
        (Method::GET, path) if path.starts_with("/users/") => {
            let user_id = path.trim_start_matches("/users/");
            handle_get_user(user_id).await
        }

        // POST /users
        (Method::POST, "/users") => handle_create_user(req).await,

        // Fallback
        _ => handle_not_found().await,
    };

    Ok(response)
}

// Handler: Home page
async fn handle_home() -> Response<Full<Bytes>> {
    let html = r#"
        <h1>API Server</h1>
        <h2>Endpoints:</h2>
        <ul>
            <li>GET /users/:id - Get user by ID</li>
            <li>POST /users - Create user (send JSON)</li>
        </ul>
    "#;

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(Full::new(Bytes::from(html)))
        .unwrap()
}

// Handler: Get user by ID
async fn handle_get_user(user_id: &str) -> Response<Full<Bytes>> {
    #[derive(Serialize)]
    struct User {
        id: String,
        name: String,
        email: String,
    }

    // In real app, this would query database
    let user = User {
        id: user_id.to_string(),
        name: format!("User {}", user_id),
        email: format!("user{}@example.com", user_id),
    };

    let json = serde_json::to_string(&user).unwrap();

    println!("[RESPONSE] 200 OK - User {}", user_id);

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Full::new(Bytes::from(json)))
        .unwrap()
}

// Handler: Create user (reads JSON body)
async fn handle_create_user(req: Request<Incoming>) -> Response<Full<Bytes>> {
    #[derive(Deserialize)]
    struct CreateUser {
        name: String,
        email: String,
    }

    #[derive(Serialize)]
    struct UserResponse {
        id: String,
        name: String,
        email: String,
    }

    // Read body - this is async!
    // Hyper might receive body in chunks due to TCP byte stream
    let body_bytes = match http_body_util::BodyExt::collect(req).await {
        Ok(collected) => collected.to_bytes(),
        Err(_) => {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Full::new(Bytes::from("Failed to read body")))
                .unwrap();
        }
    };

    // Parse JSON
    let create_req: CreateUser = match serde_json::from_slice(&body_bytes) {
        Ok(data) => data,
        Err(_) => {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Full::new(Bytes::from("Invalid JSON")))
                .unwrap();
        }
    };

    // Create user (in real app, save to database)
    let user = UserResponse {
        id: "123".to_string(),
        name: create_req.name,
        email: create_req.email,
    };

    let json = serde_json::to_string(&user).unwrap();

    println!("[RESPONSE] 201 Created");

    Response::builder()
        .status(StatusCode::CREATED)
        .header("Content-Type", "application/json")
        .body(Full::new(Bytes::from(json)))
        .unwrap()
}

// Handler: 404 Not Found
async fn handle_not_found() -> Response<Full<Bytes>> {
    println!("[RESPONSE] 404 Not Found");

    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Full::new(Bytes::from("Not Found")))
        .unwrap()
}
