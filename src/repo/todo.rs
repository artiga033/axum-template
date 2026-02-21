use axum::extract::FromRef;
use sqlx::prelude::*;

#[derive(Debug, FromRow)]
pub struct Todo {
    pub id: i64,
    pub description: String,
    pub done: bool,
}

pub struct TodoRepo {
    pool: sqlx::SqlitePool,
}

impl TodoRepo {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_table(&self) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            CREATE TABLE IF NOT EXISTS todo (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                description TEXT NOT NULL,
                done BOOLEAN NOT NULL DEFAULT FALSE
            )
            "#
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn create_todo(&self, entity: Todo) -> Result<Todo, sqlx::Error> {
        let rec = sqlx::query_as!(
            Todo,
            r#"
            INSERT INTO todo (description, done) VALUES (?1, ?2)
            RETURNING id, description, done
            "#,
            entity.description,
            entity.done
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(rec)
    }

    pub async fn list_todos(&self) -> Result<Vec<Todo>, sqlx::Error> {
        let recs = sqlx::query_as!(
            Todo,
            r#"
            SELECT id, description, done FROM todo
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(recs)
    }

    pub async fn mark_done(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE todo SET done = TRUE WHERE id = ?1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn mark_undone(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE todo SET done = FALSE WHERE id = ?1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn delete_todo(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM todo WHERE id = ?1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

impl<S> FromRef<S> for TodoRepo
where
    sqlx::SqlitePool: FromRef<S>,
{
    fn from_ref(s: &S) -> TodoRepo {
        TodoRepo::new(sqlx::SqlitePool::from_ref(s))
    }
}
