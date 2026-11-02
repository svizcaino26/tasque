use crate::error::TaskError;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Task {
    id: TaskId,
    description: String,
    status: TaskStatus,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

#[derive(Default, Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy)]
pub enum TaskStatus {
    #[default]
    Pending,
    InProgress,
    Done,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "PENDING"),
            Self::InProgress => write!(f, "IN PROGRESS"),
            Self::Done => write!(f, "DONE"),
        }
    }
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]-[{}]\n<{}>\n",
            self.id.value(),
            self.status,
            self.description
        )
    }
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
            Err(TaskError::InvalidTaskId(value))
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

    #[must_use]
    const fn status(&self) -> &TaskStatus {
        &self.status
    }

    fn set_status(&mut self, status: TaskStatus) {
        self.status = status;
        self.updated_at = Local::now();
    }

    fn set_description(&mut self, new_description: impl Into<String>) {
        self.description = new_description.into();
        self.updated_at = Local::now();
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

    #[must_use]
    pub fn get_tasks_by_status(&self, status: TaskStatus) -> Vec<Task> {
        let mut tasks: Vec<Task> = self
            .tasks
            .values()
            .filter(|task| *task.status() == status)
            .cloned()
            .collect();

        tasks.sort_by_key(|task| task.id.value());

        tasks
    }

    #[must_use]
    pub fn get_all_tasks(&self) -> Vec<Task> {
        let mut tasks: Vec<Task> = self.tasks.values().cloned().collect();

        tasks.sort_by_key(|task| task.id.value());

        tasks
    }

    /// Updates the status of a [`Task`]
    ///
    /// # Errors
    ///
    /// Resturns [`TaskError::TaskNotFound`] if the given ID is not in the database.
    pub fn update_task_status(
        &mut self,
        id: TaskId,
        new_status: TaskStatus,
    ) -> Result<(), TaskError> {
        self.tasks.get_mut(&id).map_or_else(
            || Err(TaskError::TaskNotFound(id.value())),
            |task| {
                task.set_status(new_status);
                Ok(())
            },
        )
    }

    /// Updates the [`Task`] description
    ///
    /// # Errors
    ///
    /// Resturns [`TaskError::TaskNotFound`] if the given ID is not in the database.
    pub fn update_description(
        &mut self,
        id: TaskId,
        new_description: impl Into<String>,
    ) -> Result<(), TaskError> {
        self.tasks.get_mut(&id).map_or_else(
            || Err(TaskError::TaskNotFound(id.value())),
            |task| {
                task.set_description(new_description);
                Ok(())
            },
        )
    }
}

pub fn display_tasks(tasks: &[Task]) {
    for task in tasks {
        println!("{task}");
    }
}
