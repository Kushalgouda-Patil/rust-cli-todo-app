# Rust To-Do App

A simple, command-line to-do list application written in Rust.

## Features

*   **Add tasks:** Add new tasks with a title and an optional description.
*   **List tasks:** View all your tasks in a clean, formatted table.
*   **Mark tasks as done:** Mark tasks as completed.
*   **Remove tasks:** Delete tasks by their ID.
*   **Persistent storage:** Tasks are stored in a local SQLite database (`todo.db`).

## Prerequisites

Before you begin, ensure you have the following installed:

*   [Rust and Cargo](https://www.rust-lang.org/tools/install)
*   [SQLx CLI](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli)

## Installation and Setup

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/your-username/todo-rust-app.git
    cd todo-rust-app
    ```

2.  **Install dependencies:**
    ```bash
    cargo build
    ```

3.  **Set up the database:**
    
    Create a `.env` file in the root of the project and add the following line:
    ```
    DATABASE_URL=sqlite:todo.db
    ```
    
    Then, run the migrations to set up the database schema:
    ```bash
    sqlx database create
    sqlx migrate run
    ```

## Usage

Here's how to use the to-do app from your terminal:

### Add a new task

```bash
cargo run -- add "Buy groceries" -d "Milk, bread, and eggs"
```

### List all tasks

```bash
cargo run -- list
```

### Mark a task as completed

To mark a task with `ID=1` as completed:

```bash
cargo run -- update 1 --completed true
```

### Remove a task

To remove a task with `ID=1`:

```bash
cargo run -- remove 1
```

## Dependencies

*   [clap](https://crates.io/crates/clap): For command-line argument parsing.
*   [sqlx](https://crates.io/crates/sqlx): For interacting with the SQLite database.
*   [tokio](https://crates.io/crates/tokio): Asynchronous runtime.
*   [chrono](https://crates.io/crates/chrono): For date and time handling.
*   [prettytable-rs](https://crates.io/crates/prettytable-rs): For displaying tasks in a formatted table.

## Database Schema

The application uses a single table named `tasks` with the following columns:

*   `id`: INTEGER (Primary Key)
*   `title`: TEXT
*   `description`: TEXT
*   `completed`: BOOLEAN
*   `created_at`: DATETIME
*   `updated_at`: DATETIME
