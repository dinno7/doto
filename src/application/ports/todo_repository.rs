use async_trait::async_trait;

use crate::{application::errors::ApplicationError, domain::entities::todo::Todo};

#[async_trait]
pub trait TodoRepository {
    async fn create(&self, todo: Todo) -> Result<(), ApplicationError>;
    async fn get_all(&self) -> Result<Vec<Todo>, ApplicationError>;
    async fn get_all_with_tags(&self) -> Result<Vec<Todo>, ApplicationError>;
}
