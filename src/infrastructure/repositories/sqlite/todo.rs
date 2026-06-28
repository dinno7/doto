use std::collections::HashMap;

use crate::{
    application::{errors::ApplicationError, ports::todo_repository::TodoRepository},
    domain::{
        entities::todo::{Priority, Todo},
        vo::todo_status::TodoStatus,
    },
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Sqlite, SqlitePool, prelude::FromRow};

pub struct TodoRepositorySqlite {
    db: SqlitePool,
}

impl TodoRepositorySqlite {
    pub fn new(db: SqlitePool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl TodoRepository for TodoRepositorySqlite {
    async fn create(&self, todo: Todo) -> Result<(), ApplicationError> {
        let mut tx = self
            .db
            .begin()
            .await
            .map_err(|e| ApplicationError::Internal(e.into()))?;

        let inserted_todo = sqlx::query!(
            "INSERT INTO 
            todos (title, description, priority, status, due_at, updated_at, created_at) 
            VALUES (?, ?, ?, ?, ?, ?, ?)",
            todo.title,
            todo.description,
            todo.priority.to_string(),
            todo.status.to_string(),
            todo.due_at.map(|d| d.to_rfc3339()),
            todo.updated_at.to_rfc3339(),
            todo.created_at.to_rfc3339(),
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| ApplicationError::Internal(e.into()))?;

        // NOTE: Adding tags
        if !todo.tags.is_empty() {
            let mut qb: sqlx::QueryBuilder<Sqlite> =
                sqlx::QueryBuilder::new("INSERT OR IGNORE INTO tags(name) ");
            qb.push_values(todo.tags.iter(), |mut separated, tag| {
                separated.push_bind(tag);
            });
            qb.build()
                .execute(&mut *tx)
                .await
                .map_err(|e| ApplicationError::Internal(e.into()))?;

            // NOTE: Junction table
            let mut qb: sqlx::QueryBuilder<Sqlite> =
                sqlx::QueryBuilder::new("SELECT id FROM tags WHERE name IN ");
            qb.push_tuples(todo.tags.iter(), |mut separated, tag| {
                separated.push_bind(tag);
            });
            let tag_ids = qb
                .build_query_as::<(i64,)>()
                .fetch_all(&mut *tx)
                .await
                .map_err(|e| ApplicationError::Internal(e.into()))?;

            // NOTE: Insert relation
            let tag_ids: Vec<i64> = tag_ids.into_iter().map(|(tid,)| tid).collect();
            let mut q: sqlx::QueryBuilder<Sqlite> =
                sqlx::QueryBuilder::new("INSERT OR IGNORE INTO todo_tags(todo_id, tag_id) ");
            q.push_values(tag_ids, |mut separated, tagid| {
                separated.push_bind(inserted_todo.last_insert_rowid());
                separated.push_bind(tagid);
            });
            q.build()
                .execute(&mut *tx)
                .await
                .map_err(|e| ApplicationError::Internal(e.into()))?;
        }

        tx.commit()
            .await
            .map_err(|e| ApplicationError::Internal(e.into()))?;

        Ok(())
    }

    async fn get_all(&self) -> Result<Vec<Todo>, ApplicationError> {
        let todos = sqlx::query_as::<_, TodoSqliteEntity>(
            r#"SELECT id, title, description, priority, status, due_at, completed_at, updated_at, created_at FROM todos"#,
        )
        .fetch_all(&self.db)
            .await
        .map_err(|e| ApplicationError::Internal(e.into()))?;
        let todos = todos.into_iter().map(Todo::from).collect::<Vec<Todo>>();
        Ok(todos)
    }

    async fn get_tags_with_todo_ids(
        &self,
        todo_ids: Vec<u64>,
    ) -> Result<HashMap<u64, Vec<String>>, ApplicationError> {
        let mut qb = sqlx::QueryBuilder::new(
            "SELECT todo_id, name FROM todo_tags JOIN tags ON tags.id = tag_id WHERE todo_id IN (",
        );
        let mut sep = qb.separated(", ");

        for todo_id in todo_ids {
            sep.push_bind(todo_id as i64);
        }
        sep.push_unseparated("");

        qb.push(")");

        let records: Vec<(i64, String)> = qb
            .build_query_as()
            .fetch_all(&self.db)
            .await
            .map_err(|e| ApplicationError::Internal(e.into()))?;

        let mut result: HashMap<u64, Vec<String>> = HashMap::new();
        for (todo_id, tag_name) in records {
            result
                .entry(todo_id as u64)
                .and_modify(|tags| {
                    tags.push(tag_name.clone());
                })
                .or_insert(vec![tag_name]);
        }
        Ok(result)
    }

    async fn delete_by_id(&self, todo_id: u64) -> Result<bool, ApplicationError> {
        let result = sqlx::query!("DELETE FROM todos WHERE id = ?", todo_id as i64)
            .execute(&self.db)
            .await;

        match result {
            Ok(r) => {
                if r.rows_affected() == 1 {
                    return Ok(true);
                }
                Ok(false)
            }
            Err(e) => Err(ApplicationError::Internal(e.into())),
        }
    }

    async fn delete_all(&self) -> Result<u64, ApplicationError> {
        let result = sqlx::query!("DELETE FROM todos")
            .execute(&self.db)
            .await
            .map_err(|e| ApplicationError::Internal(e.into()))?;
        Ok(result.rows_affected())
    }

    async fn delete_all_by_status(&self, todo_status: TodoStatus) -> Result<u64, ApplicationError> {
        let result = sqlx::query!(
            "DELETE FROM todos WHERE status = ?",
            todo_status.to_string()
        )
        .execute(&self.db)
        .await
        .map_err(|e| ApplicationError::Internal(e.into()))?;
        Ok(result.rows_affected())
    }
}

#[derive(Debug, FromRow)]
struct TodoSqliteEntity {
    id: i64,
    title: String,
    description: Option<String>,
    priority: String,
    status: String,
    due_at: Option<DateTime<Utc>>,
    completed_at: Option<DateTime<Utc>>,
    updated_at: DateTime<Utc>,
    created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
struct TodoTagSqliteEntity {
    id: i64,
    name: String,
}

impl From<TodoTagSqliteEntity> for String {
    fn from(value: TodoTagSqliteEntity) -> Self {
        value.name
    }
}

impl From<TodoSqliteEntity> for Todo {
    fn from(value: TodoSqliteEntity) -> Self {
        Self {
            id: value.id as u64,
            title: value.title,
            description: value.description,
            priority: value.priority.parse().unwrap_or(Priority::Low),
            status: match value.status.as_str() {
                "pending" => TodoStatus::Pending,
                "in_progress" => TodoStatus::InProgress,
                "done" => TodoStatus::Done,
                "cancelled" => TodoStatus::Cancelled,
                _ => TodoStatus::Pending,
            },
            tags: vec![],
            due_at: value.due_at,
            completed_at: value.completed_at,
            updated_at: value.updated_at,
            created_at: value.created_at,
        }
    }
}

impl From<Todo> for TodoSqliteEntity {
    fn from(
        Todo {
            id,
            title,
            description,
            priority,
            status,
            due_at,
            completed_at,
            updated_at,
            created_at,
            ..
        }: Todo,
    ) -> Self {
        Self {
            id: id as i64,
            title,
            description,
            priority: priority.to_string(),
            status: status.to_string(),
            due_at,
            completed_at,
            updated_at,
            created_at,
        }
    }
}
