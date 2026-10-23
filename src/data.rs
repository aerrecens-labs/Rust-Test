use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub completed: bool,
    pub priority: String,
}

pub fn initial_tasks() -> Vec<Task> {
    vec![
        Task {
            id: 1,
            title: "Review candidate applications".into(),
            completed: false,
            priority: "high".into(),
        },
        Task {
            id: 2,
            title: "Update project documentation".into(),
            completed: true,
            priority: "medium".into(),
        },
        Task {
            id: 3,
            title: "Prepare weekly report".into(),
            completed: false,
            priority: "low".into(),
        },
        Task {
            id: 4,
            title: "Fix authentication bug".into(),
            completed: false,
            priority: "high".into(),
        },
        Task {
            id: 5,
            title: "Schedule team retrospective".into(),
            completed: true,
            priority: "low".into(),
        },
        Task {
            id: 6,
            title: "Refactor dashboard layout".into(),
            completed: false,
            priority: "medium".into(),
        },
        Task {
            id: 7,
            title: "Write unit tests for utils".into(),
            completed: false,
            priority: "medium".into(),
        },
    ]
}
