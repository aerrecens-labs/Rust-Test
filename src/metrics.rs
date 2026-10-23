use std::collections::BTreeMap;

pub struct Metrics {
    counters: BTreeMap<String, u64>,
    observations: BTreeMap<String, Vec<u64>>,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            counters: BTreeMap::new(),
            observations: BTreeMap::new(),
        }
    }

    pub fn increment(&mut self, name: &str, amount: u64) {
        *self.counters.entry(name.to_string()).or_insert(0) += amount;
    }

    pub fn observe(&mut self, name: &str, value: u64) {
        let bucket = self.observations.entry(name.to_string()).or_default();
        bucket.push(value);
        if bucket.len() > 200 {
            bucket.remove(0);
        }
    }

    pub fn snapshot(&self) -> String {
        let mut parts = Vec::new();
        for (name, count) in &self.counters {
            parts.push(format!("{name}={count}"));
        }
        for (name, values) in &self.observations {
            let total: u64 = values.iter().sum();
            let avg = if values.is_empty() {
                0
            } else {
                total / values.len() as u64
            };
            let max = values.iter().copied().max().unwrap_or(0);
            parts.push(format!("{name}:samples={}:avg={avg}:max={max}", values.len()));
        }
        parts.join(",")
    }
}
