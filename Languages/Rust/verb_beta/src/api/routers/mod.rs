use axum::Router;
use axum::routing::get;

mod router_v1;

/// route is the exposed api to the rest of the routers external crates access routers through this api
pub fn router() -> Router {
    Router::new().route("/", get(|| async { "router" }))
}

// api -> Router-.
