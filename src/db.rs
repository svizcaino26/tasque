use crate::model::DbState;
use std::fs::{File, OpenOptions};
use std::io::Seek;

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

    /// # Errors
    pub fn save(&mut self, db_state: &DbState) -> Result<(), std::io::Error> {
        self.data.set_len(0)?;
        self.data.rewind()?;
        serde_json::to_writer(&self.data, &db_state)?;
        Ok(())
    }
}
