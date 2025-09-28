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
    pub adding_subtask: bool,
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
            adding_subtask: false,
        }
    }

    pub fn zoom_in(&mut self) {
        self.margin = self.margin.saturating_sub(1);
    }

    pub fn zoom_out(&mut self) {
        self.margin = self.margin.saturating_add(1);
    }


    pub fn next(&mut self) {
        let displayed_tasks = self.get_displayed_tasks();
        if displayed_tasks.is_empty() {
            return;
        }
        let i = self.state.selected().map_or(0, |i| (i + 1) % displayed_tasks.len());
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let displayed_tasks = self.get_displayed_tasks();
        if displayed_tasks.is_empty() {
            return;
        }
        let i = self.state.selected().map_or(0, |i| (i + displayed_tasks.len() - 1) % displayed_tasks.len());
        self.state.select(Some(i));
    }

    pub fn toggle_completed(&mut self) {
        if let Some(selected_index) = self.state.selected() {
            let displayed_tasks = self.get_displayed_tasks();
            if let Some(selected_task) = displayed_tasks.get(selected_index) {
                // Find the task in the main tasks vector by ID
                if let Some(main_task) = self.tasks.iter_mut().find(|t| t.id == selected_task.id) {
                    main_task.completed = !main_task.completed;
                }
            }
        }
    }

    pub fn cycle_priority(&mut self) {
        if let Some(selected_index) = self.state.selected() {
            let displayed_tasks = self.get_displayed_tasks();
            if let Some(selected_task) = displayed_tasks.get(selected_index) {
                // Find the task in the main tasks vector by ID
                if let Some(main_task) = self.tasks.iter_mut().find(|t| t.id == selected_task.id) {
                    main_task.priority = match main_task.priority {
                        Priority::Low => Priority::Medium,
                        Priority::Medium => Priority::High,
                        Priority::High => Priority::Low,
                    };
                }
            }
        }
    }

    pub fn save(&self) {
        save_tasks("tasks.json", &self.tasks).unwrap_or_else(|_| {});
    }

    pub fn add_task(&mut self) {
        if self.adding_subtask {
            self.add_sub_task();
            self.adding_subtask = false;
        } else {
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
        }
        self.mode = AppMode::Normal;
    }

    pub fn add_sub_task(&mut self) {
        if let Some(selected_index) = self.state.selected() {
            let displayed_tasks = self.get_displayed_tasks();
            if let Some(selected_task) = displayed_tasks.get(selected_index) {
                // Find the task in the main tasks vector by ID
                if let Some(main_task) = self.tasks.iter_mut().find(|t| t.id == selected_task.id) {
                    let new_id = main_task.sub_tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
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
                    main_task.sub_tasks.push(new_task);
                }
            }
        }
        self.mode = AppMode::Normal;
    }

    pub fn set_due_date(&mut self) {
        if let Some(selected_index) = self.state.selected() {
            let displayed_tasks = self.get_displayed_tasks();
            if let Some(selected_task) = displayed_tasks.get(selected_index) {
                // Find the task in the main tasks vector by ID
                if let Some(main_task) = self.tasks.iter_mut().find(|t| t.id == selected_task.id) {
                    main_task.due_date = Some(self.date_input.drain(..).collect());
                }
            }
        }
        self.mode = AppMode::Normal;
    }

    pub fn delete_task(&mut self) {
        if let Some(selected_index) = self.state.selected() {
            let displayed_tasks = self.get_displayed_tasks();
            if let Some(selected_task) = displayed_tasks.get(selected_index) {
                // Find and remove the task in the main tasks vector by ID
                if let Some(main_index) = self.tasks.iter().position(|t| t.id == selected_task.id) {
                    self.tasks.remove(main_index);
                }
                
                // Update selection
                let new_displayed_tasks = self.get_displayed_tasks();
                if !new_displayed_tasks.is_empty() {
                    self.state.select(Some(selected_index.min(new_displayed_tasks.len() - 1)));
                } else {
                    self.state.select(None);
                }
            }
        }
    }

    pub fn filter_tasks(&self) -> Vec<Task> {
        if self.search_input.is_empty() {
            return self.tasks.clone();
        }

        let search_lower = self.search_input.to_lowercase();
        self.tasks
            .iter()
            .filter(|task| {
                // Filter by description (case-insensitive)
                task.description.to_lowercase().contains(&search_lower)
                    // Filter by tags (case-insensitive)
                    || task.tags.iter().any(|tag| tag.to_lowercase().contains(&search_lower))
                    // Filter by priority
                    || match search_lower.as_str() {
                        "high" | "h" => matches!(task.priority, Priority::High),
                        "medium" | "med" | "m" => matches!(task.priority, Priority::Medium),
                        "low" | "l" => matches!(task.priority, Priority::Low),
                        _ => false,
                    }
                    // Filter by completion status
                    || match search_lower.as_str() {
                        "completed" | "done" | "finished" => task.completed,
                        "incomplete" | "pending" | "todo" => !task.completed,
                        _ => false,
                    }
                    // Filter by due date (if it exists)
                    || task.due_date.as_ref().map_or(false, |date| date.contains(&search_lower))
                    // Filter by subtasks content
                    || task.sub_tasks.iter().any(|subtask| {
                        subtask.description.to_lowercase().contains(&search_lower)
                            || subtask.tags.iter().any(|tag| tag.to_lowercase().contains(&search_lower))
                    })
            })
            .cloned()
            .collect()
    }

    pub fn get_displayed_tasks(&self) -> Vec<Task> {
        match self.mode {
            AppMode::Search if !self.search_input.is_empty() => self.filter_tasks(),
            _ => self.tasks.clone(),
        }
    }
}
