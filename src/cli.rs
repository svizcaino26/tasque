use std::fs::File;
use std::io::Seek;

use clap::{Parser, Subcommand, ValueEnum};

use crate::model::{display_tasks, DbState, Task, TaskId, TaskStatus};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add new task
    Add { description: String },
    /// Update task description
    Update { id: u32, new_description: String },
    /// Delete task
    Delete { id: u32 },
    /// Update task status to in-progress
    MarkInProgress { id: u32 },
    /// Set task status to done
    MarkDone { id: u32 },
    /// List tasks (see option)
    List {
        #[arg(value_enum, default_value = "all")]
        option: ListOption,
    },
}

#[derive(ValueEnum, Clone)]
pub enum ListOption {
    All,
    Done,
    Pending,
    InProgress,
}

impl Cli {
    /// # Errors
    pub fn dispatch(
        self,
        db_state: &mut DbState,
        mut db: File,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self.command {
            Commands::Add { description } => {
                let task = Task::new(&db_state, &description);
                let id = db_state.add_task(task);
                db.set_len(0)?;
                db.rewind()?;
                serde_json::to_writer(&db, &db_state)?;
                println!("Task added successfully: (ID: {})", id.value());
                Ok(())
            }
            Commands::Update {
                id,
                new_description,
            } => {
                let id = TaskId::try_from(id)?;
                db_state.update_description(id, new_description)?;
                db.set_len(0)?;
                db.rewind()?;
                serde_json::to_writer(&db, &db_state)?;
                println!("Task updated successfully");
                Ok(())
            }
            Commands::Delete { id } => {
                let id = TaskId::try_from(id)?;
                db_state.delete_task(id)?;
                db.set_len(0)?;
                db.rewind()?;
                serde_json::to_writer(&db, &db_state)?;
                println!("Task successfully deleted");
                Ok(())
            }
            Commands::MarkInProgress { id } => {
                let id = TaskId::try_from(id)?;
                db_state.update_task_status(id, TaskStatus::InProgress)?;
                db.set_len(0)?;
                db.rewind()?;
                serde_json::to_writer(&db, &db_state)?;
                println!("Task updated successfully");
                Ok(())
            }
            Commands::MarkDone { id } => {
                let id = TaskId::try_from(id)?;
                db_state.update_task_status(id, TaskStatus::Done)?;
                db.set_len(0)?;
                db.rewind()?;
                serde_json::to_writer(&db, &db_state)?;
                println!("Task updated successfully");
                Ok(())
            }
            Commands::List { option } => match option {
                ListOption::All => {
                    let tasks = db_state.get_all_tasks();
                    display_tasks(&tasks);
                    Ok(())
                }
                ListOption::Done => {
                    let tasks = db_state.get_tasks_by_status(TaskStatus::Done);
                    display_tasks(&tasks);
                    Ok(())
                }
                ListOption::Pending => {
                    let tasks = db_state.get_tasks_by_status(TaskStatus::Pending);
                    display_tasks(&tasks);
                    Ok(())
                }
                ListOption::InProgress => {
                    let tasks = db_state.get_tasks_by_status(TaskStatus::InProgress);
                    display_tasks(&tasks);
                    Ok(())
                }
            },
        }
    }
}
