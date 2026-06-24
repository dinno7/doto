use std::{fmt::Display, str::FromStr};

use crate::domain::errors::todo::TodoError;

#[derive(Debug, PartialEq, Eq)]
pub enum TodoStatus {
    Pending,
    InProgress,
    Done,
    Cancelled,
}

impl TodoStatus {
    pub fn is_done(&self) -> bool {
        *self == TodoStatus::Done
    }
}

impl FromStr for TodoStatus {
    type Err = TodoError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "pending" => Ok(TodoStatus::Pending),
            "in_progress" => Ok(TodoStatus::InProgress),
            "done" => Ok(TodoStatus::Done),
            "cancelled" => Ok(TodoStatus::Cancelled),
            _ => Err(TodoError::InvalidPriority(s.to_string())),
        }
    }
}

impl Display for TodoStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status_string = match self {
            TodoStatus::Pending => "pending",
            TodoStatus::InProgress => "in_progress",
            TodoStatus::Done => "done",
            TodoStatus::Cancelled => "cancelled",
        };
        write!(f, "{}", status_string)
    }
}
