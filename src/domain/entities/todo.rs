use std::{fmt::Display, str::FromStr};

use chrono::{DateTime, Utc};

use crate::domain::{errors::todo::TodoError, vo::todo_status::TodoStatus};

#[derive(Debug)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub priority: Priority,
    pub status: TodoStatus,
    pub tags: Vec<String>,
    pub due_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl FromStr for Priority {
    type Err = TodoError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "critical" => Ok(Priority::Critical),
            "high" => Ok(Priority::High),
            "medium" => Ok(Priority::Medium),
            "low" => Ok(Priority::Low),
            _ => Err(TodoError::InvalidPriority(s.to_string())),
        }
    }
}
impl Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let priority_string = match self {
            Priority::Critical => "critical",
            Priority::High => "high",
            Priority::Medium => "medium",
            Priority::Low => "low",
        };
        write!(f, "{}", priority_string)
    }
}

impl Todo {
    pub fn new(
        title: &str,
        description: &str,
        priority: Priority,
        tags: Vec<String>,
        due_at: Option<DateTime<Utc>>,
    ) -> Todo {
        let description = if description.is_empty() {
            None
        } else {
            Some(description.trim().to_string())
        };
        Self {
            id: 1,
            title: title.trim().to_string(),
            description,
            priority,
            status: TodoStatus::InProgress,
            tags,
            due_at,
            completed_at: None,
            updated_at: Utc::now(),
            created_at: Utc::now(),
        }
    }
}
