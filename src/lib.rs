pub mod cli;
pub mod db;
pub mod error;
pub mod model;

use comfy_table::{Cell, Table};

use crate::model::Task;

pub fn display_tasks(tasks: &[Task]) {
    let mut table = Table::new();
    table.add_row(vec!["ID", "Description", "Status"]);
    for task in tasks {
        table.add_row(vec![
            Cell::new(task.id().value()),
            Cell::new(task.description()),
            Cell::new(task.status()),
        ]);
    }
    println!("{table}");
}
