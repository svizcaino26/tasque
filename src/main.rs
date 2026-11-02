use clap::Parser;
use std::fs::OpenOptions;
use tasque::{cli::Cli, db::JsonFileDatabase, model::DbState};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let mut db = JsonFileDatabase::new()?;
    let mut db_state = db.read()?;
    cli.dispatch(&mut db_state, db)?;

    Ok(())
}
