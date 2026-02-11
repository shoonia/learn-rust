use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::{Row, Sqlite, SqlitePool, migrate::MigrateDatabase, prelude::FromRow, query, query_as};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Task {
    pub id: i64,
    pub name: String,
    pub details: String,
}

pub struct Database {
    pool: SqlitePool,
}

impl Clone for Database {
    fn clone(&self) -> Self {
        Database {
            pool: self.pool.clone(),
        }
    }
}

impl Database {
    pub async fn new(url: &str) -> Self {
        if !Sqlite::database_exists(url).await.unwrap_or(false) {
            Sqlite::create_database(url).await.unwrap();
        }

        let pool = SqlitePool::connect(url).await.unwrap();

        query(
            "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT, 
            name VARCHAR(255) NOT NULL,
            details VARCHAR(255) NOT NULL
        )",
        )
        .execute(&pool)
        .await
        .unwrap();

        Database { pool }
    }

    pub async fn count_tasks(&self) -> i64 {
        query("SELECT COUNT(*) as count FROM tasks")
            .fetch_one(&self.pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
            .unwrap()
            .get::<i64, _>("count")
    }

    pub async fn create_task(&self, name: &str, details: &str) -> i64 {
        query("INSERT INTO tasks (name, details) VALUES (?, ?)")
            .bind(name)
            .bind(details)
            .execute(&self.pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
            .unwrap()
            .last_insert_rowid()
    }

    pub async fn delete_task(&self, id: i64) -> u64 {
        query("DELETE FROM tasks WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
            .unwrap()
            .rows_affected()
    }

    pub async fn get_task(&self, id: i64) -> Task {
        query_as::<_, Task>("SELECT * FROM tasks WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
            .unwrap()
    }
}
