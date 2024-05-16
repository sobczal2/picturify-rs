use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

pub struct Progress {
    value: Arc<AtomicU32>,
    max_value: u32,
}

impl Progress {
    pub fn new() -> Self {
        Progress {
            value: Arc::new(AtomicU32::new(0)),
            max_value: 0,
        }
    }
    
    pub fn setup(&mut self, max_value: u32) {
        self.max_value = max_value;
        self.value.store(0, Ordering::SeqCst);
    }

    pub fn increment(&self) {
        self.value.fetch_add(1, Ordering::SeqCst);
    }

    pub fn get(&self) -> u32 {
        self.value.load(Ordering::SeqCst)
    }

    pub fn get_max(&self) -> u32 {
        self.max_value
    }
}