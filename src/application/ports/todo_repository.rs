use async_trait::async_trait;

use crate::{
    application::errors::ApplicationError,
    domain::{entities::todo::Todo, vo::todo_status::TodoStatus},
};

#[async_trait]
pub trait TodoRepository: Send + Sync {
    async fn create(&self, todo: Todo) -> Result<(), ApplicationError>;
    async fn get_all(&self) -> Result<Vec<Todo>, ApplicationError>;
    async fn get_all_with_tags(&self) -> Result<Vec<Todo>, ApplicationError>;
    async fn delete_by_id(&self, todo_id: u64) -> Result<bool, ApplicationError>;
    async fn delete_all(&self) -> Result<u64, ApplicationError>;
    async fn delete_all_by_status(&self, todo_status: TodoStatus) -> Result<u64, ApplicationError>;
}
