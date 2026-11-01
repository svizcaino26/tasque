use crate::error::TaskError;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    id: TaskId,
    description: String,
    status: TaskStatus,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

#[derive(Default, Debug, Deserialize, Serialize)]
enum TaskStatus {
    #[default]
    Pending,
    InProgress,
    Done,
}

#[derive(Debug, Default, Clone, Copy, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct TaskId(u32);

impl TaskId {
    #[must_use]
    pub const fn value(&self) -> u32 {
        self.0
    }
}

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

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct DbState {
    tasks: HashMap<TaskId, Task>,
    last_id: TaskId,
}
impl DbState {
    #[must_use]
    const fn next_id(&self) -> TaskId {
        TaskId(self.last_id.0.saturating_add(1))
    }

    pub fn add_task(&mut self, task: Task) -> TaskId {
        let id = task.id;
        // let task = Task::new(self.next_id(), description);
        self.tasks.insert(id, task);
        self.last_id = id;
        id
    }
}
