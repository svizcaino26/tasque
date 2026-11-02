use clap::{Parser, Subcommand, ValueEnum};

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

impl Cli {}
