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
    File::open(path).map(BufReader::new).and_then(|reader| {
        serde_json::from_reader(reader).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }).or_else(|_| Ok(Vec::new()))
}

pub fn save_tasks<P: AsRef<Path>>(path: P, tasks: &[Task]) -> io::Result<()> {
    let file = OpenOptions::new().write(true).create(true).truncate(true).open(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, tasks)?;
    Ok(())
}
