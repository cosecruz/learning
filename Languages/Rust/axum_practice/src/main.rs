pub use self::error::{CustomErr, Result};
use crate::model::ModelController;
use axum::{
    Router,
    extract::{Path, Query},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
};
use serde::Deserialize;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod ctx;
mod error;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize ModelController
    let mc = ModelController::new().await?;

    // Protected API routes
    let routes_apis = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    // Main router
    let app = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            web::mw_auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("->> [SERVER] LISTENING on {addr}");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

// region: --- Layers
async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    res
}
// endregion: --- Layers

// region: --- Routes Static
fn routes_static() -> Router {
    Router::new().fallback_service(get_service(ServeDir::new("./")).handle_error(
        |err| async move {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Static file error: {err}"),
            )
        },
    ))
}
// endregion: --- Routes Static

// region: --- Routes Hello
#[derive(Debug, Deserialize)]
struct HelloParam {
    name: Option<String>,
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(hello_handler))
        .route("/hello/{id}", get(hello_p_handler))
}

// Handler with query param: /hello?name=ebuka
async fn hello_handler(Query(params): Query<HelloParam>) -> impl IntoResponse {
    println!("->> {:<12} - hello_handler - {params:?}", "HANDLER");
    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("<strong>Hello, {name}!</strong>"))
}

// Handler with path param: /hello/123
async fn hello_p_handler(Path(id): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - hello_p_handler - {id:?}", "HANDLER");
    Html(format!("<strong>Hello, {id}!</strong>"))
}
// endregion: --- Routes Hello
