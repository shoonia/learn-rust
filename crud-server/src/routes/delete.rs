use axum::{Json, debug_handler, extract::State, http::StatusCode, response::IntoResponse};
use validator::Validate;

use crate::{
    context::AppContext,
    errors::{map_error, validation_error},
    models::DeleteRequest,
};

#[debug_handler]
pub async fn delete_route(
    State(ctx): State<AppContext>,
    Json(req): Json<DeleteRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    req.validate().map_err(validation_error)?;

    ctx.db
        .delete_task(req.id, req.revision)
        .await
        .map_err(map_error)?;

    Ok((
        StatusCode::OK,
        format!("Task {} deleted successfully", req.id),
    ))
}
