//! # VERB API
//!
//! This module defines the **backend HTTP API** for the VERB application.
//!
//! ## Overview
//!
//! The VERB API is a **client-facing backend service** accessed via a
//! dedicated API subdomain:
//!
//! ```text
//! api.verb.com
//! ```
//!
//! This API is designed to be:
//! - Client-agnostic (web, CLI, mobile, other services)
//! - Versionable
//! - Protocol-correct (REST over HTTP)
//! - Stable and explicit in its contracts
//!
//! ## API Design Principles
//!
//! - **Resources are nouns**, not verbs
//! - **HTTP methods express actions**
//! - Paths represent **collections or individual resources**
//! - Responses are JSON unless otherwise specified
//!
//! ## Base URL
//!
//! ```text
//! https://api.verb.com
//! ```
//!
//! In development:
//!
//! ```text
//! http://localhost:8080
//! ```
//!
//! ## Versioning
//!
//! All API routes are versioned:
//!
//! ```text
//! /v1/...
//! ```
//!
//! This allows future versions to coexist without breaking clients.
//!
//! ## Core Resource
//!
//! The primary resource managed by this API is:
//!
//! - `Todo`
//!
//! A `Todo` represents a single actionable item in the VERB system.
//!
//! ## REST Endpoints (v1)
//!
//! | Method | Path              | Description                  |
//! |--------|-------------------|------------------------------|
//! | GET    | /todos            | Fetch all todos              |
//! | POST   | /todos            | Create a new todo            |
//! | GET    | /todos/{id}       | Fetch a todo by ID           |
//! | PUT    | /todos/{id}       | Update an existing todo      |
//! | DELETE | /todos/{id}       | Delete a todo                |
//!
//! ## Notes
//!
//! - Authentication and authorization are handled via middleware
//! - Validation errors return `400 Bad Request`
//! - Missing resources return `404 Not Found`
//! - Successful mutations return the updated resource
//!
//! This module exposes only routing and HTTP concerns.
//! Business logic lives in the domain layer.

use std::net::SocketAddr;

use axum::routing::get;
use axum::{Router, serve};

use crate::api::routers::router;
mod handlers;
mod routers;
///
/// This is where all the apis are connected and called to start the web_server_api
/// -routers
/// -general middlewares
/// - and otherwise
pub(super) async fn serve_api() {
    //a router needs routes, handlers, middleware and other dependency
    let app = router();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("->> [SERVER] LISTENING on {addr}\n");

    serve(listener, app).await.unwrap();
}
