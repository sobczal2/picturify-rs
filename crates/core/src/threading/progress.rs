use std::sync::atomic::{AtomicU32, Ordering};

pub struct Progress {
    value: AtomicU32,
    max_value: u32,
    on_increment: Option<Box<dyn Fn() + Send + Sync>>,
}

impl Progress {
    pub fn new() -> Self {
        Progress {
            value: AtomicU32::new(0),
            max_value: 0,
            on_increment: None,
        }
    }

    pub fn setup(&mut self, max_value: u32) {
        self.max_value = max_value;
        self.value.store(0, Ordering::SeqCst);
    }

    pub fn increment(&self) {
        self.value.fetch_add(1, Ordering::SeqCst);
        if let Some(on_increment) = &self.on_increment {
            on_increment();
        }
    }

    pub fn get(&self) -> u32 {
        self.value.load(Ordering::SeqCst)
    }

    pub fn get_max(&self) -> u32 {
        self.max_value
    }
    pub fn get_percentage(&self) -> f32 {
        (self.get() as f32 / self.get_max() as f32) * 100.0
    }
    
    pub fn set_on_increment<F: Fn() + Send + Sync + 'static>(&mut self, on_increment: F) {
        self.on_increment = Some(Box::new(on_increment));
    }
}
