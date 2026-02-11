use axum::{Json, debug_handler, extract::State};
use serde::Serialize;

use crate::AppContext;

#[derive(Serialize)]
pub struct CountResponse {
    count: i64,
}

#[debug_handler]
pub async fn count_route(State(ctx): State<AppContext>) -> Json<CountResponse> {
    let count = ctx.db.count_tasks().await;
    Json(CountResponse { count })
}
