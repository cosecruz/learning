use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};

use crate::web::AppState;

///
///
///CreateVerb
#[derive(Debug, Clone, Deserialize)]
pub struct CreateVerb {
    pub name: String,
    pub email: String,
}

/// Verb
#[derive(Debug, Serialize)]
pub struct Verb {
    id: u64,
    name: String,
    email: String,
}

///Handlers for routes
///
pub async fn create_verb(
    // Json(payload): Json<CreateVerb>,
    State(_state): State<AppState>,
) -> Json<Verb> {
    // prototype
    // let verb = Verb {
    //     id: 1,
    //     name: payload.name,
    //     email: payload.email,
    // };

    Json(Verb {
        id: 1,
        name: "".into(),
        email: "".into(),
    })
}
