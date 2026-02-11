use axum::{Json, debug_handler, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::context::AppContext;

#[derive(Deserialize)]

pub struct CreateRequest {
    pub name: String,
    pub details: String,
}

#[derive(Serialize)]
pub struct CreateResponse {
    pub id: i64,
    pub name: String,
    pub details: String,
}

#[debug_handler]
pub async fn create_route(
    State(ctx): State<AppContext>,
    Json(payload): Json<CreateRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let CreateRequest { name, details } = payload;

    if name.trim().is_empty() || details.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "`name` and `details` cannot be empty".to_string(),
        ));
    }

    if name.len() > 255 || details.len() > 255 {
        return Err((
            StatusCode::BAD_REQUEST,
            "`name` and `details` cannot exceed 255 characters".to_string(),
        ));
    }

    let id = ctx.db.create_task(&name, &details).await;

    Ok((
        StatusCode::CREATED,
        Json(CreateResponse { id, name, details }),
    ))
}
