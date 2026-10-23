use std::collections::BTreeMap;

pub fn group_by<T, K, F>(items: &[T], key_fn: F) -> BTreeMap<K, Vec<T>>
where
    T: Clone,
    K: Ord,
    F: Fn(&T) -> K,
{
    let mut groups = BTreeMap::new();
    for item in items {
        groups
            .entry(key_fn(item))
            .or_insert_with(Vec::new)
            .push(item.clone());
    }
    groups
}

pub fn unique<T: Clone + Ord>(items: &[T]) -> Vec<T> {
    let mut copy = items.to_vec();
    copy.sort();
    copy.dedup();
    copy
}

pub fn clamp(value: i64, min: i64, max: i64) -> i64 {
    value.max(min).min(max)
}

pub fn percent(part: usize, total: usize) -> f64 {
    if total == 0 {
        return 0.0;
    }
    ((part as f64 / total as f64) * 1000.0).round() / 10.0
}

pub fn sum(values: &[i64]) -> i64 {
    values.iter().sum()
}

pub fn truncate(value: &str, max_len: usize) -> String {
    if value.chars().count() <= max_len {
        return value.to_string();
    }
    let mut end = 0;
    for (index, (byte, _)) in value.char_indices().enumerate() {
        if index == max_len.saturating_sub(1) {
            end = byte;
            break;
        }
    }
    format!("{}…", &value[..end])
}
