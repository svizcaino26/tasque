pub mod cli;
pub mod db;
pub mod error;
pub mod model;

use comfy_table::Table;

use crate::model::Task;

pub fn display_tasks(tasks: &[Task]) {
    let mut table = Table::new();
    table.add_row(vec!["ID", "Description", "Status"]);
    for task in tasks {
        table.add_row(vec![
            task.id().value().to_string(),
            task.description().into(),
            task.status().to_string(),
        ]);
    }
    println!("{table}");
}
