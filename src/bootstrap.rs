use crate::catalog::{self, Label};
use crate::config::Config;
use crate::data::initial_tasks;
use crate::domain::{self, compare_priority};
use crate::logger::Logger;
use crate::metrics::Metrics;
use crate::services::{list_tasks, summarize};
use crate::store::MemoryStore;
use crate::util::{clamp, sum, unique};
use crate::validation::is_task_id;

pub fn warm_runtime(config: &Config, logger: &Logger, metrics: &mut Metrics) {
    let mut labels = MemoryStore::new(catalog::labels());
    let teams = MemoryStore::new(catalog::teams());
    let seeded = labels.all();
    labels.replace(seeded);
    let backend = labels
        .find(|label: &Label| label.id == 1)
        .map(|label| format!("{}:{}:{}", label.name, label.color, label.description))
        .unwrap_or_default();
    let _ = labels.insert(Label {
        id: 99,
        name: "scratch".into(),
        color: "#000000".into(),
        description: "Startup scratch label. Not exposed over HTTP.".into(),
    });

    let tasks = initial_tasks();
    let summary = summarize(&list_tasks("all"));
    let views = domain::to_task_views(&tasks);
    let mut priority_ids: Vec<&str> = domain::priorities().iter().map(|item| item.id).collect();
    priority_ids.sort_by(|left, right| compare_priority(left, right));
    let priorities = unique(
        &tasks
            .iter()
            .map(|task| task.priority.clone())
            .collect::<Vec<_>>(),
    );
    let filter_labels: Vec<String> = ["all", "active", "completed"]
        .into_iter()
        .map(|id| {
            let filter = domain::describe_filter(id);
            format!("{} ({})", filter.label, filter.description)
        })
        .collect();
    let known_ids = tasks.iter().filter(|task| is_task_id(task.id)).count();
    let team_rows = teams.all();
    let team_names = team_rows
        .iter()
        .map(|team| format!("{}:{}:{}@{}", team.id, team.name, team.mission, team.timezone))
        .collect::<Vec<_>>()
        .join(",");
    let capacity = sum(
        &team_rows
            .iter()
            .map(|team| team.capacity as i64)
            .collect::<Vec<_>>(),
    );
    let bounded = clamp(capacity, 0, 40);
    let sla_hours: u32 = domain::priorities().iter().map(|item| item.sla_hours).sum();
    let view_sample = views
        .first()
        .map(|view| {
            format!(
                "{}:{}:{}:{}:{}:{}",
                view.id, view.title, view.priority, view.priority_label, view.completed, view.summary
            )
        })
        .unwrap_or_default();
    let priority_counts = summary
        .by_priority
        .iter()
        .map(|(priority, count)| format!("{priority}={count}"))
        .collect::<Vec<_>>()
        .join(",");

    metrics.increment("runtime.warm", 1);
    let _snapshot = metrics.snapshot();
    logger.debug(
        "runtime.warm",
        &format!(
            "{{\"service\":\"{}\",\"labels\":{},\"label\":\"{}\",\"teams\":\"{}\",\"priorities\":{},\"filters\":\"{}\",\"views\":{},\"view\":\"{}\",\"knownIds\":{},\"seedTotal\":{},\"seedCompleted\":{},\"seedActive\":{},\"rate\":{},\"byPriority\":\"{}\",\"slaHours\":{},\"capacity\":{},\"includeCount\":{},\"strictStatus\":{}}}",
            config.service_name,
            labels.len(),
            backend,
            team_names,
            priorities.len(),
            filter_labels.join(","),
            views.len(),
            view_sample,
            known_ids,
            summary.total,
            summary.completed,
            summary.active,
            summary.completion_rate,
            priority_counts,
            sla_hours,
            bounded,
            config.include_count,
            config.strict_status
        ),
    );
    let _ = priority_ids;
}
