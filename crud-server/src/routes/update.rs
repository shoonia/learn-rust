use axum::{Json, debug_handler, extract::State, http::StatusCode, response::IntoResponse};
use sqlx::Error;
use validator::Validate;

use crate::{context::AppContext, models::Task};

#[debug_handler]
pub async fn update_route(
    State(ctx): State<AppContext>,
    Json(req): Json<Task>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if let Err(e) = req.validate() {
        return Err((StatusCode::BAD_REQUEST, format!("Validation error: {}", e)));
    }

    let updated_task = ctx
        .db
        .update_task(req.id, req.revision, req.title, req.details)
        .await
        .map_err(|e| match e {
            Error::RowNotFound => (StatusCode::NOT_FOUND, "Task not found".to_string()),
            Error::InvalidArgument(_) => (StatusCode::CONFLICT, "Revision mismatch".to_string()),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to update task: {}", e),
            ),
        })?;

    Ok((StatusCode::OK, Json(updated_task)))
}
