use axum::{Router, routing::get, serve};
use tokio::{main, net::TcpListener};

mod model;
mod routes;

const HOST: &str = "0.0.0.0:3000";

#[main]
async fn main() {
    let app = Router::new()
        .route(
            "/calculate",
            get(routes::calculate_get).post(routes::calculate_post),
        )
        .route("/random.png", get(routes::random_png))
        .route("/redirect", get(routes::redirect))
        .route("/", get(routes::root));

    let listener = match TcpListener::bind(HOST).await {
        Ok(listener) => listener,
        Err(error) => {
            eprintln!("Failed to bind to {HOST}: {}", error);
            return;
        }
    };

    match serve(listener, app).await {
        Ok(_) => println!("Server stopped gracefully."),
        Err(error) => eprintln!("Server error: {}", error),
    }
}
