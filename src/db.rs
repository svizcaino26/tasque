use std::fs::{File, OpenOptions};

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
}
