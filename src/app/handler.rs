use std::sync::Mutex;
use std::time::Instant;

use tiny_http::{Method, Request, StatusCode};

use crate::config::Config;
use crate::http::{
    header_value, is_health_path, is_task_list_path, match_task_toggle, not_found, path_only,
    send_json,
};
use crate::logger::Logger;
use crate::metrics::Metrics;
use crate::middleware::{build_context, record_access};
use crate::tasks::{get_tasks, toggle_task};
use crate::validation::read_status_query;

pub fn handle(
    mut request: Request,
    config: &Config,
    logger: &Logger,
    metrics: &Mutex<Metrics>,
) {
    let started = Instant::now();
    let method = request.method().clone();
    let url = request.url().to_string();
    let path = path_only(&url).to_string();
    let context = build_context(method.as_str(), &path);
    let _host = header_value(&request, "host");

    {
        let mut metrics = metrics.lock().unwrap();
        metrics.increment("http.requests", 1);
        metrics.increment(&format!("http.service.{}", config.service_name), 1);
    }

    if method == Method::Options {
        send_json(request, config, StatusCode(204), String::new());
        record_access(logger, &mut metrics.lock().unwrap(), &context, 204, started);
        return;
    }

    if method == Method::Get && is_health_path(&path) {
        send_json(request, config, StatusCode(200), r#"{"ok":true}"#.into());
        record_access(logger, &mut metrics.lock().unwrap(), &context, 200, started);
        return;
    }

    if method == Method::Get && is_task_list_path(&path) {
        let status = read_status_query(&url);
        let tasks = get_tasks(&status);
        send_json(
            request,
            config,
            StatusCode(200),
            serde_json::to_string(&tasks).unwrap(),
        );
        record_access(logger, &mut metrics.lock().unwrap(), &context, 200, started);
        return;
    }

    if method == Method::Patch {
        if let Some(id_str) = match_task_toggle(&path) {
            if let Ok(id) = id_str.parse::<u32>() {
                match toggle_task(id) {
                    Some(task) => {
                        send_json(
                            request,
                            config,
                            StatusCode(200),
                            serde_json::to_string(&task).unwrap(),
                        );
                        record_access(logger, &mut metrics.lock().unwrap(), &context, 200, started);
                    }
                    None => {
                        send_json(
                            request,
                            config,
                            StatusCode(404),
                            r#"{"error":"Task not found"}"#.into(),
                        );
                        record_access(logger, &mut metrics.lock().unwrap(), &context, 404, started);
                    }
                }
                return;
            }
        }
    }

    {
        let mut unused = Vec::new();
        let _ = request.as_reader().read_to_end(&mut unused);
    }
    not_found(request, config);
    record_access(logger, &mut metrics.lock().unwrap(), &context, 404, started);
}
