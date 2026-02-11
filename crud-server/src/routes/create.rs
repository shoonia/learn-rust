use axum::{Json, debug_handler, extract::State, http::StatusCode, response::IntoResponse};
use validator::Validate;

use crate::{
    context::AppContext,
    models::{CreateRequest, Task},
};

#[debug_handler]
pub async fn create_route(
    State(ctx): State<AppContext>,
    Json(payload): Json<CreateRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if let Err(e) = payload.validate() {
        return Err((StatusCode::BAD_REQUEST, format!("Validation error: {}", e)));
    }

    let CreateRequest { name, details } = payload;

    let id = ctx.db.create_task(&name, &details).await;

    Ok((StatusCode::CREATED, Json(Task { id, name, details })))
}
