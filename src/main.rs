use clap::{Parser, Subcommand, ValueEnum};
use std::{fs::OpenOptions, io::Seek};

use tasque::model::{display_tasks, DbState, Task, TaskId, TaskStatus};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
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
enum ListOption {
    All,
    Done,
    Pending,
    InProgress,
}

#[allow(clippy::unwrap_used)]
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

    // println!("DB state: {db_state:?}");

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
        } => (),
        Commands::Delete { id } => (),
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
    // println!("DB state: {db_state:?}");
    Ok(())
}
