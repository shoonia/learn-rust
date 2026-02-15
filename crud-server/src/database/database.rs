use sqlx::{Error, Row, Sqlite, SqlitePool, migrate::MigrateDatabase, query, query_as};

use crate::models::Task;

#[derive(Clone)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(url: &str) -> Result<Self, Error> {
        if !Sqlite::database_exists(url).await.unwrap_or(false) {
            Sqlite::create_database(url).await?;
        }

        let pool = SqlitePool::connect(url).await?;

        query(
            "
            CREATE TABLE IF NOT EXISTS tasks (
              id INTEGER PRIMARY KEY AUTOINCREMENT,
              title VARCHAR(255) NOT NULL,
              details VARCHAR(255) NOT NULL,
              date_created TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
              date_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )",
        )
        .execute(&pool)
        .await?;

        query(
            "
            CREATE TRIGGER IF NOT EXISTS update_tasks_timestamp
            AFTER UPDATE OF title, details ON tasks
            BEGIN
              UPDATE tasks SET date_updated = CURRENT_TIMESTAMP 
              WHERE id = OLD.id;
            END
            ",
        )
        .execute(&pool)
        .await?;

        Ok(Database { pool })
    }

    pub async fn count_tasks(&self) -> Result<i64, Error> {
        query("SELECT COUNT(*) as count FROM tasks")
            .fetch_one(&self.pool)
            .await
            .map(|row| row.get::<i64, _>("count"))
    }

    pub async fn create_task(&self, title: String, details: String) -> Result<Task, Error> {
        query_as::<_, Task>(
            "
            INSERT INTO tasks (title, details) VALUES (?, ?)
            RETURNING *
            ",
        )
        .bind(title)
        .bind(details)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn delete_task(&self, id: i64) -> Result<u64, Error> {
        query("DELETE FROM tasks WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map(|r| r.rows_affected())
    }

    pub async fn get_task(&self, id: i64) -> Result<Task, Error> {
        query_as::<_, Task>("SELECT * FROM tasks WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn update_task(
        &self,
        id: i64,
        title: String,
        details: String,
    ) -> Result<Task, Error> {
        query_as::<_, Task>(
            "UPDATE tasks SET title = ?, details = ?
            WHERE id = ?
            RETURNING *
            ",
        )
        .bind(title)
        .bind(details)
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn list_tasks(&self, limit: u32, offset: u32) -> Result<Vec<Task>, Error> {
        query_as::<_, Task>("SELECT * FROM tasks LIMIT ? OFFSET ?")
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
    }
}
