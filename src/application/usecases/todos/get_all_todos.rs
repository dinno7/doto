use std::sync::Arc;

use crate::{
    application::{errors::ApplicationError, ports::todo_repository::TodoRepository},
    domain::entities::todo::{Priority, Todo},
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
        let todos = match input.with_tags {
            true => self.todo_repo.get_all_with_tags().await?,
            false => self.todo_repo.get_all().await?,
        };
        Ok(todos)
    }
}
