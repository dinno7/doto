use std::sync::Arc;

use crate::{
    application::{errors::ApplicationError, ports::todo_repository::TodoRepository},
    domain::{
        entities::todo::{Priority, Todo},
        query::QuerySpec,
    },
};

pub struct GetAllTodosUC {
    todo_repo: Arc<dyn TodoRepository>,
}

impl GetAllTodosUC {
    pub fn new(todo_repo: Arc<dyn TodoRepository>) -> GetAllTodosUC {
        return Self { todo_repo };
    }
}

pub struct GetAllTodosInput {
    pub with_tags: bool,
}

impl GetAllTodosUC {
    pub async fn execute(&self, input: GetAllTodosInput) -> Result<Vec<Todo>, ApplicationError> {
        let mut todos = self.todo_repo.get_all().await?;
        if input.with_tags {
            let tags = self
                .todo_repo
                .get_tags_with_todo_ids(todos.iter().map(|t| t.id).collect::<Vec<u64>>())
                .await?;
            for todo in &mut todos {
                let default_tags: Vec<String> = vec![];
                let todo_tags = tags.get(&todo.id).unwrap_or(&default_tags);
                todo.tags = todo_tags.clone();
            }
        }
        Ok(todos)
    }
}
