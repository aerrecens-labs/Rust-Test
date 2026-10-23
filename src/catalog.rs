#[derive(Clone)]
pub struct Label {
    pub id: u32,
    pub name: String,
    pub color: String,
    pub description: String,
}

#[derive(Clone)]
pub struct Team {
    pub id: String,
    pub name: String,
    pub mission: String,
    pub capacity: u32,
    pub timezone: String,
}

pub fn labels() -> Vec<Label> {
    [
        ("backend", "#2563eb"),
        ("frontend", "#16a34a"),
        ("infra", "#d97706"),
        ("docs", "#dc2626"),
        ("hiring", "#7c3aed"),
        ("bug", "#0891b2"),
        ("chore", "#64748b"),
        ("review", "#be123c"),
    ]
    .into_iter()
    .enumerate()
    .map(|(index, (name, color))| Label {
        id: (index + 1) as u32,
        name: name.into(),
        color: color.into(),
        description: format!("Work tagged as {name}. Used by boards and weekly planning."),
    })
    .collect()
}

pub fn teams() -> Vec<Team> {
    vec![
        Team {
            id: "platform".into(),
            name: "Platform".into(),
            mission: "Owns shared services and the task API.".into(),
            capacity: 4,
            timezone: "America/New_York".into(),
        },
        Team {
            id: "product".into(),
            name: "Product".into(),
            mission: "Shapes the candidate workflow and interview tools.".into(),
            capacity: 5,
            timezone: "America/Chicago".into(),
        },
        Team {
            id: "recruiting".into(),
            name: "Recruiting".into(),
            mission: "Coordinates screens and take-home reviews.".into(),
            capacity: 6,
            timezone: "Europe/London".into(),
        },
        Team {
            id: "reliability".into(),
            name: "Reliability".into(),
            mission: "Watches health checks and request latency.".into(),
            capacity: 7,
            timezone: "Asia/Kolkata".into(),
        },
    ]
}
