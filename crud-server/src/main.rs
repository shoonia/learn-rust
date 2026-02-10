use std::sync::{Arc, Mutex};

use axum::{Router, routing::get, serve};
use rusqlite::{Connection, Result};
use tokio::{main, net::TcpListener, task};

const HOST: &str = "0.0.0.0:3000";

fn init_db() -> Result<Arc<Mutex<Connection>>> {
    let db = Connection::open("./target/db.person")?;

    db.execute(
        "CREATE TABLE IF NOT EXISTS person (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            data BLOB
        )",
        (),
    )?;

    Ok(Arc::new(Mutex::new(db)))
}

#[main]
async fn main() {
    let db = init_db().unwrap();

    let handler = task::spawn_blocking(move || {
        db.clone()
            .try_lock()
            .unwrap()
            .query_row("SELECT COUNT(*) FROM person", (), |row| {
                row.get::<_, i64>(0)
            })
            .unwrap()
            .to_string()
    })
    .await
    .unwrap();

    let app = Router::new().route("/count", get(handler));

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
