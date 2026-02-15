use axum::{
    Json, debug_handler,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use tokio::try_join;
use validator::Validate;

use crate::{
    context::AppContext,
    models::{ListRequest, ListResponse, Pagin},
};

#[debug_handler]
pub async fn list_route(
    State(ctx): State<AppContext>,
    Query(params): Query<ListRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if let Err(e) = params.validate() {
        return Err((StatusCode::BAD_REQUEST, format!("Validation error: {}", e)));
    }

    let limit = params.limit.unwrap_or(100);
    let offset = params.offset.unwrap_or(0);

    let (tasks, total) = try_join!(ctx.db.list_tasks(limit, offset), ctx.db.count_tasks())
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to fetch tasks: {}", e),
            )
        })?;

    let pagin = Pagin {
        total,
        limit,
        offset,
    };

    Ok(Json(ListResponse { tasks, pagin }))
}
