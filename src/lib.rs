pub mod cli;
pub mod db;
pub mod error;
pub mod model;

const DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

use comfy_table::{Cell, Table};

use crate::model::Task;

pub fn display_tasks(tasks: &[Task]) {
    let mut table = Table::new();
    table.add_row(vec![
        "ID",
        "Description",
        "Status",
        "Created (Local time)",
        "Updated (Local time)",
    ]);
    for task in tasks {
        table.add_row(vec![
            Cell::new(task.id().value()),
            Cell::new(task.description()),
            Cell::new(task.status()),
            Cell::new(task.created().format(DATE_FORMAT)),
            Cell::new(task.updated().format(DATE_FORMAT)),
        ]);
    }
    println!("{table}");
}
