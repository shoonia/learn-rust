use axum::{Json, debug_handler, extract::State, http::StatusCode, response::IntoResponse};

use crate::{AppContext, database::utils::map_error, models::CountResponse};

#[debug_handler]
pub async fn count_route(
    State(ctx): State<AppContext>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let count = ctx.db.count_tasks().await.map_err(map_error)?;

    Ok(Json(CountResponse { count }))
}
