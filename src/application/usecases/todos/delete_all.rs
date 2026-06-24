use std::{os::linux::raw::stat, str::FromStr, sync::Arc};

use crate::{
    application::{errors::ApplicationError, ports::todo_repository::TodoRepository},
    domain::vo::todo_status::TodoStatus,
};

pub struct DeleteAllTodosUC {
    todo_repo: Arc<dyn TodoRepository>,
}

impl DeleteAllTodosUC {
    pub fn new(todo_repo: Arc<dyn TodoRepository>) -> DeleteAllTodosUC {
        return Self { todo_repo };
    }
}

impl DeleteAllTodosUC {
    pub async fn execute(&self, status: Option<&str>) -> Result<u64, ApplicationError> {
        if let Some(status) = status {
            let todo_status =
                TodoStatus::from_str(status).map_err(|e| ApplicationError::Domain(e.into()))?;
            let affected = self.todo_repo.delete_all_by_status(todo_status).await?;
            return Ok(affected);
        }

        let affected = self.todo_repo.delete_all().await?;
        Ok(affected)
    }
}
