use axum::{Json, debug_handler, extract::State, http::StatusCode, response::IntoResponse};
use validator::Validate;

use crate::{
    context::AppContext,
    errors::{map_error, validation_error},
    models::Task,
};

#[debug_handler]
pub async fn update_route(
    State(ctx): State<AppContext>,
    Json(req): Json<Task>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    req.validate().map_err(validation_error)?;

    let updated_task = ctx
        .db
        .update_task(req.id, req.revision, req.title, req.details)
        .await
        .map_err(map_error)?;

    Ok((StatusCode::OK, Json(updated_task)))
}
