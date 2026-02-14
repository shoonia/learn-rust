use sqlx::{Row, Sqlite, SqlitePool, migrate::MigrateDatabase, query, query_as};

use crate::models::Task;

#[derive(Clone)]
pub struct Database {
    pool: SqlitePool,
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
            title VARCHAR(255) NOT NULL,
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
            .unwrap()
            .get::<i64, _>("count")
    }

    pub async fn create_task(&self, title: &str, details: &str) -> Task {
        query_as::<_, Task>(
            "
        INSERT INTO tasks (title, details) VALUES (?, ?)
        RETURNING id, title, details
        ",
        )
        .bind(title)
        .bind(details)
        .fetch_one(&self.pool)
        .await
        .unwrap()
    }

    pub async fn delete_task(&self, id: &i64) -> u64 {
        query("DELETE FROM tasks WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .unwrap()
            .rows_affected()
    }

    pub async fn get_task(&self, id: &i64) -> Task {
        query_as::<_, Task>("SELECT * FROM tasks WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .unwrap()
    }

    pub async fn update_task(&self, id: &i64, title: &str, details: &str) -> Task {
        query_as::<_, Task>(
            "UPDATE tasks SET title = ?, details = ? 
            WHERE id = ?
            RETURNING id, title, details
            ",
        )
        .bind(title)
        .bind(details)
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .unwrap()
    }

    pub async fn list_tasks(&self, limit: &u32, offset: &u32) -> Vec<Task> {
        query_as::<_, Task>("SELECT * FROM tasks LIMIT ? OFFSET ?")
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
            .unwrap()
    }
}
