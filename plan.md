# Plan: Advanced To-Do List Features

This plan outlines the next phases of development for the terminal-based to-do list, focusing on advanced features and UI/UX enhancements.

## Phase 1: Core Feature Enhancements
- [x] **Task Priorities:**
    - [x] Add a `priority` field to the `Task` struct (e.g., High, Medium, Low).
    - [x] Implement a keybinding (e.g., `p`) to cycle through priority levels for the selected task.
    - [x] Visually indicate task priority in the UI (e.g., with color-coded symbols).
- [x] **Due Dates:**
    - [x] Add an `Option<String>` `due_date` field to the `Task` struct.
    - [x] Create a new "date input" mode, triggered by a keybinding (e.g., `D`).
    - [x] Display the due date next to the task description.
    - [x] Highlight overdue tasks in the UI.
- [x] **Subtasks:**
    - [x] Modify the `Task` struct to include a `Vec<Task>` for subtasks.
    - [x] Implement keybindings to add a subtask to the selected task (e.g., `s`).
    - [x] Create a way to view and interact with subtasks (e.g., by expanding/collapsing them).

## Phase 2: Advanced Features
- [x] **Natural Language Processing (NLP) for Task Creation:**
    - [x] Integrate a lightweight NLP library (e.g., `chrono-english` for dates).
    - [x] When adding a task, parse the input string for due dates (e.g., "buy milk tomorrow").
    - [x] Automatically populate the `due_date` field based on the parsed input.
- [x] **Tags for Organization:**
    - [x] Add a `tags: Vec<String>` field to the `Task` struct.
    - [x] Allow users to add tags to tasks in the input/edit mode (e.g., "buy milk #shopping #urgent").
    - [x] Display tags alongside the task description.
- [ ] **Search/Filter Functionality:**
    - [ ] Implement a "search mode" triggered by a keybinding (e.g., `/`).
    - [ ] Allow users to filter the task list by description, priority, due date, or tags.
    - [ ] Display the current search query in the UI.

## Phase 3: UI/UX and Polish
- [ ] **Themes and Custom Colors:**
    - [ ] Create a `Theme` struct to hold the application's color palette.
    - [ ] Implement a way to switch between different themes (e.g., via a command-line flag or a configuration file).
    - [ ] Allow users to define their own custom themes.
- [ ] **Focus Mode:**
    - [ ] Add a "focus mode" that hides completed tasks and other non-essential UI elements.
    - [ ] Implement a keybinding (e.g., `f`) to toggle focus mode.
- [ ] **Confirmation Dialogs:**
    - [ ] Add a confirmation prompt before performing destructive actions (e.g., deleting all completed tasks).
    - [ ] Create a generic confirmation dialog that can be reused for different actions.