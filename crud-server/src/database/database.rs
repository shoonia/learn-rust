use sqlx::{Row, Sqlite, SqlitePool, migrate::MigrateDatabase, query};

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
            .unwrap()
            .get::<i64, _>("count")
    }
}
