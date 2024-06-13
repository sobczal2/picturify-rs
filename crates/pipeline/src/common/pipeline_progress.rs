use picturify_core::threading::progress::Progress;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct PipelineProgress {
    combined_progress: Progress,
    individual_progresses: Arc<RwLock<Vec<(String, Progress)>>>,
    ready: Arc<AtomicBool>,
}

impl Default for PipelineProgress {
    fn default() -> Self {
        Self::new()
    }
}

impl PipelineProgress {
    pub fn new() -> Self {
        Self {
            combined_progress: Progress::new(),
            individual_progresses: Arc::new(RwLock::new(Vec::new())),
            ready: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn setup_combined(&mut self, max_value: usize) {
        self.combined_progress.setup(max_value);
        self.ready.store(true, Ordering::Relaxed);
    }

    pub fn new_individual(&mut self, name: String) {
        let progress = Progress::new();
        self.individual_progresses
            .write()
            .unwrap()
            .push((name, progress));
    }

    pub fn increment_combined(&self) {
        self.combined_progress.increment();
    }

    pub fn get_current_individual_progress(&self) -> Progress {
        let current_index = self.combined_progress.get();
        self.individual_progresses.read().unwrap()[current_index]
            .1
            .clone()
    }

    pub fn get_combined_value(&self) -> usize {
        self.combined_progress.get()
    }

    pub fn get_combined_max(&self) -> usize {
        self.combined_progress.get_max()
    }

    pub fn get_current_individual_value(&self) -> usize {
        let progress = self.get_current_individual_progress();
        progress.get()
    }

    pub fn get_current_individual_max(&self) -> usize {
        let progress = self.get_current_individual_progress();
        progress.get_max()
    }

    pub fn get_current_individual_percentage(&self) -> f32 {
        let progress = self.get_current_individual_progress();
        progress.get_percentage()
    }

    pub fn get_current_individual_name(&self) -> String {
        let current_index = self.combined_progress.get();
        self.individual_progresses.read().unwrap()[current_index]
            .0
            .clone()
    }

    pub fn get_last_individual_name(&self) -> String {
        let last_index = self.individual_progresses.read().unwrap().len() - 1;
        self.individual_progresses.read().unwrap()[last_index]
            .0
            .clone()
    }

    pub fn is_ready(&self) -> bool {
        self.ready.load(Ordering::Relaxed)
    }

    pub fn is_finished(&self) -> bool {
        self.get_combined_value() == self.get_combined_max()
    }
}
