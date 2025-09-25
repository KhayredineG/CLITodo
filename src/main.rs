use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter};
use std::path::Path;

#[derive(Parser)]
#[command(name = "todo", version = "0.1.0", about = "A simple todo app")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser, Debug)]
enum Commands {
    Add {
        description: Vec<String>,
    },
    List,
    Done {
        id: usize,
    },
}


#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let tasks = load_tasks("tasks.json")?;

    match cli.command {
        Commands::Add { description } => {
            let description = description.join(" ");
            add_task("tasks.json", description)?;
        }
        Commands::List => list_tasks(&tasks),
        Commands::Done { id } => {
            complete_task("tasks.json", id)?;
        }
    }

    Ok(())
}

fn load_tasks<P: AsRef<Path>>(path: P) -> io::Result<Vec<Task>> {
    if !path.as_ref().exists() {
        return Ok(Vec::new());
    }
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let tasks = serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new());
    Ok(tasks)
}

fn save_tasks<P: AsRef<Path>>(path: P, tasks: &[Task]) -> io::Result<()> {
    let file = OpenOptions::new().write(true).create(true).truncate(true).open(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, tasks)?;
    Ok(())
}

fn add_task<P: AsRef<Path>>(path: P, description: String) -> io::Result<()> {
    let mut tasks = load_tasks(path.as_ref())?;
    let new_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    let new_task = Task {
        id: new_id,
        description,
        completed: false,
    };
    tasks.push(new_task);
    save_tasks(path, &tasks)?;
    println!("Added task {}.", new_id);
    Ok(())
}

fn list_tasks(tasks: &[Task]) {
    if tasks.is_empty() {
        println!("No tasks yet!");
    } else {
        for task in tasks {
            let status = if task.completed { "[x]" } else { "[ ]" };
            println!("{} {} - {}", status, task.id, task.description);
        }
    }
}

fn complete_task<P: AsRef<Path>>(path: P, id: usize) -> io::Result<()> {
    let mut tasks = load_tasks(path.as_ref())?;
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.completed = true;
        save_tasks(path, &tasks)?;
        println!("Completed task {}.", id);
    } else {
        println!("Task with ID {} not found.", id);
    }
    Ok(())
}
