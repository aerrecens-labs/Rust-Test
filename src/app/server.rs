use std::sync::Mutex;

use tiny_http::Server;

use crate::bootstrap::warm_runtime;
use crate::config::load_config;
use crate::logger::Logger;
use crate::metrics::Metrics;

use super::handler::handle;

pub fn start_server() {
    let config = load_config();
    let logger = Logger::new(config.log_level, config.service_name.clone());
    let metrics = Mutex::new(Metrics::new());
    warm_runtime(&config, &logger, &mut metrics.lock().unwrap());
    logger.debug(
        "listening",
        &format!("{{\"url\":\"http://localhost:{}\"}}", config.port),
    );

    let address = format!("127.0.0.1:{}", config.port);
    let server = Server::http(&address).expect("Failed to start server");
    println!("Task API listening on http://localhost:{}", config.port);

    for request in server.incoming_requests() {
        handle(request, &config, &logger, &metrics);
    }
}
