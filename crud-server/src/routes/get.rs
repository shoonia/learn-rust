use axum::{
    Json, debug_handler,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{context::AppContext, database::utils::map_error};

#[debug_handler]
pub async fn get_route(
    State(ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let task = ctx.db.get_task(id).await.map_err(map_error)?;

    Ok(Json(task))
}
