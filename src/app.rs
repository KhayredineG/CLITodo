use crate::task::{load_tasks, save_tasks, Task};
use ratatui::widgets::ListState;

pub enum AppMode {
    Normal,
    Insert,
}

pub struct App {
    pub tasks: Vec<Task>,
    pub state: ListState,
    pub mode: AppMode,
    pub input: String,
    pub margin: u16,
}

impl App {
    pub fn new() -> App {
        let mut state = ListState::default();
        if !load_tasks("tasks.json")
            .unwrap_or_else(|_| Vec::new())
            .is_empty()
        {
            state.select(Some(0));
        }
        App {
            tasks: load_tasks("tasks.json").unwrap_or_else(|_| Vec::new()),
            state,
            mode: AppMode::Normal,
            input: String::new(),
            margin: 1,
        }
    }

    pub fn zoom_in(&mut self) {
        self.margin = self.margin.saturating_sub(1);
    }

    pub fn zoom_out(&mut self) {
        self.margin = self.margin.saturating_add(1);
    }


    pub fn next(&mut self) {
        if self.tasks.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.tasks.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        if self.tasks.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.tasks.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn toggle_completed(&mut self) {
        if let Some(i) = self.state.selected() {
            if let Some(task) = self.tasks.get_mut(i) {
                task.completed = !task.completed;
            }
        }
    }

    pub fn save(&self) {
        save_tasks("tasks.json", &self.tasks).unwrap_or_else(|_| {});
    }

    pub fn add_task(&mut self) {
        let new_id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        let new_task = Task {
            id: new_id,
            description: self.input.drain(..).collect(),
            completed: false,
        };
        self.tasks.push(new_task);
        self.mode = AppMode::Normal;
    }

    pub fn delete_task(&mut self) {
        if let Some(i) = self.state.selected() {
            self.tasks.remove(i);
            if self.tasks.is_empty() {
                self.state.select(None);
            } else if i >= self.tasks.len() {
                self.state.select(Some(self.tasks.len() - 1));
            }
        }
    }
}
