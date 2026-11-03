# Tasque

A simple task-tracking CLI written in Rust. Tasks are stored in a local JSON file.

The project follows this [spec](https://roadmap.sh/projects/task-tracker) from [roadmap.sh](https://roadmap.sh)

## Features

- Add, update, and delete tasks
- Mark tasks as in progress or done
- List all tasks or filter by status
- Persist tasks between runs in "db.json"

## Requirements

- Rust and Cargo

## Usage

Run commands with Cargo:
```shell
cargo run  <COMMAND>
```

Add a task
```shell
cargo run add "Write the README"
```

Update a task
```shell
cargo run update 1 "Finish the README"
```

Delete a task
```shell
cargo run delete 1
```

Change task status
```shell
cargo run mark-in-progress 1
cargo run mark-done 1
```

List tasks
```shell
cargo run list
cargo run list done
cargo run list pending
cargo run list in-progress
```

The "list" command shows all tasks by default. Tasks are assigned an ID when created, and the JSON database file is created in the current working directory.

Build
```shell
cargo build --release
```
