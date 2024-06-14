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
    main_progress: Progress,
    stage: Arc<RwLock<ProgressStage>>,
}

impl Default for MovieProgress {
    fn default() -> Self {
        Self::new()
    }
}

impl MovieProgress {
    pub fn new() -> Self {
        Self {
            main_progress: Progress::new(),
            stage: Arc::new(RwLock::new(ProgressStage::Probe)),
        }
    }

    pub fn setup(&self, max_value: usize) {
        self.main_progress.clone().setup(max_value);
    }

    pub fn increment(&self) {
        self.main_progress.increment();
    }

    pub fn set_stage(&self, stage: ProgressStage) {
        *self.stage.write().unwrap() = stage;
    }

    pub fn get_stage(&self) -> ProgressStage {
        self.stage.read().unwrap().deref().clone()
    }

    pub fn get(&self) -> usize {
        self.main_progress.get()
    }

    pub fn get_max(&self) -> usize {
        self.main_progress.get_max()
    }
}
