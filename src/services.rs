use std::collections::BTreeMap;

use crate::data::Task;
use crate::tasks::get_tasks;
use crate::util::{group_by, percent};

pub struct TaskSummary {
    pub total: usize,
    pub completed: usize,
    pub active: usize,
    pub completion_rate: f64,
    pub by_priority: BTreeMap<String, usize>,
}

pub fn list_tasks(status: &str) -> Vec<Task> {
    get_tasks(status)
}

pub fn summarize(tasks: &[Task]) -> TaskSummary {
    let groups = group_by(tasks, |task| task.priority.clone());
    let completed = tasks.iter().filter(|task| task.completed).count();
    TaskSummary {
        total: tasks.len(),
        completed,
        active: tasks.len() - completed,
        completion_rate: percent(completed, tasks.len()),
        by_priority: groups
            .into_iter()
            .map(|(priority, items)| (priority, items.len()))
            .collect(),
    }
}
