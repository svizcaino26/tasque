use clap::{Parser, Subcommand, ValueEnum};
use std::{
    fs::OpenOptions,
    io::{BufReader, Seek},
};

use tasque::model::{DbState, Task};

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
        Commands::MarkInProgress { id } => (),
        Commands::MarkDone { id } => (),
        Commands::List { option } => match option {
            ListOption::All => db_state.list_all(),
            ListOption::Done => db_state.list_done(),
            ListOption::Pending => println!("listing all pending tasks"),
            ListOption::InProgress => println!("listing all ongoing tasks"),
        },
    }
    // println!("DB state: {db_state:?}");
    Ok(())
}
