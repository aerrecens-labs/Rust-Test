use std::sync::Mutex;

use crate::config::LogLevel;

pub struct Logger {
    level: LogLevel,
    name: String,
    sink: Mutex<()>,
}

impl Logger {
    pub fn new(level: LogLevel, name: impl Into<String>) -> Self {
        Self {
            level,
            name: name.into(),
            sink: Mutex::new(()),
        }
    }

    pub fn debug(&self, message: &str, fields: &str) {
        self.write(LogLevel::Debug, message, fields);
    }

    fn write(&self, entry_level: LogLevel, message: &str, fields: &str) {
        if entry_level.weight() < self.level.weight() {
            return;
        }
        let _guard = self.sink.lock().unwrap();
        if fields.is_empty() {
            println!(
                "{{\"level\":\"{}\",\"name\":\"{}\",\"message\":\"{}\"}}",
                level_name(entry_level),
                self.name,
                message
            );
        } else {
            println!(
                "{{\"level\":\"{}\",\"name\":\"{}\",\"message\":\"{}\",\"fields\":{}}}",
                level_name(entry_level),
                self.name,
                message,
                fields
            );
        }
    }
}

fn level_name(level: LogLevel) -> &'static str {
    match level {
        LogLevel::Debug => "debug",
        LogLevel::Info => "info",
        LogLevel::Warn => "warn",
        LogLevel::Error => "error",
        LogLevel::Silent => "silent",
    }
}
