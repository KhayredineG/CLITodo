# todo — Simple CLI Todo App (Rust)

A tiny command-line todo application written in Rust.

Features
- Add tasks, list tasks, and mark tasks as done.
- Tasks are stored in a local `tasks.json` file in the repo directory.

Prerequisites
- Rust and Cargo (https://www.rust-lang.org/tools/install)

Quick start

Build the project:

```powershell
cargo build --release
```

Run the app (examples):

```powershell
# Add a task
cargo run -- add "Buy groceries"

# List tasks
cargo run -- list

# Mark a task done (use the task ID from list)
cargo run -- done 1
```

Notes
- The app reads/writes `tasks.json` in the current working directory. If it doesn't exist it will be created automatically.
- There is an example `tasks.json` file location in the repo root when you run the app.

Cleaning build artifacts

To remove build artifacts created by Cargo:

```powershell
cargo clean
```

License
- MIT / Apache-2.0 (choose as you prefer)
