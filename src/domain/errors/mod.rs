use thiserror::Error;

use crate::domain::errors::todo::TodoError;

pub mod todo;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error(transparent)]
    Todo(#[from] TodoError),
}
