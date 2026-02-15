use axum::{Json, debug_handler, extract::State, http::StatusCode, response::IntoResponse};
use validator::Validate;

use crate::{context::AppContext, database::utils::map_error, models::DeleteRequest};

#[debug_handler]
pub async fn delete_route(
    State(ctx): State<AppContext>,
    Json(req): Json<DeleteRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if let Err(e) = req.validate() {
        return Err((StatusCode::BAD_REQUEST, format!("Validation error: {}", e)));
    }

    ctx.db
        .delete_task(req.id, req.revision)
        .await
        .map_err(map_error)?;

    Ok((
        StatusCode::OK,
        format!("Task {} deleted successfully", req.id),
    ))
}
