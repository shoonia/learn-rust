use axum::{Json, debug_handler, extract::State, http::StatusCode, response::IntoResponse};

use crate::{context::AppContext, models::DeleteRequest};

#[debug_handler]
pub async fn delete_route(
    State(ctx): State<AppContext>,
    Json(payload): Json<DeleteRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let id = payload.id;
    let rows_affected = ctx.db.delete_task(&id).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to delete task: {}", e),
        )
    })?;

    if rows_affected == 0 {
        Ok((
            StatusCode::NOT_FOUND,
            format!("Task with id {} not found", id),
        ))
    } else {
        Ok((StatusCode::OK, format!("Task {} deleted successfully", id)))
    }
}
