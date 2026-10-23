pub fn match_task_toggle(path: &str) -> Option<&str> {
    let rest = path.strip_prefix("/tasks/")?;
    rest.strip_suffix("/toggle")
}

pub fn is_health_path(path: &str) -> bool {
    path == "/health"
}

pub fn is_task_list_path(path: &str) -> bool {
    path == "/tasks"
}
