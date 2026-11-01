use crate::error::TaskError;
use chrono::{DateTime, Local};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Task {
    id: TaskId,
    description: String,
    status: TaskStatus,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

#[derive(Default, Debug)]
enum TaskStatus {
    #[default]
    Pending,
    InProgress,
    Done,
}

#[derive(Debug, Clone, Copy)]
pub struct TaskId(u32);

impl TryFrom<u32> for TaskId {
    type Error = TaskError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if (1..=u32::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(TaskError::InvalidTaskId)
        }
    }
}

impl Task {
    #[must_use]
    pub fn new(db_state: &DbState, description: &str) -> Self {
        let now = Local::now();
        let id = DbState::next_id(db_state);
        Self {
            id,
            description: description.into(),
            status: TaskStatus::default(),
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug)]
pub struct DbState {
    tasks: HashMap<TaskId, Task>,
    next_id: TaskId,
}

impl DbState {
    #[must_use]
    pub const fn next_id(&self) -> TaskId {
        self.next_id
    }
}
