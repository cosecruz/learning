pub use self::error::{CustomErr, Result};
use crate::{ctx::Ctx, error::ClientError, log::log_request, model::ModelController};
use axum::{
    Router,
    extract::{Path, Query},
    http::{Method, StatusCode, Uri},
    middleware,
    response::{Html, IntoResponse, Json, Response},
    routing::{get, get_service},
};
use serde::Deserialize;
use serde_json::json;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use uuid::Uuid;

mod ctx;
mod error;
mod log;
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
async fn main_response_mapper(
    // ctx: Option<Ctx>,
    // uri: Uri,
    // req_method: Method,
    res: Response,
) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    let uuid = Uuid::new_v4();

    // --Get eventual response error.
    let service_error: Option<&CustomErr> = res.extensions().get::<CustomErr>();
    let client_status_error: Option<(StatusCode, ClientError)> =
        service_error.map(|se| se.client_status_and_error());

    // -- if client error, build ne response.
    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error":{
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string()
                }
            });
            println!("      ->> client_error_body: {client_error_body}");

            // Build the new response from the client_error_body
            (*status_code, Json(client_error_body)).into_response()
        });

    // TODO: Build and log the server log line.
    // let client_error = client_status_error.unzip().1;
    // log_request(uuid, req_method, uri, ctx, service_error, client_error).await;
    println!("  ->> server log line -{uuid} - Error: {service_error:?}");

    println!();
    error_response.unwrap_or(res)
}
// endregion: --- Layers

// region: --- Routes Static
fn routes_static() -> Router {
    Router::new().fallback_service(get_service(ServeDir::new("./")).handle_error(
        |err| async move {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
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
