use std::fmt::Debug;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct Progress {
    inner: Arc<InnerProgress>,
}

struct InnerProgress {
    value: AtomicUsize,
    max_value: AtomicUsize,
    on_increment: RwLock<Option<Box<dyn Fn() + Send + Sync>>>,
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
            inner: Arc::new(InnerProgress {
                value: AtomicUsize::new(0),
                max_value: AtomicUsize::new(0),
                on_increment: RwLock::new(None),
            }),
        }
    }

    pub fn setup(&mut self, max_value: usize) {
        self.inner.max_value.store(max_value, Ordering::SeqCst);
        self.inner.value.store(0, Ordering::SeqCst);
    }

    pub fn increment(&self) {
        self.inner.value.fetch_add(1, Ordering::SeqCst);
        if let Some(on_increment) = self.inner.on_increment.read().unwrap().as_ref() {
            on_increment();
        }
    }

    pub fn get(&self) -> usize {
        self.inner.value.load(Ordering::SeqCst)
    }

    pub fn get_max(&self) -> usize {
        self.inner.max_value.load(Ordering::SeqCst)
    }
    pub fn get_percentage(&self) -> f32 {
        (self.get() as f32 / self.get_max() as f32) * 100.0
    }

    pub fn set_on_increment<F: Fn() + Send + Sync + 'static>(&mut self, on_increment: F) {
        *self.inner.on_increment.write().unwrap() = Some(Box::new(on_increment));
    }
}

pub struct ProgressIterator<I> {
    iter: I,
    progress: Progress,
}

impl<I> Iterator for ProgressIterator<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.progress.increment();
        let next = self.iter.next();
        next
    }
}

pub trait ProgressIteratorExt: Iterator + Sized {
    fn progress(self, progress: Progress) -> ProgressIterator<Self> {
        ProgressIterator {
            iter: self,
            progress,
        }
    }
}

impl<I> ProgressIteratorExt for I where I: Iterator {}
