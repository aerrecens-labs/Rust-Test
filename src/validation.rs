use crate::http::query_param;

const KNOWN: [&str; 3] = ["all", "active", "completed"];

pub fn read_status_query(url: &str) -> String {
    query_param(url, "status").unwrap_or_else(|| "all".into())
}

pub fn is_known_status(status: &str) -> bool {
    KNOWN.contains(&status)
}

pub fn normalize_status(status: &str) -> &str {
    if is_known_status(status) {
        status
    } else {
        "all"
    }
}

pub fn is_task_id(value: u32) -> bool {
    value > 0
}
