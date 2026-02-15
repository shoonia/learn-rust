use axum::{Json, debug_handler, extract::State, http::StatusCode, response::IntoResponse};

use crate::{AppContext, models::CountResponse};

#[debug_handler]
pub async fn count_route(
    State(ctx): State<AppContext>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let count = ctx.db.count_tasks().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to count tasks: {}", e),
        )
    })?;

    Ok(Json(CountResponse { count }))
}
