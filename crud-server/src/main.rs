use axum::{
    Router,
    routing::{get, post},
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
        update::update_route,
    },
};

#[main]
async fn main() {
    let db = Database::new(DB_URL).await;
    let app_ctx = AppContext { db };

    let app = Router::new()
        .route(
            "/task",
            post(create_route).put(update_route).delete(delete_route),
        )
        .route("/task/{id}", get(get_route))
        .route("/count", get(count_route))
        .with_state(app_ctx);

    let listener = match TcpListener::bind(HOST).await {
        Ok(listener) => listener,
        Err(error) => {
            eprintln!("Failed to bind to {HOST}: {}", error);
            return;
        }
    };

    println!("Server running on http://{HOST}");

    if let Err(error) = serve(listener, app).await {
        eprintln!("Server error: {}", error);
    }
}
