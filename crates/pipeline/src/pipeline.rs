use std::sync::{Arc, RwLock};
use picturify_core::fast_image::FastImage;
use crate::common::pipeline_progress::PipelineProgress;

pub trait Pipeline {
    fn run(&self, fast_image: FastImage, pipeline_progress: Arc<RwLock<PipelineProgress>>) -> FastImage;
}
