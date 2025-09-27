use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: usize,
    pub description: String,
    pub completed: bool,
}

pub fn load_tasks<P: AsRef<Path>>(path: P) -> io::Result<Vec<Task>> {
    if !path.as_ref().exists() {
        return Ok(Vec::new());
    }
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let tasks = serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new());
    Ok(tasks)
}

pub fn save_tasks<P: AsRef<Path>>(path: P, tasks: &[Task]) -> io::Result<()> {
    let file = OpenOptions::new().write(true).create(true).truncate(true).open(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, tasks)?;
    Ok(())
}
