use std::ops::Deref;
use std::sync::{Arc, RwLock};
use picturify_core::threading::progress::Progress;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProgressStage {
    Probe,
    Process,
    Merge,
    Finish,
}

pub struct MovieProgress {
    main_progress: Arc<RwLock<Progress>>,
    stage: Arc<RwLock<ProgressStage>>,
}

impl MovieProgress {
    pub fn new() -> Self {
        Self {
            main_progress: Arc::new(RwLock::new(Progress::new())),
            stage: Arc::new(RwLock::new(ProgressStage::Probe)),
        }
    }
    
    pub fn setup(&self, max_value: u32) {
        self.main_progress.write().unwrap().setup(max_value);
    }
    
    pub fn increment(&self) {
        self.main_progress.read().unwrap().increment();
    }
    
    pub fn set_stage(&self, stage: ProgressStage) {
        *self.stage.write().unwrap() = stage;
    }
    
    pub fn get_stage(&self) -> ProgressStage {
        self.stage.read().unwrap().deref().clone()
    }
    
    pub fn get(&self) -> u32 {
        self.main_progress.read().unwrap().get()
    }
    
    pub fn get_max(&self) -> u32 {
        self.main_progress.read().unwrap().get_max()
    }
}