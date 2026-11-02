use std::fs::{File, OpenOptions};

use crate::model::DbState;

pub struct JsonFileDatabase {
    data: File,
}

impl JsonFileDatabase {
    /// Initializes the database.
    ///
    /// # Errors
    ///
    /// - Returns [`std::io::Error`] if database file cannot be opened.
    pub fn new() -> Result<Self, std::io::Error> {
        let db = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("db.json")?;

        Ok(Self { data: db })
    }

    /// Returns the database state read from the database file
    ///
    /// # Errors
    ///
    /// - Returns [`std::io::Error`] if it fails to read metadata from file.
    /// - Returns [`std::error::Error`] if deserialization fails.
    pub fn read(&self) -> Result<DbState, Box<dyn std::error::Error>> {
        let db_state: DbState = if self.data.metadata()?.len() == 0 {
            DbState::default()
        } else {
            serde_json::from_reader(&self.data)?
        };
        Ok(db_state)
    }
}
