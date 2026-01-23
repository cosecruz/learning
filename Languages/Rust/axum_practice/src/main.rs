use crate::model::ModelController;

pub use self::error::{CustomErr, Result};
use std::net::SocketAddr;

use axum::{
    Router,
    extract::{Path, Query},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
};
use serde::Deserialize;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod error;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize ModelController
    let mc: ModelController = ModelController::new().await?;

    // router
    let app = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", web::routes_tickets::routes(mc.clone()))
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    // address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = TcpListener::bind(addr).await.unwrap();
    println!("->> [SERVER] LISTENING on {addr}");

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

// region: layers
async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    res
}
//region: routtes_static

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

#[derive(Debug, Deserialize)]
struct HelloParam {
    name: Option<String>,
}

//region: axum composition of routers -> routtes-Hello
fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(hello_handler))
        .route("/hello/{id}", get(hello_p_handler))
}
// handler
// with query param =  /hello?name=ebuka
async fn hello_handler(Query(params): Query<HelloParam>) -> impl IntoResponse {
    println!("->> {:<12} - hello_handler - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("no name");
    Html(format!("<strong>Hello, {name}</strong>"))
}

//eg /hello2/id
async fn hello_p_handler(Path(id): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - hello_handler - {id:?}", "HANDLER");

    Html(format!("<strong>Hello {id} </strong>"))
}
