use axum::{
    Json, debug_handler,
    extract::{Path, State},
};

use crate::{context::AppContext, models::Task};

#[debug_handler]
pub async fn get_route(State(ctx): State<AppContext>, Path(id): Path<i64>) -> Json<Task> {
    let task = ctx.db.get_task(&id).await;
    Json(task)
}
