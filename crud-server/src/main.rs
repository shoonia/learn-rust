use axum::{
    Router,
    routing::{get, put},
    serve,
};
use std::error::Error;
use tokio::{main, net::TcpListener};

mod consts;
mod context;
mod database;
mod models;
mod routes;

use crate::{
    consts::*,
    context::AppContext,
    database::database::Database,
    routes::{
        count::count_route, create::create_route, delete::delete_route, get::get_route,
        list::list_route, not_found::not_found_route, update::update_route,
    },
};

#[main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db = Database::new(DB_URL).await?;

    let app = Router::new()
        .route(
            "/task",
            put(create_route).patch(update_route).delete(delete_route),
        )
        .route("/task/{id}", get(get_route))
        .route("/count", get(count_route))
        .route("/tasks", get(list_route))
        .fallback(not_found_route)
        .with_state(AppContext { db });

    let listener = TcpListener::bind(HOST).await?;

    println!("Server running on http://{HOST}");
    serve(listener, app).await?;
    Ok(())
}
