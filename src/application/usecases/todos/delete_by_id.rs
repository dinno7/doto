use std::sync::Arc;

use crate::{
    application::{errors::ApplicationError, ports::todo_repository::TodoRepository},
    domain::entities::todo::{Priority, Todo},
};

pub struct DeleteTodoByIdUC {
    todo_repo: Arc<dyn TodoRepository>,
}

impl DeleteTodoByIdUC {
    pub fn new(todo_repo: Arc<dyn TodoRepository>) -> DeleteTodoByIdUC {
        return Self { todo_repo };
    }
}

impl DeleteTodoByIdUC {
    pub async fn execute(&self, todo_id: u64) -> Result<bool, ApplicationError> {
        let affected = self.todo_repo.delete_by_id(todo_id).await?;
        if !affected {
            return Err(ApplicationError::NotFound("todo".to_string()));
        }

        Ok(true)
    }
}
