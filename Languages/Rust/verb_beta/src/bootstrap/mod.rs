use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    http::{Response, StatusCode},
    response::IntoResponse,
    routing::post,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::net::TcpListener;
use tracing::{info, info_span, instrument};

use crate::{
    application::VerbFacade,
    domain::{model::VerbId, repository::VerbRepository},
    error::{AppError, AppResult},
    infra::db::{Database, DatabaseBuilder},
};

#[derive(Debug, Clone)]
struct AppState<D: Database> {
    pub app: Arc<VerbFacade<D>>,
}

#[instrument()]
pub async fn bootstrap() -> AppResult<()> {
    //start bootstrap server

    let db = DatabaseBuilder::new().in_memory().build().await?;
    let app = VerbFacade::new(Arc::from(db));
    let state = AppState { app: Arc::new(app) };

    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    let router = Router::new()
        .route("/", post(create_verb))
        .with_state(state);

    // Start server
    axum::serve(listener, router).await?;

    Ok(())
}

#[derive(Debug, Clone, Deserialize)]
struct CreateVerbDTO {
    title: String,
    description: String,
}

#[derive(Debug, Clone, Serialize)]
struct VerbDTOResponse {
    success: bool,
    id: VerbId,
}

impl IntoResponse for VerbDTOResponse {
    fn into_response(self) -> axum::response::Response {
        let body = Json(json!({
            "success": self.success,
            "id": self.id
        }));

        (StatusCode::CREATED, body).into_response()
    }
}

// create verb handler
#[instrument(skip(state))]
async fn create_verb<D: Database>(
    State(state): State<AppState<D>>,
    Json(dto): Json<CreateVerbDTO>,
) -> impl IntoResponse {
    // Normally:
    // 1. Validate input
    // 2. Call application service / usecase
    // 3. Return DTO

    // Temporary mock logic
    // Application call
    let verb_res = state.app.create_verb(dto.title, dto.description).await;

    match verb_res {
        Ok(v) => {
            tracing::info!(
                verb_id = ?v,
                "verb created successfully"
            );
            return VerbDTOResponse {
                success: true,
                id: v.id(),
            }
            .into_response();
        }

        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

// HTTP DTO  → Application Command → Domain Model
// Domain Result → Application DTO → HTTP Response DTO
