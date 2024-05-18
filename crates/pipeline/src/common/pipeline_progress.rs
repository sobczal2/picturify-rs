use picturify_core::threading::progress::Progress;
use std::sync::{Arc, RwLock};

pub struct PipelineProgress {
    combined_progress: Progress,
    individual_progresses: Vec<(String, Arc<RwLock<Progress>>)>,
    ready: bool
}

impl PipelineProgress {
    pub fn new() -> Self {
        Self {
            combined_progress: Progress::new(),
            individual_progresses: Vec::new(),
            ready: false
        }
    }

    pub fn setup_combined(&mut self, max_value: u32) {
        self.combined_progress.setup(max_value);
        self.ready = true;
    }

    pub fn new_individual(&mut self, name: String) {
        let progress = Arc::new(RwLock::new(Progress::new()));
        self.individual_progresses.push((name, progress));
    }

    pub fn increment_combined(&self) {
        self.combined_progress.increment();
    }

    pub fn get_current_individual_progress(&self) -> Arc<RwLock<Progress>> {
        let current_index = self.combined_progress.get() as usize;
        self.individual_progresses[current_index].1.clone()
    }

    pub fn get_combined_value(&self) -> u32 {
        self.combined_progress.get()
    }

    pub fn get_combined_max(&self) -> u32 {
        self.combined_progress.get_max()
    }

    pub fn get_current_individual_value(&self) -> u32 {
        let progress = self.get_current_individual_progress();
        progress.clone().read().unwrap().get()
    }

    pub fn get_current_individual_max(&self) -> u32 {
        let progress = self.get_current_individual_progress();
        progress.clone().read().unwrap().get_max()
    }

    pub fn get_current_individual_percentage(&self) -> f32 {
        let progress = self.get_current_individual_progress();
        progress.clone().read().unwrap().get_percentage()
    }

    pub fn get_current_individual_name(&self) -> String {
        let current_index = self.combined_progress.get() as usize;
        self.individual_progresses[current_index].0.clone()
    }
    
    pub fn is_ready(&self) -> bool {
        self.ready
    }
    
    pub fn is_finished(&self) -> bool {
        self.get_combined_value() == self.get_combined_max()
    }
}
