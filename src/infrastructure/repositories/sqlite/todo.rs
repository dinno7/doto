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

    async fn get_all_with_tags(&self) -> Result<Vec<Todo>, ApplicationError> {
        let todos = self.get_all().await?;

        let tags = sqlx::query_as::<_, (i64, String)>(
            r#"
            SELECT tt.todo_id, tg.name
            FROM tags tg
            JOIN todo_tags tt ON tt.tag_id = tg.id
            WHERE tt.todo_id IN (SELECT id FROM todos ORDER BY id)
            ORDER BY tt.todo_id
            "#,
        )
        .fetch_all(&self.db)
        .await
        .map_err(|e| ApplicationError::Internal(e.into()))?;

        let mut tags_map: HashMap<i64, Vec<String>> = HashMap::new();
        for (todo_id, tag_name) in tags {
            tags_map
                .entry(todo_id)
                .and_modify(|t| t.push(tag_name.clone()))
                .or_insert(vec![tag_name]);
        }

        let todos: Vec<Todo> = todos
            .into_iter()
            .map(|t| {
                let mut todo = t;
                todo.tags = tags_map.remove(&(todo.id as i64)).unwrap_or_default();
                todo
            })
            .collect();

        Ok(todos)
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
