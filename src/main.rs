mod app;
mod bootstrap;
mod catalog;
mod config;
mod data;
mod domain;
mod http;
mod logger;
mod metrics;
mod middleware;
mod services;
mod store;
mod tasks;
mod util;
mod validation;

fn main() {
    app::start_server();
}
