use axum::{Json, debug_handler, extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use validator::Validate;

use crate::{context::AppContext, database::database::Task};

#[derive(Deserialize, Validate)]

pub struct CreateRequest {
    #[validate(length(
        min = 1,
        max = 255,
        message = "`name` must be between 1 and 255 characters"
    ))]
    pub name: String,

    #[validate(length(
        min = 1,
        max = 255,
        message = "`details` must be between 1 and 255 characters"
    ))]
    pub details: String,
}

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
