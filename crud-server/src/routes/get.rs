use axum::{
    Json, debug_handler,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::Error;

use crate::context::AppContext;

#[debug_handler]
pub async fn get_route(
    State(ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let task = ctx.db.get_task(id).await.map_err(|e| match e {
        Error::RowNotFound => (
            StatusCode::NOT_FOUND,
            format!("Task with id {} not found", id),
        ),
        _ => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to retrieve task with id {}: {}", id, e),
        ),
    })?;

    Ok(Json(task))
}
