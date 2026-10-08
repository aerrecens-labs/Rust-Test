use serde::{Deserialize, Serialize};
use std::io::Read;
use std::sync::Mutex;
use tiny_http::{Header, Method, Request, Response, Server, StatusCode};

#[derive(Clone, Serialize, Deserialize)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
    priority: String,
}

struct TaskStore {
    tasks: Vec<Task>,
}

impl TaskStore {
    fn new() -> Self {
        Self {
            tasks: vec![
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
            ],
        }
    }

    /// Candidate task: filter tasks by status ("all" | "active" | "completed").
    fn filter_tasks(&self, all_tasks: &[Task], status: &str) -> Vec<Task> {
        // TODO: implement filtering for "all" | "active" | "completed"
        let _ = status;
        all_tasks.to_vec()
    }

    fn get_tasks(&self, status: &str) -> Vec<Task> {
        let normalized = match status {
            "all" | "active" | "completed" => status,
            _ => "all",
        };
        self.filter_tasks(&self.tasks, normalized)
    }

    fn toggle_task(&mut self, id: u32) -> Option<Task> {
        let task = self.tasks.iter_mut().find(|task| task.id == id)?;
        task.completed = !task.completed;
        Some(task.clone())
    }
}

fn cors_headers() -> Vec<Header> {
    vec![
        Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap(),
        Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap(),
        Header::from_bytes(&b"Access-Control-Allow-Methods"[..], &b"GET, PATCH, OPTIONS"[..])
            .unwrap(),
        Header::from_bytes(&b"Access-Control-Allow-Headers"[..], &b"Content-Type"[..]).unwrap(),
    ]
}

fn send_json(request: Request, status: StatusCode, body: String) {
    let mut response = Response::from_string(body).with_status_code(status);
    for header in cors_headers() {
        response.add_header(header);
    }
    let _ = request.respond(response);
}

fn query_param(url: &str, name: &str) -> Option<String> {
    let query = url.split('?').nth(1)?;
    for part in query.split('&') {
        let mut kv = part.splitn(2, '=');
        let key = kv.next()?;
        let value = kv.next().unwrap_or("");
        if key == name {
            return Some(value.to_string());
        }
    }
    None
}

fn handle(request: Request, store: &Mutex<TaskStore>) {
    let method = request.method().clone();
    let url = request.url().to_string();
    let path = url.split('?').next().unwrap_or(&url);

    if method == Method::Options {
        send_json(request, StatusCode(204), String::new());
        return;
    }

    if method == Method::Get && path == "/health" {
        send_json(request, StatusCode(200), r#"{"ok":true}"#.into());
        return;
    }

    if method == Method::Get && path == "/tasks" {
        let status = query_param(&url, "status").unwrap_or_else(|| "all".into());
        let tasks = store.lock().unwrap().get_tasks(&status);
        send_json(request, StatusCode(200), serde_json::to_string(&tasks).unwrap());
        return;
    }

    if method == Method::Patch {
        if let Some(rest) = path.strip_prefix("/tasks/") {
            if let Some(id_str) = rest.strip_suffix("/toggle") {
                if let Ok(id) = id_str.parse::<u32>() {
                    match store.lock().unwrap().toggle_task(id) {
                        Some(task) => {
                            send_json(request, StatusCode(200), serde_json::to_string(&task).unwrap())
                        }
                        None => send_json(
                            request,
                            StatusCode(404),
                            r#"{"error":"Task not found"}"#.into(),
                        ),
                    }
                    return;
                }
            }
        }
    }

    // Drain body for unused methods/paths to keep tiny_http happy.
    {
        let mut unused = Vec::new();
        let _ = request.as_reader().read_to_end(&mut unused);
    }
    send_json(request, StatusCode(404), r#"{"error":"Not found"}"#.into());
}

fn main() {
    let server = Server::http("127.0.0.1:3000").expect("Failed to start server");
    let store = Mutex::new(TaskStore::new());
    println!("Task API listening on http://localhost:3000");

    for request in server.incoming_requests() {
        handle(request, &store);
    }
}
