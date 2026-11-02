use std::fs::OpenOptions;

use clap::Parser;
use tasque::{cli::Cli, model::DbState};

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

    cli.dispatch(&mut db_state, db)?;

    Ok(())
}
