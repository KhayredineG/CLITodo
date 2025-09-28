use crate::task::{load_tasks, save_tasks, Task, Priority};
use chrono::prelude::*;
use chrono_english::{parse_date_string, Dialect};
use ratatui::widgets::ListState;

pub enum AppMode {
    Normal,
    Insert,
    DateInput,
    Search,
}

pub struct App {
    pub tasks: Vec<Task>,
    pub state: ListState,
    pub mode: AppMode,
    pub input: String,
    pub date_input: String,
    pub search_input: String,
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
            date_input: String::new(),
            search_input: String::new(),
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
        let i = self.state.selected().map_or(0, |i| (i + 1) % self.tasks.len());
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        if self.tasks.is_empty() {
            return;
        }
        let i = self.state.selected().map_or(0, |i| (i + self.tasks.len() - 1) % self.tasks.len());
        self.state.select(Some(i));
    }

    pub fn toggle_completed(&mut self) {
        if let Some(task) = self.state.selected().and_then(|i| self.tasks.get_mut(i)) {
            task.completed = !task.completed;
        }
    }

    pub fn cycle_priority(&mut self) {
        if let Some(i) = self.state.selected() {
            if let Some(task) = self.tasks.get_mut(i) {
                task.priority = match task.priority {
                    Priority::Low => Priority::Medium,
                    Priority::Medium => Priority::High,
                    Priority::High => Priority::Low,
                };
            }
        }
    }

    pub fn save(&self) {
        save_tasks("tasks.json", &self.tasks).unwrap_or_else(|_| {});
    }

    pub fn add_task(&mut self) {
        let new_id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        let due_date = parse_date_string(&self.input, Local::now(), Dialect::Us)
            .ok()
            .map(|date| date.format("%Y-%m-%d").to_string());
        let tags = self
            .input
            .split_whitespace()
            .filter(|word| word.starts_with('#'))
            .map(|word| word.to_string())
            .collect();

        let new_task = Task {
            id: new_id,
            description: self.input.drain(..).collect(),
            completed: false,
            priority: Priority::Medium,
            due_date,
            sub_tasks: Box::new(Vec::new()),
            tags,
        };
        self.tasks.push(new_task);
        self.mode = AppMode::Normal;
    }

    pub fn add_sub_task(&mut self) {
        if let Some(i) = self.state.selected() {
            if let Some(task) = self.tasks.get_mut(i) {
                let new_id = task.sub_tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
                let due_date = parse_date_string(&self.input, Local::now(), Dialect::Us)
                    .ok()
                    .map(|date| date.format("%Y-%m-%d").to_string());
                let tags = self
                    .input
                    .split_whitespace()
                    .filter(|word| word.starts_with('#'))
                    .map(|word| word.to_string())
                    .collect();

                let new_task = Task {
                    id: new_id,
                    description: self.input.drain(..).collect(),
                    completed: false,
                    priority: Priority::Medium,
                    due_date,
                    sub_tasks: Box::new(Vec::new()),
                    tags,
                };
                task.sub_tasks.push(new_task);
            }
        }
        self.mode = AppMode::Normal;
    }

    pub fn set_due_date(&mut self) {
        if let Some(i) = self.state.selected() {
            if let Some(task) = self.tasks.get_mut(i) {
                task.due_date = Some(self.date_input.drain(..).collect());
            }
        }
        self.mode = AppMode::Normal;
    }

    pub fn delete_task(&mut self) {
        if let Some(i) = self.state.selected() {
            self.tasks.remove(i);
            if !self.tasks.is_empty() {
                self.state.select(Some(i.min(self.tasks.len() - 1)));
            } else {
                self.state.select(None);
            }
        }
    }

    pub fn filter_tasks(&self) -> Vec<Task> {
        self.tasks
            .iter()
            .filter(|task| {
                task.description.contains(&self.search_input)
                    || task.tags.iter().any(|tag| tag.contains(&self.search_input))
            })
            .cloned()
            .collect()
    }
}
