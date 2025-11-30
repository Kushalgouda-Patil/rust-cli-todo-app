use sqlx::{SqlitePool, sqlite::SqlitePoolOptions, query};
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use sqlx::FromRow;
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
pub async fn get_db_pool(db_url: &str) -> SqlitePool {
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
        .expect("Failed to connect to SQLite")
}


pub async fn add_task(pool: &SqlitePool, title: &str, description: Option<&str>) -> sqlx::Result<i64> {
    let res = query!(
        r#"INSERT INTO tasks (title, description) VALUES (?, ?)"#,
        title,
        description
    )
    .execute(pool)
    .await?;

    Ok(res.last_insert_rowid())
}

pub async fn list_tasks(pool: &SqlitePool) -> sqlx::Result<Vec<Task>> {
    // println!("Listing tasks from DB");
    let rows = sqlx::query_as::<_, Task>(
        r#"SELECT id, title, description, completed, created_at, updated_at FROM tasks ORDER BY created_at DESC"#
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn mark_done(pool: &SqlitePool, id: i64) -> sqlx::Result<()> {
    // trigger will update `updated_at`
    sqlx::query!(
        r#"UPDATE tasks SET completed = 1 WHERE id = ?"#,
        id
    )
    .execute(pool)
    .await?;
    Ok(())
}


pub async fn delete_task(pool: &SqlitePool, id: i64) -> sqlx::Result<()> {
    sqlx::query!(
        r#"DELETE FROM tasks WHERE id = ?"#,
        id
    )
    .execute(pool)
    .await?;
    Ok(())
}