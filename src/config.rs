#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
    Silent,
}

impl LogLevel {
    pub fn weight(self) -> u8 {
        match self {
            LogLevel::Debug => 10,
            LogLevel::Info => 20,
            LogLevel::Warn => 30,
            LogLevel::Error => 40,
            LogLevel::Silent => 100,
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw.to_ascii_lowercase().as_str() {
            "debug" => Some(LogLevel::Debug),
            "info" => Some(LogLevel::Info),
            "warn" => Some(LogLevel::Warn),
            "error" => Some(LogLevel::Error),
            "silent" => Some(LogLevel::Silent),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct Config {
    pub service_name: String,
    pub port: u16,
    pub log_level: LogLevel,
    pub cors_origin: String,
    pub cors_methods: String,
    pub cors_headers: String,
    pub include_count: bool,
    pub strict_status: bool,
}

pub fn load_config() -> Config {
    Config {
        service_name: env_or("SERVICE_NAME", "task-api"),
        port: read_port(),
        log_level: LogLevel::parse(&env_or("LOG_LEVEL", "info")).unwrap_or(LogLevel::Info),
        cors_origin: "*".into(),
        cors_methods: "GET, PATCH, OPTIONS".into(),
        cors_headers: "Content-Type".into(),
        include_count: false,
        strict_status: false,
    }
}

fn env_or(name: &str, fallback: &str) -> String {
    match std::env::var(name) {
        Ok(value) if !value.is_empty() => value,
        _ => fallback.to_string(),
    }
}

fn read_port() -> u16 {
    std::env::var("PORT")
        .ok()
        .and_then(|raw| raw.parse::<u16>().ok())
        .filter(|port| *port > 0)
        .unwrap_or(3000)
}
