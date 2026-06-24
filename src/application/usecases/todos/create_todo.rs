use std::{str::FromStr, sync::Arc};

use chrono::{DateTime, Utc};

use crate::{
    application::{errors::ApplicationError, ports::todo_repository::TodoRepository},
    domain::entities::todo::{Priority, Todo},
};

pub struct CreateTodoUC {
    todo_repo: Arc<dyn TodoRepository>,
}

impl CreateTodoUC {
    pub fn new(todo_repo: Arc<dyn TodoRepository>) -> CreateTodoUC {
        return Self { todo_repo };
    }
}

pub struct CreateTodoInput {
    title: String,
    description: Option<String>,
    priority: Option<String>,
    tags: Vec<String>,
    due_at: Option<DateTime<Utc>>,
}

impl CreateTodoInput {
    pub fn new(
        title: &str,
        description: Option<String>,
        priority: Option<String>,
        tags: Vec<String>,
        due_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            title: title.to_string(),
            description: description.map(|d| d.to_string()),
            priority,
            tags,
            due_at,
        }
    }
}

impl CreateTodoUC {
    pub async fn execute(&self, input: CreateTodoInput) -> Result<(), ApplicationError> {
        let priority = input
            .priority
            .map(|p| Priority::from_str(&p))
            .unwrap_or(Ok(Priority::Low))
            .map_err(|e| ApplicationError::Domain(e.into()))?;

        let desc = input.description.unwrap_or("".to_string());
        let todo = Todo::new(&input.title, &desc, priority, input.tags, input.due_at);

        self.todo_repo.create(todo).await?;
        Ok(())
    }
}
