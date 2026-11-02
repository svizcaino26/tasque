use std::{fs::OpenOptions, io::Seek};

use clap::Parser;
use tasque::{
    cli::Cli,
    model::{display_tasks, DbState, Task, TaskId, TaskStatus},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut db = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("db.json")?;

    let mut db_state: DbState = if db.metadata()?.len() == 0 {
        DbState::default()
    } else {
        serde_json::from_reader(&db)?
    };

    match cli.command {
        Commands::Add { description } => {
            let task = Task::new(&db_state, &description);
            let id = db_state.add_task(task);
            db.set_len(0)?;
            db.rewind()?;
            serde_json::to_writer(&db, &db_state)?;
            println!("Task added successfully: (ID: {})", id.value());
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
        }
        Commands::Delete { id } => {
            let id = TaskId::try_from(id)?;
            db_state.delete_task(id)?;
            db.set_len(0)?;
            db.rewind()?;
            serde_json::to_writer(&db, &db_state)?;
            println!("Task successfully deleted");
        }
        Commands::MarkInProgress { id } => {
            let id = TaskId::try_from(id)?;
            db_state.update_task_status(id, TaskStatus::InProgress)?;
            db.set_len(0)?;
            db.rewind()?;
            serde_json::to_writer(&db, &db_state)?;
            println!("Task updated successfully");
        }
        Commands::MarkDone { id } => {
            let id = TaskId::try_from(id)?;
            db_state.update_task_status(id, TaskStatus::Done)?;
            db.set_len(0)?;
            db.rewind()?;
            serde_json::to_writer(&db, &db_state)?;
            println!("Task updated successfully");
        }
        Commands::List { option } => match option {
            ListOption::All => {
                let tasks = db_state.get_all_tasks();
                display_tasks(&tasks);
            }
            ListOption::Done => {
                let tasks = db_state.get_tasks_by_status(TaskStatus::Done);
                display_tasks(&tasks);
            }
            ListOption::Pending => {
                let tasks = db_state.get_tasks_by_status(TaskStatus::Pending);
                display_tasks(&tasks);
            }
            ListOption::InProgress => {
                let tasks = db_state.get_tasks_by_status(TaskStatus::InProgress);
                display_tasks(&tasks);
            }
        },
    }
    Ok(())
}
