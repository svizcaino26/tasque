use clap::{Parser, Subcommand};

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
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Add { description } => println!("New task: {description}"),
        Commands::Update {
            id,
            new_description,
        } => (),
        Commands::Delete { id } => (),
        Commands::MarkInProgress { id } => (),
        Commands::MarkDone { id } => (),
    }
}
