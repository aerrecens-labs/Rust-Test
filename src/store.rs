#[derive(Clone)]
pub struct MemoryStore<T> {
    records: Vec<T>,
}

impl<T: Clone> MemoryStore<T> {
    pub fn new(initial: Vec<T>) -> Self {
        Self { records: initial }
    }

    pub fn all(&self) -> Vec<T> {
        self.records.clone()
    }

    pub fn find<F>(&self, pred: F) -> Option<T>
    where
        F: Fn(&T) -> bool,
    {
        self.records.iter().find(|item| pred(item)).cloned()
    }

    pub fn insert(&mut self, record: T) -> T {
        self.records.push(record.clone());
        record
    }

    pub fn replace(&mut self, next: Vec<T>) {
        self.records = next;
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }
}
