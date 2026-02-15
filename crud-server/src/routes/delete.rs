use axum::{Json, debug_handler, extract::State, http::StatusCode, response::IntoResponse};
use sqlx::Error;
use validator::Validate;

use crate::{context::AppContext, models::DeleteRequest};

#[debug_handler]
pub async fn delete_route(
    State(ctx): State<AppContext>,
    Json(req): Json<DeleteRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if let Err(e) = req.validate() {
        return Err((StatusCode::BAD_REQUEST, format!("Validation error: {}", e)));
    }

    ctx.db
        .delete_task(req.id, req.revision)
        .await
        .map_err(|e| match e {
            Error::RowNotFound => (StatusCode::NOT_FOUND, "Task not found".to_string()),
            Error::InvalidArgument(_) => (StatusCode::CONFLICT, "Revision mismatch".to_string()),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to delete task: {}", e),
            ),
        })?;

    Ok((
        StatusCode::OK,
        format!("Task {} deleted successfully", req.id),
    ))
}
