use tiny_http::Request;

pub fn query_param(url: &str, name: &str) -> Option<String> {
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

pub fn header_value(request: &Request, name: &str) -> String {
    request
        .headers()
        .iter()
        .find(|header| header.field.as_str().as_str().eq_ignore_ascii_case(name))
        .map(|header| header.value.as_str().to_string())
        .unwrap_or_default()
}

pub fn path_only(url: &str) -> &str {
    url.split('?').next().unwrap_or(url)
}
