use axum::{Json, debug_handler, extract::State, http::StatusCode, response::IntoResponse};
use validator::Validate;

use crate::{
    context::AppContext,
    errors::{map_error, validation_error},
    models::CreateRequest,
};

#[debug_handler]
pub async fn create_route(
    State(ctx): State<AppContext>,
    Json(req): Json<CreateRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    req.validate().map_err(validation_error)?;

    let task = ctx
        .db
        .create_task(req.title, req.details)
        .await
        .map_err(map_error)?;

    Ok((StatusCode::CREATED, Json(task)))
}
