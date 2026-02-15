use axum::{
    Router,
    routing::{get, put},
    serve,
};
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
async fn main() {
    let db = Database::new(DB_URL).await;

    if let Err(e) = &db {
        eprintln!("Failed to initialize database: {}", e);
        return;
    }

    let app_ctx = AppContext { db: db.unwrap() };

    let app = Router::new()
        .route(
            "/task",
            put(create_route).patch(update_route).delete(delete_route),
        )
        .route("/task/{id}", get(get_route))
        .route("/count", get(count_route))
        .route("/tasks", get(list_route))
        .fallback(not_found_route)
        .with_state(app_ctx);

    let listener = match TcpListener::bind(HOST).await {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Failed to bind to {HOST}: {}", e);
            return;
        }
    };

    println!("Server running on http://{HOST}");

    if let Err(e) = serve(listener, app).await {
        eprintln!("Server error: {}", e);
    }
}
