use clap::{Parser, Subcommand, ValueEnum};
use std::{fs::OpenOptions, io::BufReader};

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
fn main() {
    let cli = Cli::parse();
    let db = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("db.json")
        .unwrap();

    let reader = BufReader::new(db);

    let mut db_state: DbState = serde_json::from_reader(reader).unwrap_or_default();

    println!("DB state: {db_state:?}");

    match cli.command {
        Commands::Add { description } => {
            let task = Task::new(&db_state, &description);
            let id = db_state.add_task(task);
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
            ListOption::All => println!("listing all tasks"),
            ListOption::Done => println!("listing all done tasks"),
            ListOption::Pending => println!("listing all pending tasks"),
            ListOption::InProgress => println!("listing all ongoing tasks"),
        },
    }
    println!("DB state: {db_state:?}");
}
