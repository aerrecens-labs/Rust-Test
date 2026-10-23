use std::sync::{Mutex, OnceLock};

use crate::data::{initial_tasks, Task};
use crate::validation::normalize_status;

fn store() -> &'static Mutex<Vec<Task>> {
    static TASKS: OnceLock<Mutex<Vec<Task>>> = OnceLock::new();
    TASKS.get_or_init(|| Mutex::new(initial_tasks()))
}

/// Candidate task: filter tasks by status ("all" | "active" | "completed").
pub fn filter_tasks(all_tasks: &[Task], status: &str) -> Vec<Task> {
    // TODO: implement filtering for "all" | "active" | "completed"
    let _ = status;
    all_tasks.to_vec()
}

pub fn get_tasks(status: &str) -> Vec<Task> {
    let normalized = normalize_status(status);
    let tasks = store().lock().unwrap();
    filter_tasks(&tasks, normalized)
}

pub fn toggle_task(id: u32) -> Option<Task> {
    let mut tasks = store().lock().unwrap();
    let task = tasks.iter_mut().find(|task| task.id == id)?;
    task.completed = !task.completed;
    Some(task.clone())
}
