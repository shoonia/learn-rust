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
              revision INTEGER DEFAULT 1,
              title VARCHAR(255) NOT NULL,
              details VARCHAR(255) NOT NULL,
              date_created TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
              date_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )",
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

    pub async fn delete_task(&self, id: i64, revision: i64) -> Result<(), Error> {
        let result = query(
            "
            DELETE FROM tasks 
            WHERE id = ? AND revision = ?
            ",
        )
        .bind(id)
        .bind(revision)
        .execute(&self.pool)
        .await?;

        match result.rows_affected() {
            0 => {
                if self.is_task_exists(id).await? {
                    Err(Error::InvalidArgument("revision".to_string()))
                } else {
                    Err(Error::RowNotFound)
                }
            }
            _ => Ok(()),
        }
    }

    pub async fn get_task(&self, id: i64) -> Result<Task, Error> {
        let result = query_as::<_, Task>("SELECT * FROM tasks WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        match result {
            Some(task) => Ok(task),
            None => Err(Error::RowNotFound),
        }
    }

    pub async fn update_task(
        &self,
        id: i64,
        revision: i64,
        title: String,
        details: String,
    ) -> Result<Task, Error> {
        let result = query_as::<_, Task>(
            "
            UPDATE tasks SET title = ?, details = ?, revision = revision + 1, date_updated = CURRENT_TIMESTAMP
            WHERE id = ? AND revision = ?
            RETURNING *
            ",
        )
        .bind(title)
        .bind(details)
        .bind(id)
        .bind(revision)
        .fetch_optional(&self.pool)
        .await?;

        match result {
            Some(task) => Ok(task),
            None => {
                if self.is_task_exists(id).await? {
                    Err(Error::InvalidArgument("revision".to_string()))
                } else {
                    Err(Error::RowNotFound)
                }
            }
        }
    }

    pub async fn list_tasks(&self, limit: u32, offset: u32) -> Result<Vec<Task>, Error> {
        query_as::<_, Task>(
            "
            SELECT * FROM tasks
            ORDER BY date_created DESC
            LIMIT ? OFFSET ?
            ",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
    }

    async fn is_task_exists(&self, id: i64) -> Result<bool, Error> {
        let result = query("SELECT 1 FROM tasks WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result.is_some())
    }
}
