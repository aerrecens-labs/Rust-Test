use tiny_http::{Request, Response, StatusCode};

use crate::config::Config;

use super::headers::json_headers;

pub fn send_json(request: Request, config: &Config, status: StatusCode, body: String) {
    let mut response = Response::from_string(body).with_status_code(status);
    for header in json_headers(config) {
        response.add_header(header);
    }
    let _ = request.respond(response);
}

pub fn not_found(request: Request, config: &Config) {
    send_json(
        request,
        config,
        StatusCode(404),
        r#"{"error":"Not found"}"#.into(),
    );
}
