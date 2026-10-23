use crate::data::Task;
use crate::util::truncate;

pub struct StatusFilter {
    pub id: &'static str,
    pub label: &'static str,
    pub description: &'static str,
}

pub fn status_filters() -> &'static [StatusFilter] {
    &[
        StatusFilter {
            id: "all",
            label: "All tasks",
            description: "Every task regardless of completion.",
        },
        StatusFilter {
            id: "active",
            label: "Active",
            description: "Tasks that are not completed.",
        },
        StatusFilter {
            id: "completed",
            label: "Completed",
            description: "Tasks that are marked done.",
        },
    ]
}

pub fn describe_filter(id: &str) -> &'static StatusFilter {
    status_filters()
        .iter()
        .find(|item| item.id == id)
        .unwrap_or(&status_filters()[0])
}

pub struct Priority {
    pub id: &'static str,
    pub label: &'static str,
    pub weight: i32,
    pub sla_hours: u32,
}

pub fn priorities() -> &'static [Priority] {
    &[
        Priority {
            id: "low",
            label: "Low",
            weight: 1,
            sla_hours: 120,
        },
        Priority {
            id: "medium",
            label: "Medium",
            weight: 2,
            sla_hours: 48,
        },
        Priority {
            id: "high",
            label: "High",
            weight: 3,
            sla_hours: 8,
        },
    ]
}

pub fn priority_by_id(id: &str) -> Option<&'static Priority> {
    priorities().iter().find(|item| item.id == id)
}

pub fn compare_priority(left: &str, right: &str) -> std::cmp::Ordering {
    let a = priority_by_id(left).map(|item| item.weight).unwrap_or(0);
    let b = priority_by_id(right).map(|item| item.weight).unwrap_or(0);
    b.cmp(&a)
}

pub struct TaskView {
    pub id: u32,
    pub title: String,
    pub completed: bool,
    pub priority: String,
    pub priority_label: String,
    pub summary: String,
}

pub fn to_task_view(task: &Task) -> TaskView {
    let priority_label = priority_by_id(&task.priority)
        .map(|item| item.label.to_string())
        .unwrap_or_else(|| task.priority.clone());
    TaskView {
        id: task.id,
        title: task.title.clone(),
        completed: task.completed,
        priority: task.priority.clone(),
        priority_label,
        summary: truncate(&task.title, 80),
    }
}

pub fn to_task_views(tasks: &[Task]) -> Vec<TaskView> {
    tasks.iter().map(to_task_view).collect()
}
