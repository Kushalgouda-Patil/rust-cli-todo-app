mod db;
use chrono::{FixedOffset, TimeZone, Utc};
use clap::{Args, Parser, Subcommand};
use prettytable::{Table, row,cell, Attr, color};

#[derive(Parser, Debug)]
#[command(
    name = "todo_app",
    version = "1.0",
    author = "Kushal",
    about = "A simple todo list application"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand, Debug)]
enum Commands {
    Add(AddTask),
    List,
    Remove { id: u32 },
    Update(UpdateTask),
}
#[derive(Args, Debug)]
struct AddTask {
    title: String,
    #[arg(short, long)]
    description: Option<String>,
}
#[derive(Args, Debug)]
struct UpdateTask {
    id: u32,
    #[arg(short, long)]
    title: Option<String>,
    #[arg(short, long)]
    description: Option<String>,
    #[arg(short, long)]
    completed: Option<bool>,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    // println!("{:?}", args);
    let pool = db::get_db_pool("sqlite:todo.db").await;
    let ist_offset = FixedOffset::east_opt(5 * 3600 + 30 * 60).unwrap();
    match args.command {
        Commands::Add(addtask) => {
            match db::add_task(&pool, &addtask.title, addtask.description.as_deref()).await {
                Ok(id) => println!("Added task with id: {}", id),
                Err(e) => eprintln!("Error adding task: {}", e),
            }
        }
        Commands::List => match db::list_tasks(&pool).await {
            Ok(tasks) => {
                let mut table = Table::new();
                table.add_row(row![
                    "ID",
                    "Title",
                    "Description",
                    "Completed",
                    "Created At"
                ]);
                for task in tasks {
                    let completed_cell = if task.completed {
                        cell!("✔").with_style(Attr::ForegroundColor(color::GREEN))
                    } else {
                        cell!("✘").with_style(Attr::ForegroundColor(color::RED))
                    };
                    table.add_row(row![
                        task.id,
                        task.title,
                        task.description.unwrap_or_default(),
                        completed_cell,
                        Utc.from_utc_datetime(&task.created_at)
                            .with_timezone(&ist_offset)
                            .format("%d-%m-%Y %H:%M:%S")
                    ]);
                }
                table.printstd();
            }
            Err(e) => eprintln!("Error listing tasks: {}", e),
        },
        Commands::Remove { id } => match db::delete_task(&pool, id as i64).await {
            Ok(_) => println!("Removed task with id: {}", id),
            Err(e) => eprintln!("Error removing task: {}", e),
        },
        Commands::Update(updatetask) => {
            if let Some(completed) = updatetask.completed {
                if completed {
                    match db::mark_done(&pool, updatetask.id as i64).await {
                        Ok(_) => println!("Marked task {} as done", updatetask.id),
                        Err(e) => eprintln!("Error marking task as done: {}", e),
                    }
                }
            }
        }
    }
}
