use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use axum::extract::ws::WebSocket;
use axum::extract::{Path, Query, Request, State, WebSocketUpgrade};
use axum::http::{Method, StatusCode};
use axum::middleware::Next;
use axum::response::{Html, IntoResponse};
use axum::routing::{get, post};
use axum::{Json, Router, middleware};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

#[derive(Clone)]
struct AppState {
    users: Arc<Mutex<HashMap<u64, User>>>,
    request_count: Arc<Mutex<u64>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct QueryParams {
    search: Option<String>,
}

#[derive(Serialize)]
struct StatsResponse {
    total_requests: u64,
    total_users: usize,
}

#[tokio::main]
async fn main() {
    // Initialize tracing (logging)
    tracing_subscriber::fmt::init();

    println!("=== AXUM WEB SERVER ===\n");

    // Initialize shared state
    let state = AppState {
        users: Arc::new(Mutex::new(HashMap::new())),
        request_count: Arc::new(Mutex::new(0)),
    };

    // Build router
    // This is where Axum's magic happens: declarative routing
    let app = Router::new()
        // HTML endpoints
        .route("/", get(home_handler))
        .route("/about", get(about_handler))
        .route("/stats", get(stats_handler))
        // API endpoints
        .route("/api/users", post(create_user).get(list_users))
        .route("/api/users/{id}", get(get_user))
        .route("/api/users/{id}", axum::routing::delete(delete_user))
        // WebSocket route
        .route("/ws", get(websocket_handler))
        // Inject shared state into all handlers
        .with_state(state.clone())
        // Apply middleware (order matters!)
        .layer(middleware::from_fn_with_state(
            state.clone(),
            request_counter_middleware,
        ))
        .layer(middleware::from_fn(logging_middleware))
        .layer(
            CorsLayer::new()
                .allow_origin(tower_http::cors::Any)
                .allow_methods([Method::GET, Method::POST]),
        )
        .layer(TraceLayer::new_for_http());

    // Start server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("[SERVER] Listening on http://127.0.0.1:3000");
    println!("\nEndpoints:");
    println!("  GET    /");
    println!("  GET    /about");
    println!("  GET    /api/users");
    println!("  POST   /api/users");
    println!("  GET    /api/users/:id");
    println!("  DELETE /api/users/:id\n");

    axum::serve(listener, app).await.unwrap();
}

// ============================================================================
// MIDDLEWARE
// ============================================================================

// Custom middleware: Count requests
async fn request_counter_middleware(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> impl IntoResponse {
    // Increment counter
    {
        let mut count = state.request_count.lock().unwrap();
        *count += 1;
    }

    // Call next middleware/handler
    next.run(request).await
}

// Custom middleware: Log all requests
async fn logging_middleware(request: Request, next: Next) -> impl IntoResponse {
    let method = request.method().clone();
    let uri = request.uri().clone();

    println!("[MIDDLEWARE] {} {}", method, uri);

    let response = next.run(request).await;

    println!("[MIDDLEWARE] Response status: {}", response.status());

    response
}

// ============================================================================
// HTTP HANDLERS
// ============================================================================

async fn home_handler() -> Html<&'static str> {
    Html(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Full-Stack Axum</title>
            <style>
                body { font-family: sans-serif; max-width: 800px; margin: 50px auto; }
                button { padding: 10px 20px; margin: 5px; cursor: pointer; }
                #messages { border: 1px solid #ccc; padding: 10px; height: 200px; overflow-y: auto; }
            </style>
        </head>
        <body>
            <h1>Full-Stack Axum Server</h1>

            <h2>Features Demonstrated:</h2>
            <ul>
                <li>HTTP routing</li>
                <li>JSON API</li>
                <li>WebSockets</li>
                <li>Middleware (logging, CORS, request counting)</li>
                <li>Shared state</li>
            </ul>

            <h2>WebSocket Test</h2>
            <button onclick="connect()">Connect WebSocket</button>
            <button onclick="sendMessage()">Send Message</button>
            <button onclick="disconnect()">Disconnect</button>
            <div id="messages"></div>

            <h2>Links</h2>
            <ul>
                <li><a href="/stats">Server Stats</a></li>
                <li><a href="/api/users">Users API</a></li>
            </ul>

            <script>
                let ws = null;

                function connect() {
                    ws = new WebSocket('ws://localhost:3000/ws');

                    ws.onopen = () => {
                        addMessage('✓ WebSocket connected');
                    };

                    ws.onmessage = (event) => {
                        addMessage('← Received: ' + event.data);
                    };

                    ws.onclose = () => {
                        addMessage('✗ WebSocket disconnected');
                    };
                }

                function sendMessage() {
                    if (ws && ws.readyState === WebSocket.OPEN) {
                        const msg = 'Hello from browser at ' + new Date().toLocaleTimeString();
                        ws.send(msg);
                        addMessage('→ Sent: ' + msg);
                    } else {
                        addMessage('⚠ Not connected');
                    }
                }

                function disconnect() {
                    if (ws) {
                        ws.close();
                    }
                }

                function addMessage(msg) {
                    const div = document.getElementById('messages');
                    div.innerHTML += msg + '<br>';
                    div.scrollTop = div.scrollHeight;
                }
            </script>
        </body>
        </html>
    "#,
    )
}

async fn stats_handler(State(state): State<AppState>) -> Json<StatsResponse> {
    let count = *state.request_count.lock().unwrap();
    let users_count = state.users.lock().unwrap().len();

    Json(StatsResponse {
        total_requests: count,
        total_users: users_count,
    })
}

// Handler: About page
async fn about_handler() -> Html<&'static str> {
    Html(
        r#"
        <!DOCTYPE html>
        <html>
        <head><title>About</title></head>
        <body>
            <h1>About This Server</h1>
            <p>Built with Axum on top of:</p>
            <ul>
                <li>Tokio (async runtime)</li>
                <li>Hyper (HTTP library)</li>
                <li>Tower (middleware)</li>
                <li>TCP sockets</li>
            </ul>
            <a href="/">Home</a>
        </body>
        </html>
    "#,
    )
}

// Handler: Create user (POST /api/users)
// Axum automatically:
// - Extracts JSON from request body
// - Deserializes into CreateUserRequest
// - Injects AppState
async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> (StatusCode, Json<User>) {
    let mut users = state.users.lock().unwrap();

    let id = users.len() as u64 + 1;
    let user = User {
        id,
        name: payload.name,
        email: payload.email,
    };

    users.insert(id, user.clone());

    println!("[API] Created user: {:?}", user);

    (StatusCode::CREATED, Json(user))
}

// Handler: List users (GET /api/users?search=...)
async fn list_users(
    State(state): State<AppState>,
    Query(params): Query<QueryParams>,
) -> Json<Vec<User>> {
    let users = state.users.lock().unwrap();

    let mut result: Vec<User> = users.values().cloned().collect();

    // Filter by search if provided
    if let Some(search) = params.search {
        result.retain(|u| u.name.to_lowercase().contains(&search.to_lowercase()));
    }

    println!("[API] Listed {} users", result.len());

    Json(result)
}

// Handler: Get single user (GET /api/users/:id)
// Axum extracts :id from path
async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<User>, StatusCode> {
    let users = state.users.lock().unwrap();

    users
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

// Handler: Delete user (DELETE /api/users/:id)
async fn delete_user(State(state): State<AppState>, Path(id): Path<u64>) -> StatusCode {
    let mut users = state.users.lock().unwrap();

    if users.remove(&id).is_some() {
        println!("[API] Deleted user {}", id);
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

// ============================================================================
// WEBSOCKET HANDLER
// ============================================================================

async fn websocket_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    println!("[WS] Client connected");

    // Send welcome message
    if socket
        .send(axum::extract::ws::Message::Text(
            "Welcome to WebSocket!".into(),
        ))
        .await
        .is_err()
    {
        return;
    }

    // Echo loop
    while let Some(msg) = socket.recv().await {
        let msg = match msg {
            Ok(m) => m,
            Err(e) => {
                println!("[WS] Error: {}", e);
                break;
            }
        };

        match msg {
            axum::extract::ws::Message::Text(text) => {
                println!("[WS] Received: {}", text);

                let response = format!("Echo: {}", text);
                if socket
                    .send(axum::extract::ws::Message::Text(response.into()))
                    .await
                    .is_err()
                {
                    break;
                }
            }
            axum::extract::ws::Message::Close(_) => {
                println!("[WS] Client disconnected");
                break;
            }
            _ => {}
        }
    }
}

// WHAT AXUM PROVIDES:
//
// 1. DECLARATIVE ROUTING:
//    .route("/path", get(handler).post(other_handler))
//    Much cleaner than manual matching
//
// 2. EXTRACTORS (Type-safe):
//    - State<T>: Shared state
//    - Path<T>: URL parameters
//    - Query<T>: Query strings
//    - Json<T>: Request body
//    - WebSocketUpgrade: WebSocket handshake
//    All with automatic deserialization!
//
// 3. MIDDLEWARE (Composable):
//    .layer(CorsLayer)
//    .layer(TraceLayer)
//    .layer(custom_middleware)
//    Tower's middleware system
//
// 4. WEBSOCKET INTEGRATION:
//    WebSocketUpgrade automatically:
//    - Validates upgrade request
//    - Performs handshake
//    - Returns WebSocket stream
//
// 5. ERROR HANDLING:
//    Return Result<T, E> and Axum converts errors to HTTP responses
//
// MAPPING TO LOWER LAYERS:
//
// ┌────────────────────────────────────┐
// │ Axum (handlers, routing)           │
// ├────────────────────────────────────┤
// │ Tower (middleware)                 │
// ├────────────────────────────────────┤
// │ Hyper (HTTP parsing)               │
// ├────────────────────────────────────┤
// │ Tokio (async runtime)              │
// ├────────────────────────────────────┤
// │ TCP sockets                        │
// ├────────────────────────────────────┤
// │ OS kernel (epoll/kqueue)           │
// └────────────────────────────────────┘
//
// Everything you learned is still there!
// Axum just makes it ergonomic.
