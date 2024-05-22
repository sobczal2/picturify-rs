use std::fmt::Debug;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Clone)]
pub struct Progress {
    value: Arc<AtomicUsize>,
    max_value: Arc<AtomicUsize>,
    on_increment: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl Debug for Progress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Progress")
            .field("value", &self.get())
            .field("max_value", &self.get_max())
            .field("percentage", &self.get_percentage())
            .finish()
    }
}

impl Progress {
    pub fn new() -> Self {
        Progress {
            value: Arc::new(AtomicUsize::new(0)),
            max_value: Arc::new(AtomicUsize::new(0)),
            on_increment: None,
        }
    }

    pub fn setup(&mut self, max_value: usize) {
        self.max_value.store(max_value, Ordering::SeqCst);
        self.value.store(0, Ordering::SeqCst);
    }

    pub fn increment(&self) {
        self.value.fetch_add(1, Ordering::SeqCst);
        if let Some(on_increment) = &self.on_increment {
            on_increment();
        }
    }

    pub fn get(&self) -> usize {
        self.value.load(Ordering::SeqCst)
    }

    pub fn get_max(&self) -> usize {
        self.max_value.load(Ordering::SeqCst)
    }
    pub fn get_percentage(&self) -> f32 {
        (self.get() as f32 / self.get_max() as f32) * 100.0
    }

    pub fn set_on_increment<F: Fn() + Send + Sync + 'static>(&mut self, on_increment: F) {
        self.on_increment = Some(Arc::new(on_increment));
    }
}
