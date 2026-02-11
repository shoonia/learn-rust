use axum::{Json, debug_handler, extract::State, http::StatusCode, response::IntoResponse};

use crate::{context::AppContext, database::database::Task};

#[debug_handler]
pub async fn update_route(
    State(ctx): State<AppContext>,
    Json(payload): Json<Task>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let Task { id, name, details } = payload;

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

    ctx.db.update_task(id, &name, &details).await;

    Ok((StatusCode::OK, Json(Task { id, name, details })))
}
