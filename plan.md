# Plan: Terminal-based To-Do List

This plan outlines the steps to create a modern, elegant, and interactive terminal-based to-do list application in Rust.

## Phase 1: Core Functionality & Interactivity
- [x] **State Management:** Implement state for the task list to handle selection and scrolling.
- [x] **Task Creation:**
    - [x] Implement an "input mode" to capture user input for new tasks.
    - [x] Add a keybinding (e.g., `n`) to enter input mode.
    - [x] Add the new task to the list.
- [x] **Task Completion:**
    - [x] Add a keybinding (e.g., `d` or `space`) to toggle the `completed` status of the selected task.
- [x] **Task Deletion:**
    - [x] Add a keybinding (e.g., `x`) to delete the selected task.
- [x] **Persistence:**
    - [x] Load tasks from `tasks.json` on application startup.
    - [x] Save all changes to `tasks.json` upon quitting.

## Phase 2: UI/UX Enhancement - The "Modern & Elegant" Look
- [x] **Layout:**
    - [x] Design a two-panel layout: one for the task list and a footer for help text.
    - [x] The help footer should dynamically show available keybindings based on the current mode.
- [x] **Styling:**
    - [x] Style the task list to visually differentiate between pending and completed tasks (e.g., using color and strikethrough text).
    - [x] Add a distinct style for the currently selected task to make it stand out.
    - [x] Implement a clean and visually appealing popup or overlay for the "new task" input mode.
- [x] **Branding:**
    - [x] Add a styled title/header to the application.
    - [x] Choose a modern color palette and apply it consistently across all UI components.

## Phase 3: Advanced Features & Polish
- [x] **Task Editing:**
    - [x] Add a keybinding (e.g., `e`) to enter an "edit mode" for the selected task.
    - [x] Allow the user to modify the task's title.
- [x] **Confirmation Dialogs:**
    - [x] Implement a confirmation prompt before deleting a task to prevent accidental data loss.
- [x] **Error Handling:**
    - [x] Display user-friendly error messages if `tasks.json` is malformed or cannot be read/written.

## Phase 4: Code Refinement & Finalization
- [x] **Refactoring:**
    - [x] Refactor UI components into smaller, reusable functions.
    - [x] Improve the organization of the `app.rs` state and logic.
- [x] **Review & Test:**
    - [x] Conduct a final review of the code for clarity and efficiency.
    - [x] Perform thorough testing of all features and keybindings.
