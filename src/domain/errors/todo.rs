use thiserror::Error;

#[derive(Debug, Error)]
pub enum TodoError {
    #[error("{0} is not valid priority")]
    InvalidPriority(String),
}
