use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

use crate::logger::Logger;
use crate::metrics::Metrics;

pub struct RequestContext {
    pub id: u64,
    pub method: String,
    pub path: String,
}

pub fn build_context(method: &str, path: &str) -> RequestContext {
    static NEXT: AtomicU64 = AtomicU64::new(1);
    RequestContext {
        id: NEXT.fetch_add(1, Ordering::Relaxed),
        method: method.to_string(),
        path: path.to_string(),
    }
}

pub fn record_access(
    logger: &Logger,
    metrics: &mut Metrics,
    context: &RequestContext,
    status_code: u16,
    started: Instant,
) {
    let duration_ms = started.elapsed().as_millis() as u64;
    metrics.observe("http.duration_ms", duration_ms);
    metrics.increment(&format!("http.status.{status_code}"), 1);
    logger.debug(
        "request",
        &format!(
            "{{\"requestId\":{},\"method\":\"{}\",\"path\":\"{}\",\"statusCode\":{status_code},\"durationMs\":{duration_ms}}}",
            context.id, context.method, context.path
        ),
    );
}
