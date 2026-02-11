use axum::{
    Json, debug_handler,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use validator::Validate;

use crate::{
    context::AppContext,
    models::{ListRequest, ListResponse},
};

#[debug_handler]
pub async fn list_route(
    State(ctx): State<AppContext>,
    Query(params): Query<ListRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if let Err(e) = params.validate() {
        return Err((StatusCode::BAD_REQUEST, format!("Validation error: {}", e)));
    }

    let limit = params.limit.unwrap_or(10);
    let offset = params.offset.unwrap_or(0);

    let tasks = ctx.db.list_tasks(&limit, &offset).await;

    Ok(Json(ListResponse { tasks }))
}
