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
    errors::{map_error, validation_error},
    models::{ListRequest, ListResponse, Pagin},
};

#[debug_handler]
pub async fn list_route(
    State(ctx): State<AppContext>,
    Query(req): Query<ListRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    req.validate().map_err(validation_error)?;

    let limit = req.limit.unwrap_or(100);
    let offset = req.offset.unwrap_or(0);

    let (tasks, total) =
        try_join!(ctx.db.list_tasks(limit, offset), ctx.db.count_tasks()).map_err(map_error)?;

    let pagin = Pagin {
        total,
        limit,
        offset,
    };

    Ok(Json(ListResponse { tasks, pagin }))
}
