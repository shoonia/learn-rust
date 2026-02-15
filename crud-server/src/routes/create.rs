use axum::{Json, debug_handler, extract::State, http::StatusCode, response::IntoResponse};
use validator::Validate;

use crate::{context::AppContext, models::CreateRequest};

#[debug_handler]
pub async fn create_route(
    State(ctx): State<AppContext>,
    Json(payload): Json<CreateRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if let Err(e) = payload.validate() {
        return Err((StatusCode::BAD_REQUEST, format!("Validation error: {}", e)));
    }

    let CreateRequest { title, details } = payload;

    let task = ctx.db.create_task(&title, &details).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create task {}", e),
        )
    })?;

    Ok((StatusCode::CREATED, Json(task)))
}
