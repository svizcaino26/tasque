use clap::{Parser, Subcommand, ValueEnum};

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

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Add { description } => println!("New task: {description}"),
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
}
