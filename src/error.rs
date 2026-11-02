use std::fmt::Display;

#[derive(Debug)]
pub enum TaskError {
    InvalidTaskId(u32),
    TaskNotFound(u32),
}

impl Display for TaskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidTaskId(id) => write!(f, "invalid task id: {id}"),
            Self::TaskNotFound(id) => write!(f, "task not found: {id}"),
        }
    }
}

impl std::error::Error for TaskError {}
