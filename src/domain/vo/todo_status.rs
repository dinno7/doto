#[derive(Debug, PartialEq, Eq)]
pub enum TodoStatus {
    Pending,
    InProgress,
    Done,
    Cancelled,
}

impl TodoStatus {
    fn is_done(&self) -> bool {
        *self == TodoStatus::Done
    }
}
