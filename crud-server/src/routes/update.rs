use axum::{Json, debug_handler, extract::State, http::StatusCode, response::IntoResponse};
use validator::Validate;

use crate::{context::AppContext, models::Task};

#[debug_handler]
pub async fn update_route(
    State(ctx): State<AppContext>,
    Json(payload): Json<Task>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if let Err(e) = payload.validate() {
        return Err((StatusCode::BAD_REQUEST, format!("Validation error: {}", e)));
    }

    let updated_task = ctx
        .db
        .update_task(&payload.id, &payload.title, &payload.details)
        .await;

    Ok((StatusCode::OK, Json(updated_task)))
}
