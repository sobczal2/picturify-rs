use std::sync::{Arc, RwLock};
use picturify_core::threading::progress::Progress;

pub struct PipelineProgress {
    combined_progress: Arc<RwLock<Progress>>,
    individual_progresses: Vec<(String, Arc<RwLock<Progress>>)>,
}

impl PipelineProgress {
    pub fn new() -> Self {
        Self {
            combined_progress: Arc::new(RwLock::new(Progress::new())),
            individual_progresses: Vec::new(),
        }
    }

    pub fn setup_combined(&mut self, max_value: u32) {
        self.combined_progress.write().unwrap().setup(max_value);
    }

    pub fn new_individual(&mut self, name: String) {
        let progress = Arc::new(RwLock::new(Progress::new()));
        self.individual_progresses.push((name, progress));
    }

    pub fn increment_combined(&self) {
        self.combined_progress.write().unwrap().increment();
    }

    pub fn get_individual_progress(&self, name: String) -> Arc<RwLock<Progress>> {
        for (progress_name, progress) in &self.individual_progresses {
            if progress_name == &name {
                return progress.clone();
            }
        }
        panic!("Progress with name {} not found", name);
    }
    
    pub fn get_combined_value(&self) -> u32 {
        self.combined_progress.read().unwrap().get()
    }
    
    pub fn get_combined_max(&self) -> u32 {
        self.combined_progress.read().unwrap().get_max()
    }
    
    pub fn get_individual_value(&self, name: String) -> u32 {
        let progress = self.get_individual_progress(name);
        progress.clone().read().unwrap().get()
    }
    
    pub fn get_individual_max(&self, name: String) -> u32 {
        let progress = self.get_individual_progress(name);
        progress.clone().read().unwrap().get_max()
    }
    
    pub fn get_individual_percentage(&self, name: String) -> f32 {
        let value = self.get_individual_value(name.clone()) as f32;
        let max = self.get_individual_max(name.clone()) as f32;
        value / max * 100.0
    }
    
    pub fn get_individual_names(&self) -> Vec<String> {
        self.individual_progresses.iter().map(|(name, _)| name.clone()).collect()
    }
}